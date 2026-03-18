use axum::http;
use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct RequestIdGenerator;

impl MakeRequestId for RequestIdGenerator {
    fn make_request_id<B>(
        &mut self,
        _request: &http::Request<B>,
    ) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string();
        Some(RequestId::new(id.parse().ok()?))
    }
}

pub fn set_request_id_layer() -> SetRequestIdLayer<RequestIdGenerator> {
    SetRequestIdLayer::x_request_id(RequestIdGenerator)
}

pub fn propagate_request_id_layer() -> PropagateRequestIdLayer {
    PropagateRequestIdLayer::x_request_id()
}
