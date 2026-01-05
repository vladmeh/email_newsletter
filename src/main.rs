use email_newsletter::config::get_configuration;
use email_newsletter::startup::run;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database");
    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = std::net::TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
