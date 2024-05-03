use chrono::TimeDelta;
use wasm_mod::task::{
    TaskEnum,
    conditional::{
        ConditionalTask,
        action::AddReadingList,
        condition::{Always, Changed,},
        scheduler::{Interval, OneTime,},
    },
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output: String,
}

fn main() {
    let tasks: Vec<TaskEnum> = vec![
        // Your tasks here
        ConditionalTask::new(
            "name",
            None,
            Changed::new("https://", "(//a)[1]/@href"),
            Interval::new(TimeDelta::try_weeks(1).unwrap()),
            AddReadingList::new("https://", "name"),
        ).into(),
        ConditionalTask::new(
            "name",
            None,
            Always{},
            OneTime{},
            AddReadingList::new("https://", "name"),
        ).into(),
    ];
    let serialized = serde_json::to_string(&tasks).map_err(|e| e.to_string()).unwrap();

    let filename = Args::parse().output;
    std::fs::write(filename, serialized).unwrap();
}
