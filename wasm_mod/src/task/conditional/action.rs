use futures::Future;
use serde::{Deserialize, Serialize};
use enum_dispatch::enum_dispatch;


#[enum_dispatch(ActionEnum)]
pub trait Action {
    fn run(&mut self) -> impl Future<Output = Result<(), String>>;
}
#[enum_dispatch]
#[derive(Serialize, Deserialize)]
pub enum ActionEnum {
    AddReadingList,
}

#[derive(Serialize, Deserialize)]
pub struct AddReadingList {
    url: String,
    title: String,
}
impl AddReadingList {
    pub fn new(url: &str, title: &str) -> Self {
        Self { url: String::from(url), title: String::from(title) }
    }
}
impl Action for AddReadingList {
    async fn run(&mut self) -> Result<(),String> {
        #[allow(unused_must_use)]
        {crate::external::add_to_reading_list(&self.url, &self.title).await;}  // May fail if the entry exists.
        Ok(())
    }
}
