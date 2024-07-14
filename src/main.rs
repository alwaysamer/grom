use std::process;

use clap::{Parser, Subcommand};
use grom::commands::{diary, project, quick_note, sync};
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
    Sync {
        #[command(subcommand)]
        command: SyncCommand,
    },
}

#[derive(Subcommand)]
enum SyncCommand {
    Init {
        #[arg(value_name = "REMOTE_URL")]
        remote_url: String,
    },
    Push {
        #[arg(value_name = "MESSAGE")]
        message: String,
    },
    Pull {},
}

fn main() {
    ctrlc::set_handler(move || {}).expect("settings ctrl-c handler");
    let cli = Cli::parse();
    let config = match config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            cliclack::note("T_T", format!("Unable to load config: {}", e.to_string())).unwrap();
            process::exit(1);
        }
    };

    if let Some(command) = &cli.command {
        match &command {
            Command::Quick {} => quick_note::quick_note(config),
            Command::Today {} => diary::daily_diary(config),
            Command::Week {} => diary::weekly_diary(config),
            Command::Month {} => diary::monthly_diary(config),
            Command::New { project_name } => project::create(project_name.clone(), config),
            Command::Sync { command } => match command {
                SyncCommand::Init { remote_url } => {
                    sync::init(remote_url.clone(), config);
                }
                SyncCommand::Push { message } => sync::push(message.to_owned(), config),
                SyncCommand::Pull {} => sync::pull(config),
            },
        }
    } else if let Some(project) = &cli.project {
        project::open(project.clone(), config);
    } else {
        project::interative_selecion(config);
    }
}
