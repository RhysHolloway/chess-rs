use std::net::{SocketAddr, TcpListener};
use std::io::Error;

use actix_web::web::Data;
use actix_web::{App, HttpServer, dev::Server};
use settings::Settings;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

mod telemetry;
mod settings;
mod types;

mod routes;

mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    dotenv::from_filename(".env.development").ok();

    let settings = settings::get_settings().expect("Failed to read settings.");

    let subscriber = telemetry::get_subscriber(settings.clone().debug);
    telemetry::init_subscriber(subscriber);

    let application = Application::build(settings, None).await?;

    tracing::event!(target: "backend", tracing::Level::INFO, "Listening on http://127.0.0.1:{}/", application.port());

    application.run_until_stopped().await
}

pub struct Application {
    server: Server,
    port: u16,
}

impl Application {

    pub async fn build(settings: Settings, test_pool: Option<PgPool>) -> Result<Self, Error> {

        let connection_pool = match test_pool {
            Some(pool) => pool,
            None => {
                let db_url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL.");
                match PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&db_url)
                    .await
                {
                    Ok(pool) => pool,
                    Err(e) => {
                        tracing::event!(target: "sqlx",tracing::Level::ERROR, "Couldn't establish DB connection!: {:#?}", e);
                        panic!("Couldn't establish DB connection!")
                    }
                }
            }
        };

        sqlx::migrate!()
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database!");

        let address = SocketAddr::new(settings.application.host, settings.application.port);
        let listener = TcpListener::bind(&address)?;
        
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, connection_pool, settings).await?;

        Ok(Self { server, port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), Error> {
        self.server.await
    }

}

async fn run(listener: TcpListener, db_pool: PgPool, settings: Settings) -> Result<Server, Error> {

    // let data = Data::new(());

    let pool = Data::new(db_pool);

    let redis_url = std::env::var("REDIS_URL").expect("Failed to get REDIS_URL.");

    let cfg = deadpool_redis::Config::from_url(redis_url.clone());
    let redis_pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Cannot create deadpool redis.");
    let redis_pool_data = actix_web::web::Data::new(redis_pool);

    let secret_key = actix_web::cookie::Key::from(settings.secret.hmac_secret.as_bytes());
    let redis_store = actix_session::storage::RedisSessionStore::new(redis_url.clone())
        .await
        .expect("Cannot unwrap redis session.");

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(
                actix_session::SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_http_only(true)
                    .cookie_same_site(actix_web::cookie::SameSite::Lax)
                    .cookie_secure(true)
                    .cookie_name("sessionid".to_string())
                    .build(),
            )
            .service(crate::routes::health_check)
            // Authentication routes
            .configure(crate::routes::auth_routes_config)
            // Add database pool to application state
            .app_data(pool.clone())
            // Add redis pool to application state
            .app_data(redis_pool_data.clone())
            // Logging middleware
            .wrap(actix_web::middleware::Logger::default())
    })
    .listen(listener)?
    .run();

    Ok(server)

}