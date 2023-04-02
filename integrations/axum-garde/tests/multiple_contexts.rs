#![cfg(feature = "json")]

use axum::extract::FromRef;
use axum::http::StatusCode;
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

macro_rules! gen_custom_test {
    ($name:ident, $context_name:ident, $custom_name:ident, $handler_name:ident) => {
        #[derive(Debug, Validate, Serialize, Deserialize)]
        #[garde(context($context_name))]
        struct $name {
            #[garde(custom($custom_name))]
            name: String,
        }

        #[derive(Debug, Clone, Copy, Default)]
        struct $context_name;

        fn $custom_name(_: &str, _: &$context_name) -> garde::Result {
            Ok(())
        }

        async fn $handler_name(_: WithValidation<Json<$name>>) {}
    };
}

gen_custom_test!(Cat, CatContext, custom_cat, cat_handler);
gen_custom_test!(Dog, DogContext, custom_dog, dog_handler);

#[derive(Debug, Clone, Copy, Default, FromRef)]
struct AppState {
    cat_context: CatContext,
    dog_context: DogContext,
}

#[fixture]
fn app_state() -> AppState {
    Default::default()
}

#[fixture]
fn router(app_state: AppState) -> IntoMakeService<Router> {
    Router::new()
        .route("/cat", post(cat_handler))
        .route("/dog", post(dog_handler))
        .with_state(app_state)
        .into_make_service()
}
#[fixture]
fn test_server(router: IntoMakeService<Router>) -> TestServer {
    TestServer::new(router).unwrap()
}

#[rstest]
#[case::cat("/cat", json!({"name": "kitty"}))]
#[case::dog("/dog", json!({"name": "doggy"}))]
#[trace]
#[tokio::test]
async fn assert_that_state_can_contain_multiple_contexts(
    #[case] path: &'static str,
    #[case] payload: serde_json::Value,
    test_server: TestServer,
) {
    // Perform the request
    // Although the type system has verified that multiple contexts can coexist,
    // we perform the request to verify no runtime errors (eg panics) happens
    test_server
        .post(path)
        .json(&payload)
        .await
        .assert_status(StatusCode::OK);
}
