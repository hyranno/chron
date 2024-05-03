use chrono::{DateTime, TimeDelta, Utc};
use futures::Future;
use serde::{Deserialize, Serialize};


// #[enum_dispatch(SchedulerEnum)]
pub trait Scheduler {
    fn next(&mut self) -> impl Future<Output = Option<DateTime<Utc>>>;
}
// #[enum_dispatch]
#[derive(Serialize, Deserialize)]
pub enum SchedulerEnum {
    OneTime(OneTime),
    Interval(Interval),
}
impl Scheduler for SchedulerEnum {
    async fn next(&mut self) -> Option<DateTime<Utc>> {
        match self {
            Self::OneTime(value) => value.next().await,
            Self::Interval(value) => value.next().await,
        }        
    }
}
impl From<OneTime> for SchedulerEnum {fn from(value: OneTime) -> Self {Self::OneTime(value)}}
impl From<Interval> for SchedulerEnum {fn from(value: Interval) -> Self {Self::Interval(value)}}

#[derive(Serialize, Deserialize)]
pub struct OneTime {}
impl Scheduler for OneTime {
    async fn next(&mut self) -> Option<DateTime<Utc>> {
        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct Interval {
    interval_seconds: i64,  // TimeDelta is not Serializable
    previous: Option<DateTime<Utc>>,
}
impl Interval {
    pub fn new(interval: TimeDelta) -> Self {
        Self { interval_seconds: interval.num_seconds(), previous: None }
    }
}
impl Scheduler for Interval {
    async fn next(&mut self) -> Option<DateTime<Utc>> {
        let previous = self.previous.unwrap_or(Utc::now());
        let delay = Utc::now() - previous;
        let interval = TimeDelta::try_seconds(self.interval_seconds).unwrap();
        let mut next = previous + interval;
        while next < previous + delay {
            next += interval;
        }
        self.previous = Some(next);
        Some(next)
    }
}
