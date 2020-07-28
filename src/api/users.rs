use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};

use crate::db::users::User;
use crate::dberror;

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

pub async fn login(
    login_info: web::Json<LoginInfo>,
    id: Identity,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;

    let login_info = login_info.into_inner();

    let user_id = User::validate_login_info(&client, &login_info).await?;

    id.remember(user_id.to_string());

    Ok(HttpResponse::Ok().body(""))
}

pub async fn greet(
    _req: HttpRequest,
    path: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<User, Error> {
    let username = path.into_inner();

    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;

    let user = User::get_user_by_name(&client, username).await?;

    Ok(user)
}
