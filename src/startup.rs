use actix_session::SessionMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_flash_messages::storage::CookieMessageStore;
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::config::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::{
    admin_dashboard, confirm, health_check, home, login, login_form, publish_newsletter, subscribe,
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&config.database);

        let sender_email = config
            .email_client
            .sender()
            .expect("Failed to read sender email");

        let timeout = config.email_client.timeout();
        let email_client = EmailClient::new(
            config.email_client.base_url,
            sender_email,
            config.email_client.auth_token,
            timeout,
        );

        let address = format!("{}:{}", config.application.host, config.application.port);

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            config.application.base_url,
            config.application.hmac_secret,
            config.redis_uri,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(config_db: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config_db.with_db())
}

pub struct ApplicationBaseUrl(pub String);

#[derive(Clone)]
pub struct HmacSecret(pub SecretString);

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: SecretString,
    redis_uri: SecretString,
) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/subscriptions/confirm", web::get().to(confirm))
            .route("/newsletters", web::post().to(publish_newsletter))
            .route("/admin/dashboard", web::get().to(admin_dashboard))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
            .app_data(web::Data::new(HmacSecret(hmac_secret.clone())))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
