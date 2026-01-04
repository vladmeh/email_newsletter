use email_newsletter::config::get_configuration;
use email_newsletter::startup::run;
use sqlx::PgPool;

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("newsletter".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set global default subscriber");

    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = std::net::TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
