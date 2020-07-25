mod api;
mod config;
mod db;
mod dberror;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer};
use deadpool_postgres::config as deadpool_config;
use dotenv::dotenv;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use rand::Rng;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut config = crate::config::Config::from_env().unwrap();
    config.pg.ssl_mode = Some(deadpool_config::SslMode::Require);

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());

    let pool = config.pg.create_pool(connector).unwrap();

    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    let _server = HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("authentication")
                    .secure(false),
            ))
            .service(
                web::resource("/login")
                    .wrap(api::auth::CheckLogin)
                    .route(web::post().to(api::users::login)),
            )
            // .wrap(api::auth::CheckLogin)
            .data(pool.clone())
            .service(
                web::resource("/users/{username}")
                    .wrap(api::auth::CheckLogin)
                    .route(web::get().to(api::users::greet)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;
    Ok(())
}
