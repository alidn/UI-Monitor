#![feature(backtrace)]

mod api;
mod config;
mod db;
mod dberror;

use actix_cors::Cors;
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

    // let private_key = rand::thread_rng().gen::<[u8; 32]>();
    // FIXME: Don't forget to use random key (the above line) in prod mode.
    let private_key: [u8; 32] = [0; 32];

    let _server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().supports_credentials().finish())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("user_auth")
                    .secure(false),
            ))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("report_auth")
                    .secure(false),
            ))
            .service(web::resource("/login").route(web::post().to(api::users::login)))
            .data(pool.clone())
            .service(
                web::resource("/projects")
                    .wrap(api::user_auth::CheckLogin)
                    .route(web::get().to(api::projects::get_projects)),
            )
            .service(
                web::resource("/projects/{name}")
                    .wrap(api::user_auth::CheckLogin)
                    .route(web::post().to(api::projects::save_project)),
            )
            .service(web::resource("/reports").route(web::post().to(api::reports::save_report)))
            .service(
                web::resource("/projects/{project_id}/sessions")
                    .route(web::get().to(api::reports::get_sessions)),
            )
            .service(
                web::resource("/projects/{project_id}/grouped")
                    .route(web::get().to(api::reports::get_grouped_sessions)),
            )
            .service(
                web::resource("/projects/{project_id}/percentages")
                    .route(web::get().to(api::reports::get_percentages)),
            )
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await;
    Ok(())
}
