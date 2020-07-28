use crate::db::reports::{Report, ReportInfo};
use crate::dberror::DataError;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use deadpool_postgres::Client;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

pub type ProjectStats = Vec<Step>;

#[derive(Serialize, Deserialize)]
pub struct Step {
    pub step_number: usize,
    pub tag_group: TagGroup,
    pub avg_time_ms: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TagGroup {
    pub id: i32,
    pub tags_names: Vec<String>,
}

impl TagGroup {
    pub fn contains_any(&self, tags: &[String]) -> bool {
        for self_tag in &self.tags_names {
            for other_tag in tags {
                if self_tag == other_tag {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub session_id: uuid::Uuid,
    pub reports: Vec<ReportInfo>,
}

impl Responder for Session {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let serialized_session = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok().body(serialized_session)))
    }
}

impl Session {
    pub async fn get_sessions(client: &Client, project_id: i32) -> Result<Vec<Session>, DataError> {
        let session_ids = Self::get_session_ids(client, project_id).await?;
        let sessions = futures::future::join_all(
            session_ids
                .into_iter()
                .map(|session_id| Self::get_session(client, session_id)),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<Session>, DataError>>()?;

        Ok(sessions)
    }

    pub async fn get_session(
        client: &Client,
        session_id: uuid::Uuid,
    ) -> Result<Session, DataError> {
        let reports_futures = Report::get_reports_of_session(client, session_id)
            .await?
            .into_iter()
            .map(|report| report.into_report_info(client));
        let reports = futures::future::join_all(reports_futures)
            .await
            .into_iter()
            .collect::<Result<Vec<ReportInfo>, DataError>>()?;

        Ok(Session {
            session_id,
            reports,
        })
    }

    pub async fn get_session_ids(
        client: &Client,
        project_id: i32,
    ) -> Result<Vec<uuid::Uuid>, DataError> {
        let stmt_str = include_str!("../../sql/get_session_ids.sql");
        let stmt = client.prepare(&stmt_str).await?;

        Ok(client
            .query(&stmt, &[&project_id])
            .await?
            .iter()
            .map(|row| row.get::<_, uuid::Uuid>("session_id"))
            .collect())
    }

    pub fn into_group(self, tag_groups: &[TagGroup]) -> GroupedSession {
        let tag_group_ids = self
            .reports
            .iter()
            .map(|report_info| {
                tag_groups
                    .iter()
                    .find(|&tag_group| tag_group.contains_any(&report_info.tags))
                    .map_or(0, |tag_group| tag_group.id)
            })
            .collect::<Vec<i32>>();

        let last_group_id = -1;
        let mut group_reports: Vec<Vec<ReportInfo>> = Vec::new();
        let mut current_group: Vec<ReportInfo> = Vec::new();
        for (idx, report_info) in self.reports.iter().enumerate() {
            if tag_group_ids[idx] != last_group_id {
                group_reports.push(current_group.clone());
                current_group = Vec::new();
            } else {
                current_group.push(report_info.clone())
            }
        }
        // if !current_group.is_empty() {
        //     group_reports.push(current_group)
        // }

        let steps = group_reports
            .iter()
            .enumerate()
            .map(|(idx, report_group)| {
                let last_time = report_group.last().map_or(0, |report| report.time_ms);
                let first_time = report_group.first().map_or(0, |report| report.time_ms);
                Step {
                    step_number: idx,
                    tag_group: tag_groups[idx].clone(),
                    avg_time_ms: last_time - first_time,
                }
            })
            .collect();

        GroupedSession { steps }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GroupedSession {
    pub steps: Vec<Step>,
}
