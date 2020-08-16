use crate::db::sessions::{GroupedSession, Step, TagGroup};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone)]
pub struct TagGroupFrequency {
    tag_group: TagGroup,
    count: u32,
    average_duration: Duration,
}

impl TagGroupFrequency {
    pub fn new(step: &Step) -> Self {
        TagGroupFrequency {
            tag_group: step.tag_group.clone(),
            count: 1,
            average_duration: step.duration.clone(),
        }
    }

    pub fn merge(&mut self, step: &Step) {
        self.average_duration = Duration::from_millis(
            ((self.average_duration.as_millis() * self.count as u128 + step.duration.as_millis())
                / (self.count + 1) as u128) as u64,
        );
        self.count += 1;
    }
}

pub async fn tag_group_frequency_at_step(
    grouped_sessions: Vec<GroupedSession>,
    step_number: usize,
) {
    let freqs = HashMap::<usize, TagGroupFrequency>::new();

    let steps = grouped_sessions
        .into_iter()
        .map(|session| session.steps[step_number].clone())
        .collect::<Vec<Step>>();

    for step in steps {
        let prev_freq = freqs.get(&step.step_number);
        let new_freq = match prev_freq {
            Some(fr) => {
                let mut a = fr.clone();
                a.merge(&step);
                a
            }
            None => TagGroupFrequency::new(&step),
        };
    }
}
