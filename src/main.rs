use email_newsletter::config::get_configuration;
use email_newsletter::startup::Application;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "email_newsletter".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");
    let application = Application::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}
