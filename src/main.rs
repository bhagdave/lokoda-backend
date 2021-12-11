use std::net::TcpListener;
use lokoda_backend::startup::run;
use lokoda_backend::configuration::*;
use sqlx::mysql::MySqlPoolOptions;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    println!("DB String:{}", &configuration.database.connection_string());
    let connection_pool = MySqlPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(200))
        .connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to database");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
