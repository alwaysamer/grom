use crate::core::config::Config;
use std::{
    fs::{self, OpenOptions},
    io::{self, ErrorKind},
    path::Path,
    process::Command,
};

pub fn save_file(file: String) -> Result<(), io::Error> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file.clone())?;
    Ok(())
}

pub fn open_file(cmd: String, file: String) -> Result<(), io::Error> {
    Command::new(cmd).arg(file).status()?;
    Ok(())
}

pub fn ensure_all_dirs(path: String) -> Result<(), io::Error> {
    let path = Path::new(path.as_str());
    if let Some(parent) = path.parent() {
        match fs::create_dir_all(parent) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.exists()
}

pub fn find_projects(dir: String) -> Result<Vec<(String, String)>, io::Error> {
    let mut projects = Vec::new();
    let dir = Path::new(&dir);

    let entries = fs::read_dir(dir).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            let start_md_path = path.join("start.md");
            if start_md_path.exists() {
                if let Some(parent) = path.file_name().and_then(|n| n.to_str()) {
                    projects.push((
                        String::from(start_md_path.to_str().unwrap()),
                        parent.to_string(),
                    ))
                }
            }
        }
    }
    Ok(projects)
}

pub fn select_project(config: Config) -> Result<String, io::Error> {
    let projects_dir = config.core.note_dir + "/projects";
    let projects = match find_projects(projects_dir) {
        Ok(p) => p,
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                "Couldn't find or access either the projects-dir or even the whole note-dir.",
            ))
        }
    };
    if projects.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No Projects found.",
        ));
    }
    cliclack::intro(console::style(" Grom ").on_cyan().black()).unwrap();

    let mut items: Vec<(String, String, String)> = Vec::new();
    for (_index, (name, path)) in projects.iter().enumerate() {
        items.push((name.clone(), path.clone(), String::from("")));
    }
    match cliclack::select(format!("Select a Project"))
        .items(&items)
        .interact()
    {
        Ok(index) => return Ok(index),
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::Interrupted,
                "Couldn't select a project.",
            ))
        }
    };
}
