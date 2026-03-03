use axum::{
    extract::Request,
    response::Response,
};
use std::time::Duration;
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::{MakeSpan, OnFailure, OnRequest, OnResponse, TraceLayer},
};
use tracing::{info, warn, Span};

#[derive(Clone, Debug)]
pub struct CustomMakeSpan;

impl<B> MakeSpan<B> for CustomMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let request_id = uuid::Uuid::new_v4().to_string();
        tracing::info_span!(
            "http_request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
            request_id = %request_id,
        )
    }
}

#[derive(Clone, Debug)]
pub struct CustomOnRequest;

impl<B> OnRequest<B> for CustomOnRequest {
    fn on_request(&mut self, request: &Request<B>, _span: &Span) {
        let method = request.method();
        let uri = request.uri();
        let headers = request.headers();
        
        // Log user agent if present
        let user_agent = headers
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");
        
        // Log content type if present
        let content_type = headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("none");

        info!(
            target: "http_log",
            direction = "IN",
            method = %method,
            uri = %uri,
            user_agent = %user_agent,
            content_type = %content_type,
            "Request received"
        );
    }
}

#[derive(Clone, Debug)]
pub struct CustomOnResponse;

impl<B> OnResponse<B> for CustomOnResponse {
    fn on_response(self, response: &Response<B>, latency: Duration, _span: &Span) {
        let status = response.status();
        let latency_ms = latency.as_millis();
        
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("none");

        if status.is_success() {
            info!(
                target: "http_log",
                direction = "OUT",
                status = %status.as_u16(),
                latency_ms = %latency_ms,
                content_type = %content_type,
                "Response sent"
            );
        } else if status.is_client_error() {
            warn!(
                target: "http_log",
                direction = "OUT",
                status = %status.as_u16(),
                latency_ms = %latency_ms,
                content_type = %content_type,
                "Client error response"
            );
        } else {
            warn!(
                target: "http_log",
                direction = "OUT",
                status = %status.as_u16(),
                latency_ms = %latency_ms,
                content_type = %content_type,
                "Server error response"
            );
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomOnFailure;

impl OnFailure<ServerErrorsFailureClass> for CustomOnFailure {
    fn on_failure(&mut self, failure: ServerErrorsFailureClass, latency: Duration, _span: &Span) {
        let latency_ms = latency.as_millis();
        
        match failure {
            ServerErrorsFailureClass::StatusCode(status) => {
                warn!(
                    target: "http_log",
                    direction = "OUT",
                    status = %status.as_u16(),
                    latency_ms = %latency_ms,
                    "Request failed with status code"
                );
            }
            ServerErrorsFailureClass::Error(error) => {
                warn!(
                    target: "http_log",
                    direction = "OUT",
                    error = %error,
                    latency_ms = %latency_ms,
                    "Request failed with error"
                );
            }
        }
    }
}

pub fn logging_layer() -> TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
    CustomMakeSpan,
    CustomOnRequest,
    CustomOnResponse,
    (),
    (),
    CustomOnFailure,
> {
    TraceLayer::new_for_http()
        .make_span_with(CustomMakeSpan)
        .on_request(CustomOnRequest)
        .on_response(CustomOnResponse)
        .on_body_chunk(())
        .on_eos(())
        .on_failure(CustomOnFailure)
}

