use email_newsletter::config::get_configuration;
use email_newsletter::startup::run;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(config.database.connection_string().expose_secret())
        .expect("Failed to connect to database");

    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = std::net::TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
