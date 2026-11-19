#[macro_export]
macro_rules! generate_post_api {
    (
        $name:ident,
        $type:ty
    ) => {
        #[cfg(feature = "frontend")]
        #[doc = concat!("Sends a POST request to the backend API endpoint at `/api/", stringify!($name), "` with the given value serialized as JSON.")]
        pub async fn $name(value: $type) -> std::result::Result<nsh_core::element::HttpResponse, std::string::String> {
            let request = nsh_core::platform::HttpRequest::post(format!("{}{}", $crate::api::API_BASE_URL, stringify!($name)), serde_json::to_vec(&value).unwrap())
                .header("Content-Type", "application/json");
            nsh_core::element::fetch(request).await
        }

        #[cfg(feature = "backend")]
        #[doc = concat!("Registers a backend API endpoint at `/api/", stringify!($name), "`.")]
        #[doc = concat!("Send data to this endpoint as a POST request with the value serialized as JSON.")]
        pub fn $name<S, F, Fut>(handler: F, state: S) -> axum::Router
        where
            F: Fn(S, $type) -> Fut + Clone + Send + Sync + 'static,
            S: Clone + Send + Sync + 'static,
            Fut: std::future::Future<Output = axum::http::StatusCode> + Send + 'static,
        {
            let route = axum::routing::post(move |axum::Json(value): axum::Json<$type>| handler(state, value) );
            axum::Router::new().route(concat!("/api/", stringify!($name)), route)
        }
    };
}
