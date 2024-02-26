use clap::Parser;

pub mod core;
pub mod gui;
pub mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CLI {
    #[arg(short, long)]
    json: String,
}

fn main() -> iced::Result {
    gui::main()
    // let cli = CLI::parse();

    // let data: Command = cli.json
    //     .and_then(|path| fs::read_to_string(file_path).expect("Unable to read file"))
    //     .and_then(|json| serde_json::from_str(json).expect("Unable to parse json"))
}
