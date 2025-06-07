use crate::spawn_app;

#[actix_web::test]
async fn health_returns_ok() {
    let test_app = spawn_app().await;
    let address = test_app.address.clone();
    let res = reqwest::get(format!("{}/health", address))
        .await
        .expect("Request could not be resolved");

    assert!(res.status().is_success());

}
