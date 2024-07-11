use super::utils::{self, path_exists};
use crate::core::config::Config;
use std::process;

pub fn create_and_open_quick_note(config: Config) {
    let note_name: String = cliclack::input("What do you want to name your new Quick-Note?")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please enter a name.")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    let filepath = config.core.note_dir.clone() + "/quick-notes/" + note_name.as_str() + ".md";
    if path_exists(filepath.clone()) {
        cliclack::note("-_-", "Name is taken.").unwrap();
        process::exit(0);
    }

    utils::ensure_all_dirs(filepath.clone());
    utils::save_and_open_file(filepath, config);
}
