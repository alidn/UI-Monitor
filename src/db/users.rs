use deadpool_postgres::Client;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::api::users::LoginInfo;
use crate::dberror::DataError;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use openssl::hash::hash_xof;

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub username: String,
    pub email: String,

    #[serde(skip_serializing)]
    password: String,
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application.json")
            .body(body)))
    }
}

impl User {
    pub async fn get_user_by_name(client: &Client, username: String) -> Result<User, DataError> {
        let stmt_str = include_str!("../../sql/get_user_by_username.sql");
        let stmt = client.prepare(stmt_str).await.unwrap();

        client
            .query(&stmt, &[&username])
            .await?
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop()
            .ok_or(DataError::NotFound)
    }

    pub async fn get_user_by_email(client: &Client, email: &String) -> Result<User, DataError> {
        let stmt_str = include_str!("../../sql/get_user_by_email.sql");
        let stmt = client.prepare(stmt_str).await?;

        client
            .query(&stmt, &[&email])
            .await?
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop()
            .ok_or(DataError::EmailNotFound)
    }

    pub async fn validate_login_info(
        client: &Client,
        login_info: &LoginInfo,
    ) -> Result<(), DataError> {
        let user = Self::get_user_by_email(client, &login_info.email).await?;
        if hash_pass(&login_info.password) != user.password {
            Err(DataError::WrongPassword)
        } else {
            Ok(())
        }
    }
}

fn hash_pass(pass: &String) -> String {
    pass.to_owned()
}
