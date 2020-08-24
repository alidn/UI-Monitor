use crate::db::projects::Project;
use crate::dberror;
use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use deadpool_postgres::{Client, Pool};
use crate::db::sessions::Session;

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


pub async fn get_project_sessions_count(
    _req: HttpRequest,
    path: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;

    let project_id: i32 =
        Project::get_project_id_from_access_key(&client, path.into_inner().parse().unwrap())
            .await?;

    let count = Session::get_sessions_count(&client, project_id).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&count)?))
}

pub async fn get_project_tags(
    _req: HttpRequest,
    path: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;

    let tag_names = Project::get_tags(&client, path.into_inner().parse().unwrap()).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&tag_names)?))
}

pub async fn get_average_session_duration(
    _req: HttpRequest,
    path: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;

    let project_id: i32 =
        Project::get_project_id_from_access_key(&client, path.into_inner().parse().unwrap())
            .await?;

    let avg_duration = Session::get_average_session_duration(&client, project_id).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&avg_duration.as_secs())?))
}