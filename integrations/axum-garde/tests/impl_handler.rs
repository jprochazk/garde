use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use axum_garde::WithValidation;
use axum_test::TestServer;
use garde::Validate;
use prost::Message;
use rstest::*;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Deserialize, Validate, Message, Clone)]
struct Person {
    #[prost(string, tag = "1")]
    #[garde(length(min = 1))]
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
struct PathTuple(#[garde(length(min = 1))] pub String);

#[derive(Debug, Error)]
enum CustomRejection {
    #[cfg_attr(feature = "json", error(transparent))]
    #[cfg(feature = "json")]
    JsonRejection(#[from] axum::extract::rejection::JsonRejection),
}

impl IntoResponse for CustomRejection {
    fn into_response(self) -> axum::response::Response {
        todo!("Mock implementation")
    }
}

macro_rules! gen_assert_impl {
    ($test_name:ident,$extractor:ty) => {
        #[rstest]
        #[tokio::test]
        async fn $test_name() {
            #[axum::debug_handler]
            async fn handler(_: WithValidation<$extractor>) {}

            let svc = Router::new().route("/", post(handler)).into_make_service();
            _ = TestServer::new(svc).unwrap();
        }
    };
}

// Axum
#[cfg(feature = "json")]
gen_assert_impl!(assert_json_impl_handler, axum::extract::Json<Person>);
gen_assert_impl!(
    assert_extension_impl_handler,
    axum::extract::Extension<Person>
);
#[cfg(feature = "form")]
gen_assert_impl!(assert_form_impl_handler, axum::extract::Form<Person>);
gen_assert_impl!(assert_path_impl_handler, axum::extract::Path<PathTuple>);
#[cfg(feature = "query")]
gen_assert_impl!(assert_query_impl_handler, axum::extract::Query<Person>);
gen_assert_impl!(assert_state_impl_handler, axum::extract::State<()>);

// Axum extra
#[cfg(feature = "axum-extra-query")]
gen_assert_impl!(
    assert_extra_query_impl_handler,
    axum_extra::extract::Query<Person>
);
#[cfg(all(feature = "axum-extra", feature = "json"))]
gen_assert_impl!(
    assert_withrejection_impl_handler,
    axum_extra::extract::WithRejection<axum::Json<Person>, CustomRejection>
);

#[cfg(feature = "axum-extra-protobuf")]
gen_assert_impl!(
    assert_protobuf_impl_handler,
    axum_extra::protobuf::Protobuf<Person>
);

// Other
#[cfg(feature = "axum-msgpack")]
gen_assert_impl!(assert_mgspack_impl_handler, axum_msgpack::MsgPack<Person>);
#[cfg(feature = "axum-msgpack")]
gen_assert_impl!(
    assert_mgspackraw_impl_handler,
    axum_msgpack::MsgPackRaw<Person>
);
#[cfg(feature = "axum-yaml")]
gen_assert_impl!(assert_yaml_impl_handler, axum_yaml::Yaml<Person>);
