use tracing_subscriber::{EnvFilter, Registry, fmt::MakeWriter, layer::SubscriberExt};
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
/// fmtSubscriber i tak. 
/// We need to explicitly call out that the returned subscriber is 
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.
/// 
/// trace > debug > info > warn > error
pub fn get_subscriber<Sink>(name: String, level_filter: String, sink: Sink) -> impl Subscriber + Send + Sync 
        where
        // This "weird" syntax is a higher-ranked trait bound (HRTB)
        // It basically means that Sink implements the `MakeWriter`
        // trait for all choices of the lifetime parameter `'a`
        // Check out https://doc.rust-lang.org/nomicon/hrtb.html
        // for more details.
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
    {
        let formatting_layer = BunyanFormattingLayer::new(name, sink);
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(level_filter));

        Registry::default()
            .with(JsonStorageLayer)
            .with(formatting_layer)
            .with(filter)
    }

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set LogTracer logger");
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
