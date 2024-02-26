use std::fs;

use clap::Parser;

pub mod core;
pub mod gui;
pub mod utils;

use core::commands::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    json: String,
}

fn main() -> iced::Result {
    let cli = Cli::parse();

    let json_string = fs::read_to_string(cli.json).expect("Unable to read file");

    let command: Command = serde_json::from_str(&json_string).expect("Unable to parse json");

    gui::main(command)
}
