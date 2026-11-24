use axum::{
    body::Bytes,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

const WHEP_URL: &str = "http://127.0.0.1:8889/stream/whep";

lazy_static::lazy_static! {
    static ref WHEP_CLIENT: reqwest::Client = reqwest::Client::new();
}

/// Forwards the browser's SDP offer to MediaMTX and returns the SDP answer.
pub async fn whep_offer_handler(offer: Bytes) -> Response {
    let response = match WHEP_CLIENT
        .post(WHEP_URL)
        .header(header::CONTENT_TYPE, "application/sdp")
        .body(offer)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => return (StatusCode::BAD_GATEWAY, e.to_string()).into_response(),
    };

    let status = response.status();
    if !status.is_success() {
        return (StatusCode::BAD_GATEWAY, format!("WHEP {status}")).into_response();
    }

    match response.text().await {
        Ok(sdp) => ([(header::CONTENT_TYPE, "application/sdp")], sdp).into_response(),
        Err(e) => (StatusCode::BAD_GATEWAY, e.to_string()).into_response(),
    }
}
