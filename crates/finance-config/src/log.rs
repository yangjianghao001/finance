use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn base_layer<S>() -> tracing_subscriber::fmt::Layer<
    S,
    tracing_subscriber::fmt::format::DefaultFields,
    tracing_subscriber::fmt::format::Format<
        tracing_subscriber::fmt::format::Full,
        tracing_subscriber::fmt::time::ChronoLocal,
    >,
> {
    tracing_subscriber::fmt::layer()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
            "%F %H:%M:%S%.f".into(),
        ))
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE)
        .with_level(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_target(true)
}
pub fn init() -> Option<WorkerGuard> {
    let registry = tracing_subscriber::registry();
    if cfg!(debug_assertions) {
        let stdout_layer = base_layer().with_writer(std::io::stdout);
        registry.with(stdout_layer).init();
        None
    } else {
        let appender = tracing_appender::rolling::hourly("logs", "finance");
        let (non_blocking_logfile, guard) = tracing_appender::non_blocking(appender);
        let file_layer = base_layer()
            .with_ansi(false)
            .with_writer(non_blocking_logfile);
        registry.with(file_layer).init();
        Some(guard)
    }
}
