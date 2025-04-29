use axum::{Router, routing::get};
use metrics_exporter_prometheus::PrometheusHandle;
use crate::state::AppState;

pub fn metrics_routes(handle: PrometheusHandle) -> Router<AppState> {
    Router::new()
        .route("/metrics", get(move || {
            let handle = handle.clone();
            async move {
                handle.render()
            }
        }))
}