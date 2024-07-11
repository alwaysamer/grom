use super::utils;
use crate::core::config::Config;
use std::process;

pub fn create_project(project_name: String, config: Config) {
    let path = config.core.note_dir.clone() + "/projects/" + project_name.as_str() + "/start.md";

    if utils::path_exists(path.clone()) {
        cliclack::note("-_-", "Project already exists.").unwrap();
        process::exit(1);
    } else {
        utils::ensure_all_dirs(path.clone());
        utils::save_file(path);
        cliclack::note("^_^", "Project created.").unwrap();
        process::exit(0);
    }
}

pub fn open_project(project_name: String, config: Config) {
    let project_base: String =
        config.core.note_dir.clone() + "/projects/" + project_name.as_str() + "/start.md";
    if utils::path_exists(project_base.clone()) {
        utils::open_file(config.core.open_cmd.clone(), project_base.clone());
    } else {
        cliclack::note(":(", "Project does not exist.").unwrap();
    }
}

pub fn interative_project_selecion(config: Config) {
    let open_cmd = config.core.open_cmd.clone();
    let project = match utils::select_project(config) {
        Ok(p) => p,
        Err(_e) => {
            cliclack::note(":(", "No Projects found.").unwrap();
            process::exit(1);
        }
    };
    utils::open_file(open_cmd, project);
}
