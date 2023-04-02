use std::fmt::Debug;
use std::ops::Deref;

use super::IntoInner;
use super::WithValidationRejection;

use axum::async_trait;
use axum::extract::FromRef;
use axum::extract::FromRequest;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use garde::Unvalidated;
use garde::Valid;
use garde::Validate;

/// An extractor for validating payloads with garde
///
/// `WithValidation` wraps another extractor and validates it's payload. The
/// `T` generic type must be an [`extractor`] that implements `IntoInner`,
///  where `T::Inner: garde::Validate`. The validation context will be extracted
/// from the router's state.
///
/// T is expected to implement [`FromRequest`] or [`FromRequestParts`], and
/// [`IntoInner`]
///
/// The desired validation context ([`garde::Validate::Context`](garde::Validate))
/// must be provided as router state
///
#[cfg_attr(
    feature = "json",
    doc = r#"
### Example

```
use axum::Json;
use serde::{Serialize,Deserialize};
use garde::Validate;
use axum_garde::WithValidation;
#[derive(Debug, Serialize, Deserialize, Validate)]
struct Person {
    #[garde(length(min = 1, max = 10))]
    name: String
}
async fn handler(
    WithValidation(valid_person): WithValidation<Json<Person>>,
) -> String{
    format!("{valid_person:?}")
}
# // Assert that handler compiles
# axum::Router::<_, axum::body::BoxBody>::new()
#   .route("/", axum::routing::post(handler))
#   .with_state(())
#   .into_make_service();
```
"#
)]
/// [`FromRequestParts`]: axum::extract::FromRequestParts
/// [`FromRequest`]: axum::extract::FromRequest
/// [`IntoInner`]: crate::IntoInner
/// [`Valid`]: garde::Valid
/// [`extractor`]: axum::extract
pub struct WithValidation<Extractor>(pub Valid<Extractor::Inner>)
where
    Extractor: IntoInner;

#[async_trait]
impl<State, Extractor, Context> FromRequestParts<State> for WithValidation<Extractor>
where
    State: Send + Sync,
    Extractor: FromRequestParts<State> + IntoInner,
    Extractor::Inner: Validate<Context = Context>,
    Context: FromRef<State>,
{
    type Rejection = WithValidationRejection<Extractor::Rejection>;

    async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        let value = Extractor::from_request_parts(parts, state)
            .await
            .map_err(WithValidationRejection::ExtractionError)?;
        let ctx = FromRef::from_ref(state);
        let value = value.into_inner();
        let value = Unvalidated::new(value)
            .validate(&ctx)
            .map_err(WithValidationRejection::ValidationError)?;
        Ok(WithValidation(value))
    }
}

#[async_trait]
impl<State, Body, Extractor, Context> FromRequest<State, Body> for WithValidation<Extractor>
where
    Body: Send + 'static,
    State: Send + Sync,
    Extractor: FromRequest<State, Body> + IntoInner,
    Extractor::Inner: Validate<Context = Context>,
    Context: FromRef<State>,
{
    type Rejection = WithValidationRejection<Extractor::Rejection>;

    async fn from_request(req: Request<Body>, state: &State) -> Result<Self, Self::Rejection> {
        let value = Extractor::from_request(req, state)
            .await
            .map_err(WithValidationRejection::ExtractionError)?;
        let ctx = FromRef::from_ref(state);
        let value = value.into_inner();
        let value = Unvalidated::new(value)
            .validate(&ctx)
            .map_err(WithValidationRejection::ValidationError)?;
        Ok(WithValidation(value))
    }
}

impl<Extractor> Debug for WithValidation<Extractor>
where
    Extractor: IntoInner + Debug,
    Extractor::Inner: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WithValidation").field(&self.0).finish()
    }
}

impl<Extractor> Clone for WithValidation<Extractor>
where
    Extractor: IntoInner + Clone,
    Extractor::Inner: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Extractor> Copy for WithValidation<Extractor>
where
    Extractor: IntoInner + Copy,
    Extractor::Inner: Copy,
{
}

impl<Extractor> Deref for WithValidation<Extractor>
where
    Extractor: IntoInner,
{
    type Target = Valid<Extractor::Inner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
