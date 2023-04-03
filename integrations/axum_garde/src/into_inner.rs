/// Trait for unwrapping extractor's payloads
///
/// Types that extract data from request should implement this trait, as it
/// unlocks extractor composition with this library
pub trait IntoInner {
    /// Wrapped payload type
    type Inner;
    /// Consume the extractor and unwrap the payload
    fn into_inner(self) -> Self::Inner;
}

macro_rules! impl_into_inner_simple {
    (
        $name:ty,
        [$($type_var:ident),* $(,)?]
    ) => {
        impl<$($type_var),*> IntoInner for $name {
            type Inner = T;
            fn into_inner(self) -> Self::Inner {
                self.0
            }
        }
    };
}
#[allow(unused_macros)]
macro_rules! impl_into_inner_wrapper {
    (
        $name:ty,
        $inner_type_var:ident,
        [$($type_var:ident),* $(,)?]
    ) => {
        impl<$inner_type_var, $($type_var),*> IntoInner for $name
        where
            $inner_type_var: IntoInner,
        {
            type Inner = <$inner_type_var as IntoInner>::Inner;
            fn into_inner(self) -> Self::Inner {
                self.0.into_inner()
            }
        }
    };
}

// Axum
#[cfg(feature = "json")]
impl_into_inner_simple!(axum::extract::Json<T>, [T]);
impl_into_inner_simple!(axum::extract::Extension<T>, [T]);
#[cfg(feature = "form")]
impl_into_inner_simple!(axum::extract::Form<T>, [T]);
impl_into_inner_simple!(axum::extract::Path<T>, [T]);
#[cfg(feature = "query")]
impl_into_inner_simple!(axum::extract::Query<T>, [T]);
impl_into_inner_simple!(axum::extract::State<T>, [T]);

// Axum extra
#[cfg(feature = "axum-extra")]
impl_into_inner_wrapper!(axum_extra::extract::WithRejection<E, T>, E, [T]);
#[cfg(feature = "axum-extra")]
impl_into_inner_wrapper!(axum_extra::extract::Cached<T>, T, []);
#[cfg(feature = "axum-extra-protobuf")]
impl_into_inner_simple!(axum_extra::protobuf::Protobuf<T>, [T]);
#[cfg(feature = "axum-extra-query")]
impl_into_inner_simple!(axum_extra::extract::Query<T>, [T]);

// Other
#[cfg(feature = "axum-yaml")]
impl_into_inner_simple!(axum_yaml::Yaml<T>, [T]);
#[cfg(feature = "axum-msgpack")]
impl_into_inner_simple!(axum_msgpack::MsgPack<T>, [T]);
#[cfg(feature = "axum-msgpack")]
impl_into_inner_simple!(axum_msgpack::MsgPackRaw<T>, [T]);
