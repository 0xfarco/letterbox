use secrecy::ExposeSecret;
use sqlx::postgres::PgPool;
use std::net::TcpListener;
use letterbox::configuration::get_configuration;
use letterbox::startup::run;
use letterbox::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subsciber = get_subscriber("letterbox".into(), "info".into(), std::io::stdout);
    init_subscriber(subsciber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
