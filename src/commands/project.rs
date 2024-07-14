use crate::core::config::Config;
use crate::core::utils;
use std::process;

pub fn create(project_name: String, config: Config) {
    let path = config.core.note_dir.clone() + "/projects/" + project_name.as_str() + "/start.md";

    if utils::path_exists(path.clone()) {
        cliclack::note("-_-", "Project already exists.").unwrap();
        process::exit(1);
    } else {
        match utils::ensure_all_dirs(path.clone()) {
            Ok(_) => (),
            Err(e) => {
                cliclack::note("T_T", format!("Unable to create all parent-dirs: {}", e)).unwrap();
                process::exit(1);
            }
        }
        match utils::save_file(path) {
            Ok(_) => {
                cliclack::note("^_^", "Project created.").unwrap();
                process::exit(0);
            }
            Err(e) => {
                cliclack::note("T_T", format!("Unable to save file: {}", e)).unwrap();
                process::exit(1);
            }
        }
    }
}

pub fn open(project_name: String, config: Config) {
    let project_base: String =
        config.core.note_dir.clone() + "/projects/" + project_name.as_str() + "/start.md";
    if utils::path_exists(project_base.clone()) {
        match utils::open_file(config.core.editor.clone(), project_base.clone()) {
            Ok(_) => (),
            Err(e) => {
                cliclack::note("T_T", format!("Unable to open file: {}", e)).unwrap();
                process::exit(1);
            }
        }
    } else {
        cliclack::note("-_-", "Project does not exist.").unwrap();
        process::exit(0);
    }
}

pub fn interative_selecion(config: Config) {
    let open_cmd = config.core.editor.clone();
    let project = match utils::select_project(config) {
        Ok(p) => p,
        Err(_e) => {
            cliclack::note("-_-", "No Projects found.").unwrap();
            process::exit(1);
        }
    };
    match utils::open_file(open_cmd, project) {
        Ok(_) => process::exit(0),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to open file: {}", e)).unwrap();
            process::exit(1);
        }
    }
}
