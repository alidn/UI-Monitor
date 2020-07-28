use crate::dberror::DataError;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use deadpool_postgres::Client;
use futures::future::{ready, AndThen, Ready};
use futures::{Future, TryFutureExt, TryStreamExt};
use postgres_types::FromSql;
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;

#[derive(PostgresMapper)]
#[pg_mapper(table = "reports")]
pub struct Report {
    pub report_id: i32,
    pub session_id: uuid::Uuid,
    pub project_id: i32,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, PostgresMapper, Clone)]
#[pg_mapper(table = "users")]
pub struct ReportInfo {
    #[serde(skip_serializing)]
    pub access_key: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub time_ms: i64,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "tags")]
pub struct Tag {
    pub tag_id: i32,
    pub name: String,
}

impl Report {
    pub async fn into_report_info(self, client: &Client) -> Result<ReportInfo, DataError> {
        let tags = self.get_tags(client).await?;

        Ok(ReportInfo {
            access_key: uuid::Uuid::nil(),
            session_id: self.session_id,
            time_ms: self.timestamp,
            tags: tags.into_iter().map(|t| t.name).collect(),
        })
    }

    pub async fn get_reports_of_session(
        client: &Client,
        session_id: uuid::Uuid,
    ) -> Result<Vec<Report>, DataError> {
        let stmt_str = include_str!("../../sql/get_reports_of_session.sql");
        let stmt = client.prepare(&stmt_str).await?;

        Ok(client
            .query(&stmt, &[&session_id])
            .await?
            .iter()
            .map(|row| Report::from_row_ref(row).unwrap())
            .collect::<Vec<Report>>())
    }

    pub async fn get_tags(&self, client: &Client) -> Result<Vec<Tag>, DataError> {
        let stmt_str = include_str!("../../sql/get_tags_of_report.sql");
        let stmt = client.prepare(&stmt_str).await?;

        Ok(client
            .query(&stmt, &[&self.report_id])
            .await?
            .iter()
            .map(|row| Tag::from_row_ref(row).unwrap())
            .collect::<Vec<Tag>>())
    }

    pub async fn save_report(client: &Client, report_info: ReportInfo) -> Result<(), DataError> {
        let stmt_str = include_str!("../../sql/insert_report.sql");
        let stmt = client.prepare(&stmt_str).await?;

        let report_id = client
            .query_one(
                &stmt,
                &[
                    &report_info.access_key,
                    &report_info.session_id,
                    &report_info.time_ms,
                ],
            )
            .await?
            .get("report_id");
        Self::save_tags_to_report(client, report_info.tags, report_id).await?;

        Ok(())
    }

    async fn save_tags_to_report(
        client: &Client,
        tags: Vec<String>,
        report_id: i32,
    ) -> Result<(), DataError> {
        let futures = tags
            .iter()
            .map(|tag_name| Self::save_tag_to_report(client, tag_name, report_id));

        let b = futures::future::try_join_all(futures).await?;
        Ok(())
    }

    async fn save_tag_to_report(
        client: &Client,
        tag_name: &String,
        report_id: i32,
    ) -> Result<(), DataError> {
        let stmt_str = include_str!("../../sql/save_tag_to_report.sql");
        let stmt = client.prepare(&stmt_str).await?;

        let tag = Self::get_or_insert_tag(client, &tag_name).await?;

        client.execute(&stmt, &[&report_id, &tag.tag_id]).await?;

        Ok(())
    }

    async fn get_or_insert_tag(client: &Client, tag_name: &String) -> Result<Tag, DataError> {
        let stmt_str = include_str!("../../sql/get_or_insert_tag.sql");
        let stmt = client.prepare(stmt_str).await?;

        let row = client.query_one(&stmt, &[&tag_name, &tag_name]).await?;
        Ok(Tag::from_row_ref(&row).unwrap())
    }
}
