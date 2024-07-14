use std::process;

use crate::core::config::Config;
use crate::core::git;

pub fn init(remote: String, config: Config) {
    match git::init_sync(config.core.note_dir, remote) {
        Ok(()) => {
            cliclack::note("^_^", "Sync via. git has been initialized.").unwrap();
            process::exit(0);
        }
        Err(e) => {
            cliclack::note(
                "T_T",
                format!("Initializing sync via. git has failed: {}", e),
            )
            .unwrap();
            process::exit(1);
        }
    }
}

pub fn push(message: String, config: Config) {
    match git::push_changes(config.core.note_dir, message) {
        Ok(()) => {
            cliclack::note("^_^", "Sync has been pushed.").unwrap();
            process::exit(0);
        }
        Err(e) => {
            cliclack::note("T_T", format!("Pushing changes has failed: {}", e)).unwrap();
            process::exit(1);
        }
    }
}

pub fn pull(config: Config) {
    match git::pull_changes(config.core.note_dir) {
        Ok(()) => {
            cliclack::note("^_^", "Changes have been pulled.").unwrap();
            process::exit(0);
        }
        Err(e) => {
            cliclack::note("T_T", format!("Pulling changes has failed: {}", e)).unwrap();
            process::exit(1);
        }
    }
}
