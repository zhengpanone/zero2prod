// use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

// pub mod http_middleware;

// pub fn init_metrics_exporter() -> PrometheusHandle {
// 	// 可选：自定义直方图桶（示例：HTTP 请求时延）
// 	let builder = PrometheusBuilder::new()
// 		.set_buckets_for_metric(
// 			"http_server_request_duration_seconds",
// 			&[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0],
// 		)
// 		.expect("invalid buckets");
// 	builder.install_recorder().expect("install recorder")
// }

// pub fn http_middleware() -> http_middleware::HttpMetricsLayer {
// 	http_middleware::HttpMetricsLayer
// }
