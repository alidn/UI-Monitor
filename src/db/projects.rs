use crate::dberror::DataError;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use deadpool_postgres::Client;
use futures::future::{ready, Ready};
use postgres_types::FromSql;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct Project {
    name: String,
    access_key: Uuid,
}

impl Responder for Project {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl Project {
    pub async fn get_projects_of_user(
        client: &Client,
        user_id: i32,
    ) -> Result<Vec<Project>, DataError> {
        let stmt_str = include_str!("../../sql/get_project_of_user.sql");
        let stmt = client.prepare(stmt_str).await?;

        Ok(client
            .query(&stmt, &[&user_id])
            .await?
            .iter()
            .map(|row| Project::from_row_ref(row).unwrap())
            .collect::<Vec<Project>>())
    }

    pub async fn save_project(
        client: &Client,
        user_id: i32,
        name: String,
    ) -> Result<Project, DataError> {
        let stmt_str = include_str!("../../sql/insert_project.sql");
        let stmt = client.prepare(stmt_str).await?;

        let row = client.query_one(&stmt, &[&name, &user_id]).await?;

        let saved_project = Project::from_row_ref(&row).unwrap();

        Ok(saved_project)
    }
}
