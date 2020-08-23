use crate::db::reports::{Report, ReportInfo};
use crate::dberror::DataError;
use actix_cors::Cors;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use deadpool_postgres::Client;
use futures::future::{ready, Ready};
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::db::percentage::Percentage;

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

    /// returns the sessions that have at least one of the tags in tag group.
    pub async fn get_sessions_with_tag_group(
        client: &Client,
        project_id: i32,
        tag_group: &TagGroup,
    ) -> Result<Vec<Session>, DataError> {
        Ok(Self::get_sessions(client, project_id)
            .await?
            .into_iter()
            .filter(|session| session.contains_tag_group(tag_group))
            .collect::<Vec<Session>>())
    }

    pub async fn get_percentages(
        client: &Client,
        project_id: i32,
        tag_groups: &[TagGroup],
    ) -> Result<Vec<Percentage>, DataError> {
        let sessions_lists = futures::future::join_all(
            tag_groups
                .iter()
                .map(|tag_group| Self::get_sessions_with_tag_group(client, project_id, tag_group)),
        )
            .await
            .into_iter()
            .collect::<Result<Vec<Vec<Session>>, DataError>>();

        let session_counts = sessions_lists
            .into_iter()
            .map(|list| list.len())
            .collect::<Vec<usize>>();

        let total_count = Self::get_sessions_count(client, project_id).await?;

        if total_count == 0 {
            return Err(DataError::NoSessionFound);
        }

        Ok(session_counts
            .into_iter()
            .map(|count| ((count / total_count) as u32).into())
            .collect::<Vec<Percentage>>())
    }

    async fn get_sessions_count(client: &Client, project_id: i32) -> Result<usize, DataError> {
        Ok(Self::get_sessions(client, project_id).await?.len())
    }

    fn contains_tag_group(&self, tag_group: &TagGroup) -> bool {
        self.reports
            .iter()
            .any(|report| tag_group.contains_any(&report.tags))
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

    pub fn into_grouped_session(self, tag_groups: &[TagGroup]) -> GroupedSession {
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

#[derive(Serialize, Deserialize)]
type SessionsAnalytics = Vec<StepAnalysis>;

#[derive(Serialize, Deserialize)]
pub struct StepAnalysis {
    pub step_number: usize,
    pub average_duration: i64,
    pub tag_groups_sorted: Vec<TagGroup>,
}

pub fn grouped_sessions_to_session_analysis(grouped_sessions: &[GroupedSession]) -> SessionsAnalytics {
    // TODO: don't guess the max
    let max_steps = 100;
    let mut session_analytics = vec![];
    for step_number in 0..max_steps {
        let step_analysis = get_step_analysis(grouped_sessions, step_number);
        session_analytics.push(step_analysis);
    }
    session_analytics
}

pub fn get_step_analysis(grouped_sessions: &[GroupedSession], step_number: usize) -> StepAnalysis {
    let mut tag_group_counts = HashMap::<TagGroup, u32>::new();
    let mut duration_sum = 0;
    let sessions_count = grouped_sessions.len() as i64;

    // count tag-groups
    grouped_sessions.iter().for_each(|gs| {
        if gs.steps.len() <= step_number {
            return;
        }

        let step = &gs.steps[step_number];
        duration_sum += step.avg_time_ms;

        let prev_count = tag_group_counts.entry(step.tag_group.clone()).or_insert(0);
        tag_group_counts.insert(step.tag_group.clone(), prev_count + 1);
    });

    // sort the tag-groups based on their count
    let mut tag_group_counts: Vec<(TagGroup, i32)> = tag_group_counts.iter().collect();
    tag_group_counts.sort_by(|l, r| {
        l.1.cmp(&r.1)
    });

    StepAnalysis {
        step_number,
        average_duration: duration_sum / sessions_count,
        tag_groups_sorted: tag_group_counts.iter().map(|(key, val)| k).collect()
    }
}


