use crate::db::projects::Project;
use crate::dberror;
use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn get_projects(
    _req: HttpRequest,
    id: Identity,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;

    let user_id: i32 = id.identity().unwrap().parse().unwrap();

    let projects = Project::get_projects_of_user(&client, user_id).await?;
    let projects_serialized = serde_json::to_string(&projects)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(projects_serialized))
}

pub async fn save_project(
    _req: HttpRequest,
    id: Identity,
    path: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<Project, Error> {
    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;

    let user_id: i32 = id.identity().unwrap().parse().unwrap();

    Ok(Project::save_project(&client, user_id, path.into_inner()).await?)
}
