use crate::core::config::Config;
use crate::core::utils;
use std::process;

pub fn quick_note(note_name: String, config: Config) {
    let filepath = config.core.note_dir.clone() + "/quick-notes/" + note_name.as_str() + ".md";
    if utils::path_exists(filepath.clone()) {
        match utils::open_file(config.core.editor, filepath) {
            Ok(_) => process::exit(0),
            Err(e) => {
                cliclack::note("T_T", format!("Unable to open file: {}", e)).unwrap();
                process::exit(1);
            }
        }
    }
    match utils::ensure_all_dirs(filepath.clone()) {
        Ok(_) => (),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to create all parent-dirs: {}", e)).unwrap();
            process::exit(1);
        }
    }
    match utils::save_file(filepath.clone()) {
        Ok(_) => (),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to save file: {}", e)).unwrap();
            process::exit(1);
        }
    }

    match utils::open_file(config.core.editor, filepath) {
        Ok(_) => process::exit(0),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to open file: {}", e)).unwrap();
            process::exit(1);
        }
    }
}
