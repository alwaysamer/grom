use crate::core::config::Config;
use crate::core::utils;
use std::process;

pub fn quick_note(config: Config) {
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
    if utils::path_exists(filepath.clone()) {
        cliclack::note("-_-", "Name is taken.").unwrap();
        process::exit(0);
    }

    match utils::ensure_all_dirs(filepath.clone()) {
        Ok(_) => (),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to create all parent-dirs: {}", e)).unwrap();
            process::exit(1);
        }
    }
    match utils::save_file(filepath) {
        Ok(_) => process::exit(0),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to save file: {}", e)).unwrap();
            process::exit(1);
        }
    }
}
