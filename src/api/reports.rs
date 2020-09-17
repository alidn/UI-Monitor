use crate::db::reports::{Report, ReportInfo};
use crate::db::sessions::{GroupedSession, Session, TagGroup, grouped_sessions_to_session_analysis};
use crate::dberror;
use actix_web::web::Json;
use actix_web::{web, Error, HttpRequest, HttpResponse, post};
use deadpool_postgres::{Client, Pool};

#[post("/reports")]
pub async fn save_report(
    _req: HttpRequest,
    report_info: Json<ReportInfo>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;

    Report::save_report(&client, report_info.into_inner()).await?;

    Ok(HttpResponse::Ok().body(""))
}

pub async fn get_sessions(
    _req: HttpRequest,
    path: web::Path<i32>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;

    let sessions = Session::get_sessions(&client, path.into_inner()).await?;
    let sessions_serialized = serde_json::to_string(&sessions).unwrap();

    Ok(HttpResponse::Ok().body(sessions_serialized))
}

pub async fn get_grouped_sessions(
    _req: HttpRequest,
    path: web::Path<i32>,
    tag_groups: web::Json<Vec<TagGroup>>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;

    let tag_groups = tag_groups.into_inner();

    let grouped_sessions = Session::get_sessions(&client, path.into_inner())
        .await?
        .into_iter()
        .map(|session| session.into_grouped_session(&tag_groups))
        .collect::<Vec<GroupedSession>>();

    Ok(HttpResponse::Ok().body(serde_json::to_string(&grouped_sessions).unwrap()))
}

pub async fn get_sessions_analysis(
    _req: HttpRequest,
    path: web::Path<i32>,
    tag_groups: web::Json<Vec<TagGroup>>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;

    let tag_groups = tag_groups.into_inner();

    let grouped_sessions = Session::get_sessions(&client, path.into_inner())
        .await?
        .into_iter()
        .map(|session| session.into_grouped_session(&tag_groups))
        .collect::<Vec<GroupedSession>>();

    let sessions_analysis = grouped_sessions_to_session_analysis(&grouped_sessions);
    let sessions_analysis_serialized = serde_json::to_string(&sessions_analysis)?;

    Ok(HttpResponse::Ok().body(sessions_analysis_serialized))
}


pub async fn get_percentages(
    _req: HttpRequest,
    path: web::Path<i32>,
    tag_groups: web::Json<Vec<TagGroup>>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(dberror::DataError::PoolError)?;
    let tag_groups = tag_groups.into_inner();

    let percentages = Session::get_percentages(&client, path.into_inner(), &tag_groups).await?;

    Ok(HttpResponse::Ok().body(serde_json::to_string(&percentages).unwrap()))
}
