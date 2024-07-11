use crate::core::config::Config;
use std::{
    fs::{self, OpenOptions},
    io::{self},
    path::Path,
    process::{self, Command},
};

pub fn save_file(file: String) {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file.clone())
    {
        Ok(_) => {
            #[cfg(debug_assertions)]
            println!("Note created successfully!");
        }
        Err(e) => match e.kind() {
            _ => {
                #[cfg(debug_assertions)]
                println!("Error creating Note : {:?}", e);
                process::exit(1);
            }
        },
    };
}

pub fn open_file(cmd: String, file: String) {
    let status = Command::new(cmd).arg(file).status();
    match status {
        Ok(s) if s.success() => {
            #[cfg(debug_assertions)]
            println!("Note opened successfully!");
            process::exit(0);
        }
        Ok(_s) => {
            #[cfg(debug_assertions)]
            eprintln!("Command couldn't open File");
            process::exit(1);
        }
        Err(_e) => {
            #[cfg(debug_assertions)]
            eprintln!("Error executing open_cmd!");
            process::exit(1);
        }
    }
}

pub fn ensure_all_dirs(path: String) {
    let path = Path::new(path.as_str());
    if let Some(parent) = path.parent() {
        if let Err(_e) = fs::create_dir_all(parent) {
            #[cfg(debug_assertions)]
            eprintln!("Unable to create Parent-Directories!");
            process::exit(1);
        }
    }
}

pub fn save_and_open_file(file: String, config: Config) {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file.clone())
    {
        Ok(_) => {
            #[cfg(debug_assertions)]
            println!("Note created successfully!");
        }
        Err(e) => match e.kind() {
            _ => {
                #[cfg(debug_assertions)]
                println!("Error creating Note : {:?}", e);
                process::exit(1);
            }
        },
    };
    open_file(config.core.open_cmd, file);
    process::exit(0);
}

pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.exists()
}

pub fn find_projects(dir: String) -> Option<Vec<(String, String)>> {
    let mut projects = Vec::new();
    let dir = Path::new(&dir);

    if let Ok(entries) = fs::read_dir(dir) {
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
        Some(projects)
    } else {
        None
    }
}

pub fn select_project(config: Config) -> Result<String, io::Error> {
    let projects_dir = config.core.note_dir + "/projects";
    let projects = find_projects(projects_dir).unwrap();
    if projects.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No Projects found.",
        ));
    }
    cliclack::intro(console::style(" NoJo ").on_cyan().black()).unwrap();

    let mut items: Vec<(String, String, String)> = Vec::new();
    for (_index, (name, path)) in projects.iter().enumerate() {
        items.push((name.clone(), path.clone(), String::from("")));
    }
    return Ok(cliclack::select(format!("Select a Project"))
        .items(&items)
        .interact()
        .unwrap());
}
