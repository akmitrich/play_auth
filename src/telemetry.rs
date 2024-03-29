use tracing_subscriber::layer::SubscriberExt;

pub fn get_subscriber<Sink>(
    name: &str,
    env_filter: &str,
    sink: Sink,
) -> impl tracing::Subscriber + Send + Sync
where
    Sink: for<'a> tracing_subscriber::fmt::MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(env_filter));
    let formatting_layer = tracing_bunyan_formatter::BunyanFormattingLayer::new(name.into(), sink);
    tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl tracing::Subscriber + Send + Sync) {
    tracing_log::LogTracer::init().expect("Failed to init logger");
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber.");
}
