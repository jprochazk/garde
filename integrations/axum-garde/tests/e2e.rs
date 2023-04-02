#![cfg(feature = "json")]

use axum::debug_handler;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::routing::IntoMakeService;
use axum::Json;
use axum::Router;
use axum_garde::WithValidation;
use axum_test::TestServer;
use garde::Validate;
use rstest::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use speculoos::assert_that;

#[derive(Debug, Serialize, Deserialize, Validate, PartialEq, Eq)]
struct Person {
    #[garde(length(min = 1, max = 10))]
    name: String,
}

#[debug_handler]
async fn valid_echo_person_handler(
    WithValidation(valid_person): WithValidation<Json<Person>>,
) -> impl IntoResponse {
    Json(valid_person.into_inner())
}

#[debug_handler]
async fn echo_person_handler(json: Json<Person>) -> impl IntoResponse {
    json
}

type AppState = ();

#[fixture]
fn app_state() -> AppState {
    Default::default()
}

#[fixture]
fn router(app_state: AppState) -> IntoMakeService<Router> {
    Router::new()
        .route("/echo_person", post(echo_person_handler))
        .route("/valid_echo_person", post(valid_echo_person_handler))
        .with_state(app_state)
        .into_make_service()
}

#[fixture]
fn test_server(router: IntoMakeService<Router>) -> TestServer {
    TestServer::new(router).unwrap()
}

#[rstest]
#[case::hello(Person { name: "hello".into() })]
#[case::world(Person { name: "World".into() })]
#[tokio::test]
async fn assert_that_valid_requests_are_transparent(
    test_server: TestServer,
    #[case] payload: Person,
) {
    let expected = test_server.post("/echo_person").json(&payload).await;
    let obtained = test_server.post("valid_echo_person").json(&payload).await;

    assert_that!(obtained.headers()).is_equal_to(expected.headers());

    obtained
        .assert_status(expected.status_code())
        .assert_json(&expected.json::<Person>());
}

#[rstest]
#[case::not_a_person(StatusCode::UNPROCESSABLE_ENTITY, json!({"not": "a person"}))]
#[case::long_name(StatusCode::UNPROCESSABLE_ENTITY, json!({"name": "too long to be valid......."}))]
#[case::short_name(StatusCode::UNPROCESSABLE_ENTITY, json!({"name": ""}))]
#[tokio::test]
async fn assert_that_invalid_requests_are_sucessfully_rejected(
    test_server: TestServer,
    #[case] status_code: StatusCode,
    #[case] payload: Value,
) {
    let response = test_server
        .post("/valid_echo_person")
        .json(&payload)
        .expect_failure()
        .await;
    response.assert_status(status_code);
}
