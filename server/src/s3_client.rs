use std::{path::Path, sync::Arc};

use actix_multipart::form::tempfile::TempFile;
use aws_config::SdkConfig;
use aws_sdk_s3::{self as s3, primitives::ByteStream};
use s3::Client;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use crate::config::S3Config;

pub fn get_file_extension(file: &TempFile) -> String {
    Path::new(&file.file_name.as_deref().unwrap())
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
        .unwrap()
}

#[derive(Debug, Clone)]
pub struct S3Client {
    pub s3: Arc<Client>,
    pub bucket_name: String,
    pub base_url: String,
}

impl S3Client {
    pub fn new(config: &SdkConfig, bucket_name: &str, base_url: &str) -> Self {
        let client = s3::Client::new(config);

        S3Client {
            s3: Arc::new(client),
            bucket_name: bucket_name.to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub fn url(&self, key: &str) -> String {
        format!("{}/{}", self.base_url, key)
    }

    pub async fn build_config(s3_config: &S3Config) -> SdkConfig {
        aws_config::from_env()
            .endpoint_url(format!(
                "https://{}.r2.cloudflarestorage.com",
                s3_config.account_id
            ))
            .credentials_provider(aws_sdk_s3::config::Credentials::new(
                &s3_config.access_key_id,
                &s3_config.access_key_secret,
                None,
                None,
                "R2",
            ))
            .region("auto")
            .load()
            .await
    }

    pub async fn upload(&self, file: &TempFile, key_prefix: &str, content_type: &str) -> String {
        let file_path = file.file.path().to_str().unwrap();
        let ext = get_file_extension(file);

        let id = Uuid::new_v4();

        let key = format!("{key_prefix}{id}.{ext}");
        let s3_url = self
            .put_object_from_file(file_path, &key, content_type)
            .await;

        tokio::fs::remove_file(file_path).await.unwrap();

        s3_url.to_string()
    }

    async fn put_object_from_file(
        &self,
        local_path: &str,
        key: &str,
        content_type: &str,
    ) -> String {
        let mut file = tokio::fs::File::open(local_path).await.unwrap();

        let size_estimate = file
            .metadata()
            .await
            .map(|md| md.len())
            .unwrap_or(1024)
            .try_into()
            .expect("file too big");

        let mut contents = Vec::with_capacity(size_estimate);
        file.read_to_end(&mut contents).await.unwrap();

        let _res = self
            .s3
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .content_type(content_type)
            .body(ByteStream::from(contents))
            .send()
            .await
            .expect("Failed to put object");

        self.url(key)
    }

    pub async fn delete_file(&self, key: &str) -> bool {
        self.s3
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .is_ok()
    }

    pub fn get_key_from_url(&self, url: &str) -> String {
        url.replace(&format!("{}/", self.base_url), "")
    }
}
