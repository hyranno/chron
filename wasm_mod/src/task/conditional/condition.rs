use futures::Future;
use serde::{Deserialize, Serialize};

use crate::external::Tab;


// #[enum_dispatch(ConditionEnum)]
pub trait Condition {
    fn check(&mut self) -> impl Future<Output = Result<bool, String>>;
}
// #[enum_dispatch]
#[derive(Serialize, Deserialize)]
pub enum ConditionEnum {
    Always(Always),
    Changed(Changed),
}
impl Condition for ConditionEnum {
    async fn check(&mut self) -> Result<bool, String> {
        match self {
            Self::Always(value) => value.check().await,
            Self::Changed(value) => value.check().await,
        }
    }
}
impl From<Always> for ConditionEnum {fn from(value: Always) -> Self { Self::Always(value) }}
impl From<Changed> for ConditionEnum {fn from(value: Changed) -> Self { Self::Changed(value) }}

#[derive(Serialize, Deserialize)]
pub struct Always {}
impl Condition for Always {
    async fn check(&mut self) -> Result<bool,String> {
        Ok(true)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Changed {
    url: String,
    xpath: String,
    previous: Option<String>,
}
impl Changed {
    pub fn new(url: &str, xpath: &str) -> Self {
        Self {
            url: String::from(url),
            xpath: String::from(xpath),
            previous: None,
        }
    }
}
impl Condition for Changed {
    async fn check(&mut self) -> Result<bool, String> {
        let tab = Tab::open(&self.url).await;
        let fetch_res = tab.fetch_string_by_xpath_w_retry(&self.xpath, 0).await;
        tab.close().await;
        fetch_res.map(|fetched| {
            let res = if let Some(prev) = self.previous.clone() {
                prev != fetched
            } else {
                true
            };
            self.previous = Some(fetched.clone());
            res
        })
    }
}
