use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::{trace, Resource};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_tracing(
	service_name: &str,
	jaeger_endpoint: Option<String>,
) -> anyhow::Result<()> {
	let env_filter = EnvFilter::try_from_default_env()
		.unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug,sqlx=warn"));

	let fmt_layer = tracing_subscriber::fmt::layer().with_target(false).json();

	let registry = tracing_subscriber::registry()
		.with(env_filter)
		.with(fmt_layer);

	// 如果配置了 Jaeger，添加追踪层
	if let Some(endpoint) = jaeger_endpoint {
		let tracer = opentelemetry_jaeger::new_agent_pipeline()
			.with_endpoint(endpoint)
			.with_service_name(service_name)
			.with_trace_config(trace::config().with_resource(Resource::new(vec![
				KeyValue::new("service.name", service_name.to_string()),
			])))
			.install_batch(opentelemetry_sdk::runtime::Tokio)?;

		let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
		registry.with(telemetry_layer).try_init()?;
	} else {
		registry.try_init()?;
	}

	Ok(())
}

pub fn shutdown_tracing() {
	global::shutdown_tracer_provider();
}
