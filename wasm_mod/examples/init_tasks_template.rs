use wasm_mod::task::TaskEnum;

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
    ];
    let serialized = serde_json::to_string(&tasks).map_err(|e| e.to_string()).unwrap();

    let filename = Args::parse().output;
    std::fs::write(filename, serialized).unwrap();
}
