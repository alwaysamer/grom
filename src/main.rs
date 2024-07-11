use clap::{Parser, Subcommand};

use grom::commands::{diary, project, quick_note};
use grom::core::config;

#[derive(Parser)]
#[command(name = "grom")]
#[command(author = "alwaysamer")]
#[command(version = "1.0")]
#[command(about = "Note-Taking")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
    project: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    Quick {},
    New {
        #[arg(value_name = "PROJECT_NAME")]
        project_name: String,
    },
    Today {},
    Week {},
    Month {},
}

fn main() {
    ctrlc::set_handler(move || {}).expect("settings ctrl-c handler");
    let cli = Cli::parse();
    let config = config::load_config();

    if let Some(command) = &cli.command {
        match &command {
            Command::Quick {} => quick_note::create_and_open_quick_note(config),
            Command::Today {} => diary::create_or_open_daily_diary(config),
            Command::Week {} => diary::create_or_open_weekly_diary(config),
            Command::Month {} => diary::create_or_open_monthly_diary(config),
            Command::New { project_name } => project::create_project(project_name.clone(), config),
        }
    } else if let Some(project) = &cli.project {
        project::open_project(project.clone(), config);
    } else {
        project::interative_project_selecion(config);
    }
}
