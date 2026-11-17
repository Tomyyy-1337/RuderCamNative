#[macro_export]
macro_rules! generate_api {
    (
        $name:ident,
        $type:ty
    ) => {
        #[cfg(feature = "frontend")]
        use nsh_core::prelude::*;

        #[cfg(feature = "frontend")]
        pub fn $name(value: $type) {
            spawn(async move {
                let request = HttpRequest::post(concat!("http://127.0.0.1:3000/api/", stringify!($name)), serde_json::to_vec(&value).unwrap())
                    .header("Content-Type", "application/json");
                let _ = fetch(request).await;
            });
        }

        #[cfg(feature = "backend")]
        use std::future::Future;
        #[cfg(feature = "backend")]
        use axum::{
            extract::State,
            http::StatusCode,
            routing::post,
            Json, Router,
        };

        #[cfg(feature = "backend")]
        pub fn $name<S, F, Fut>(handler: F) -> Router<S>
        where
            F: Fn(S, $type) -> Fut + Clone + Send + Sync + 'static,
            S: Clone + Send + Sync + 'static,
            Fut: Future<Output = StatusCode> + Send + 'static,
        {
            let route = post(move |State(state): State<S>, Json(value): Json<$type>| handler(state, value) );
            Router::new().route(concat!("/api/", stringify!($name)), route)
        }
    };
}