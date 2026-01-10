use email_newsletter::config::get_configuration;
use email_newsletter::email_client::EmailClient;
use email_newsletter::startup::run;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.database.with_db());

    let sender_email = config
        .email_client
        .sender()
        .expect("Failed to read sender email");
    let email_client = EmailClient::new(config.email_client.base_url, sender_email);

    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = std::net::TcpListener::bind(address)?;
    run(listener, connection_pool, email_client)?.await
}
