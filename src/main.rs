use email_newsletter::config::get_configuration;
use email_newsletter::startup::run;
use env_logger::Env;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = std::net::TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
