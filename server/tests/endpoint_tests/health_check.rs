use crate::spawn_app;

#[actix_web::test]
async fn health_check_returns_ok() {
    let address = spawn_app().await;

    let res = reqwest::get(format!("{}/health", address))
        .await
        .expect("Request could not be resolved");

    assert!(res.status().is_success());
}
