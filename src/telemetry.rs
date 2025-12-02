use std::str::FromStr;

use tracing_subscriber::{EnvFilter, Registry, filter::{LevelFilter}, layer::SubscriberExt};
use tracing_log::LogTracer;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to 
/// spell out the actual type of the returned subscriber, which is 
/// indeed quite complex. jer moze biti Registry ili Layered ili
/// fmtSubscriber i tak
/// We need to explicitly call out that the returned subscriber is 
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.
pub fn get_subscriber(name: String, level_filter: String) -> impl Subscriber + Send + Sync {
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    let filter = EnvFilter::from_default_env()
        .add_directive(LevelFilter::from_str(&level_filter).unwrap().into());

    Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(filter)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set LogTracer logger");
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
