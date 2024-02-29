use std::fs;

use clap::Parser;

pub mod core;
pub mod gui;
pub mod utils;

use core::{commands::Command, daemon};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    json: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let config_path = cli
        .json
        .unwrap_or_else(|| shellexpand::tilde("~/.config/iced_prompt/commands.json").into_owned());
    let json_string = fs::read_to_string(config_path).expect("{} doesn't exist");

    let command: Command = serde_json::from_str(&json_string).expect("Unable to parse json");

    match gui::main(command) {
        Ok(cmd) => {
            let _ = daemon::exec(cmd.command_string());
            std::process::exit(0);
        }
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };
}
