use std::process;

use crate::core::config::Config;
use crate::core::utils;
use chrono::{Datelike, Local};

pub fn daily_diary(config: Config) {
    let today = Local::now();
    let file = format!(
        "{}/diary/{}/{}/week{}/{}.md",
        config.core.note_dir,
        today.year(),
        today.format("%B"),
        today.iso_week().week(),
        today.format("%m-%d-%Y")
    );
    if utils::path_exists(file.clone()) {
        match utils::open_file(config.core.open_cmd.clone(), file.clone()) {
            Ok(_) => process::exit(0),
            Err(e) => {
                cliclack::note("T_T", format!("Unable to open file:  {}", e)).unwrap();
                process::exit(1);
            }
        }
    }
    match utils::ensure_all_dirs(file.clone()) {
        Ok(_) => (),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to create all parent-dirs: {}", e)).unwrap();
            process::exit(1);
        }
    }
    match utils::save_file(file.clone()) {
        Ok(_) => (),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to save file: {}", e)).unwrap();
            process::exit(1);
        }
    }

    if config.core.always_open {
        match utils::open_file(config.core.open_cmd.clone(), file.clone()) {
            Ok(_) => process::exit(0),
            Err(e) => {
                cliclack::note("T_T", format!("Unable to open file:  {}", e)).unwrap();
                process::exit(1);
            }
        }
    }
}

pub fn weekly_diary(config: Config) {
    let today = Local::now();
    let file = format!(
        "{}/diary/{}/{}/week{}/week.md",
        config.core.note_dir,
        today.year(),
        today.format("%B"),
        today.iso_week().week(),
    );
    if utils::path_exists(file.clone()) {
        match utils::open_file(config.core.open_cmd.clone(), file.clone()) {
            Ok(_) => (),
            Err(e) => {
                cliclack::note("T_T", format!("Unable to open file: {}", e)).unwrap();
                process::exit(1);
            }
        }
    }
    match utils::ensure_all_dirs(file.clone()) {
        Ok(_) => (),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to create all parent-dirs: {}", e)).unwrap();
            process::exit(1);
        }
    }
    match utils::save_file(file.clone()) {
        Ok(_) => (),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to save file {}", e)).unwrap();
            process::exit(0);
        }
    }
    if config.core.always_open {
        match utils::open_file(config.core.open_cmd.clone(), file.clone()) {
            Ok(_) => process::exit(0),
            Err(e) => {
                cliclack::note("T_T", format!("Unable to open file:  {}", e)).unwrap();
                process::exit(1);
            }
        }
    }
}

pub fn monthly_diary(config: Config) {
    let today = Local::now();
    let file = format!(
        "{}/diary/{}/{}/month.md",
        config.core.note_dir,
        today.year(),
        today.format("%B"),
    );
    if utils::path_exists(file.clone()) {
        match utils::open_file(config.core.open_cmd.clone(), file.clone()) {
            Ok(_) => (),
            Err(e) => {
                cliclack::note("T_T", format!("Unable to open file: {}", e)).unwrap();
                process::exit(1);
            }
        }
    }
    match utils::ensure_all_dirs(file.clone()) {
        Ok(_) => (),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to create all parent-dirs: {}", e)).unwrap();
            process::exit(1)
        }
    }
    match utils::save_file(file.clone()) {
        Ok(_) => (),
        Err(e) => {
            cliclack::note("T_T", format!("Unable to save file: {}", e)).unwrap();
            process::exit(1);
        }
    }
    if config.core.always_open {
        match utils::open_file(config.core.open_cmd.clone(), file.clone()) {
            Ok(_) => process::exit(0),
            Err(e) => {
                cliclack::note("T_T", format!("Unable to open file:  {}", e)).unwrap();
                process::exit(1);
            }
        }
    }
}
