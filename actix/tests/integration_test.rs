use actix_web::{test, App, http::StatusCode};

#[actix_rt::test]
async fn test_index() {
    // Create the test server
    let app = App::new().service(web::resource("/").to(crate::index));
    let mut app = test::init_service(app).await;

    // Perform a GET request to the / endpoint
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response is successful (200 OK)
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert that the response body is as expected
    let body = test::read_body(resp).await;
    assert_eq!(body, "Hello, Actix!");
}