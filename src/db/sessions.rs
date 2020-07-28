use crate::db::reports::{Report, ReportInfo};
use crate::dberror::DataError;
use actix_cors::Cors;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use deadpool_postgres::Client;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    fn group_ids(&self, tag_groups: &[TagGroup]) -> Vec<i32> {
        self.reports
            .iter()
            .map(|report_info| {
                tag_groups
                    .iter()
                    .find(|&tag_group| tag_group.contains_any(&report_info.tags))
                    .map_or(0, |tag_group| tag_group.id)
            })
            .collect()
    }

    fn group_by_ids(&self, group_ids: &Vec<i32>) -> Vec<Vec<ReportInfo>> {
        let mut current_id = -1;
        let mut current_group = vec![];
        let mut result = vec![];
        for (idx, id) in group_ids.iter().enumerate() {
            if *id != current_id && !current_group.is_empty() {
                result.push(current_group.clone());
                current_group = vec![];
            }
            current_group.push(self.reports[idx].clone());
            current_id = *id;
        }
        result
    }

    pub fn into_group(self, tag_groups: &[TagGroup]) -> GroupedSession {
        let tag_group_ids = self.group_ids(tag_groups);

        let group_reports = self.group_by_ids(&tag_group_ids);

        let timestamps = group_reports
            .iter()
            .map(|report_group| {
                report_group
                    .last()
                    .map_or(0, |last_report| last_report.time_ms)
            })
            .collect::<Vec<i64>>();

        let mut last_timestamp = 0;
        let mut durations = Vec::<i64>::new();
        for timestamp in timestamps {
            durations.push(timestamp - last_timestamp);
            last_timestamp = timestamp;
        }

        let steps = group_reports[1..]
            .iter()
            .enumerate()
            .map(|(idx, _report_group)| Step {
                step_number: idx,
                tag_group: tag_groups[tag_group_ids[idx + 1] as usize].clone(),
                avg_time_ms: durations[idx + 1],
            })
            .collect();

        GroupedSession { steps }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GroupedSession {
    pub steps: Vec<Step>,
}
