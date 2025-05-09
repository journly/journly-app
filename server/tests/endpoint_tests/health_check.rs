use actix_web::{test, App};
use journaly_server::controllers::check_health;

#[actix_web::test]
async fn health_check_returns_ok() {
    let app = test::init_service(App::new().service(check_health)).await;
    let req = test::TestRequest::get()
        .to_request();

    let response = test::call_service(&app, req).await;

    assert!(response.status().is_success());
}
