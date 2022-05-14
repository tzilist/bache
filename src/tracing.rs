use std::time::Duration;

use clap::Parser;
use eyre::WrapErr;
use opentelemetry::{
    global,
    sdk::{
        trace::{self, IdGenerator, Sampler, Tracer},
        Resource,
    },
    KeyValue,
};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use tracing::{subscriber::set_global_default, Level};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[derive(Parser, Debug, Clone)]
#[clap(rename_all = "kebab-case", next_help_heading = "LOGGING CONFIGS")]
pub struct LoggingConfig {
    /// Disables all logging
    #[clap(long = "log_disabled", env = "BACHE_LOG_DISABLED")]
    pub disabled: bool,

    /// Log level. Must be one of trace, debug, info, warn, error
    #[clap(short, long = "log_level", env = "BACHE_LOG_LEVEL", default_value_t = Level::INFO)]
    pub level: Level,

    /// Enables excessively pretty, multi-line logs, optimized for human readability.
    #[clap(long = "log_fmt_pretty_print", env = "BACHE_LOG_FMT_PRETTY")]
    pub pretty: bool,

    /// Enables human-readable, single-line logs for each event that occurs,
    /// with the current span context displayed before the formatted representation of the event.
    #[clap(long = "log_fmt_full", env = "BACHE_LOG_FMT_FULL")]
    pub full: bool,

    /// Enables a less verbose log formatter
    #[clap(long = "log_fmt_json", env = "BACHE_LOG_FMT_JSON")]
    pub json: bool,

    /// Exclude thread names in the logging output
    #[clap(
        long = "log_exclude_thread_names",
        env = "BACHE_LOG_EXCLUDE_THREAD_NAMES"
    )]
    pub exclude_thread_names: bool,

    /// Exclude thread names in the logging output
    #[clap(long = "log_no_color", env = "NO_COLOR")]
    pub no_color: bool,
}

#[derive(Parser, Debug, Clone)]
#[clap(rename_all = "kebab-case", next_help_heading = "OPENTELEMETRY OPTIONS")]
pub struct OpenTelemetryConfig {
    /// Enable OpenTelemetry tracing
    #[clap(long = "telemetry_enable_otpl", env = "BACHE_TELEMETRY_ENABLE_OTPL")]
    pub enable: bool,

    /// OpenTelemetry endpoint
    #[clap(
        long = "telemetry_otlp_endpoint",
        env = "BACHE_TELEMETRY_OTLP_ENDPOINT",
        default_value = "http://localhost:55680"
    )]
    pub endpoint: String,

    /// Timeout for sending OpenTelemetry traces
    #[clap(
        long = "telemetry_tracing_timeout",
        env = "BACHE_TELEMETRY_OTLP_TIMEOUT",
        default_value_t = 3
    )]
    pub timeout: u64,
}

#[derive(Parser, Debug, Clone)]
pub struct TracingConfig {
    #[clap(flatten)]
    pub logging_config: LoggingConfig,

    #[clap(flatten)]
    pub open_telemetry_config: OpenTelemetryConfig,
}

fn create_opentelemetry_layer(otel_config: &OpenTelemetryConfig) -> eyre::Result<Tracer> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(&otel_config.endpoint)
                .with_timeout(Duration::from_secs(otel_config.timeout))
                .with_protocol(Protocol::Grpc),
        )
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(IdGenerator::default())
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_resource(Resource::new(vec![KeyValue::new("service.name", "bache")])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .wrap_err("Failed to create OpenTelemetry tracing layer")
}

/// Unit struct, used to shutdown the global tracing-subscriber when it is dropped
pub struct ShutdownGuard;

impl Drop for ShutdownGuard {
    fn drop(&mut self) {
        global::shutdown_tracer_provider();
    }
}

pub fn init(config: &TracingConfig) -> eyre::Result<ShutdownGuard> {
    // nice errors for humans
    if !config.logging_config.no_color {
        color_eyre::install()?;
    } else {
        stable_eyre::install()?;
    }

    let opentelemetry_layer = if config.open_telemetry_config.enable {
        let otlp_layer = create_opentelemetry_layer(&config.open_telemetry_config)
            .map(|optl| tracing_opentelemetry::layer().with_tracer(optl))?;

        Some(otlp_layer)
    } else {
        None
    };

    let subscriber = Registry::default()
        .with(EnvFilter::new(config.logging_config.level.to_string()))
        .with(opentelemetry_layer);

    // add logging layer if not disabled
    if !config.logging_config.disabled {
        let logging_layer = tracing_subscriber::fmt::layer()
            .with_thread_names(!config.logging_config.exclude_thread_names)
            .with_ansi(!config.logging_config.no_color);

        let tracing_init_results = if config.logging_config.pretty {
            set_global_default(subscriber.with(logging_layer.pretty()))
        } else if config.logging_config.json {
            set_global_default(
                subscriber.with(
                    logging_layer
                        .json()
                        .flatten_event(true)
                        .with_current_span(true)
                        .with_span_list(true),
                ),
            )
        } else if config.logging_config.full {
            set_global_default(subscriber.with(logging_layer))
        } else {
            set_global_default(subscriber.with(logging_layer.compact()))
        };

        tracing_init_results.wrap_err("Failed to initialize global tracing subscriber")?;

        tracing::debug!("Logging successfully initialized");
    }

    Ok(ShutdownGuard)
}
