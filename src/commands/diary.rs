use super::utils::{self, path_exists};
use crate::core::config::Config;
use chrono::{Datelike, Local};

pub fn create_or_open_daily_diary(config: Config) {
    let today = Local::now();
    let file = format!(
        "{}/diary/{}/{}/week{}/{}.md",
        config.core.note_dir,
        today.year(),
        today.format("%B"),
        today.iso_week().week(),
        today.format("%m-%d-%Y")
    );
    if path_exists(file.clone()) {
        utils::open_file(config.core.open_cmd.clone(), file.clone());
    }
    utils::ensure_all_dirs(file.clone());
    utils::save_and_open_file(file, config);
}

pub fn create_or_open_weekly_diary(config: Config) {
    let today = Local::now();
    let file = format!(
        "{}/diary/{}/{}/week{}/week.md",
        config.core.note_dir,
        today.year(),
        today.format("%B"),
        today.iso_week().week(),
    );
    if path_exists(file.clone()) {
        utils::open_file(config.core.open_cmd.clone(), file.clone());
    }
    utils::ensure_all_dirs(file.clone());
    utils::save_and_open_file(file, config)
}

pub fn create_or_open_monthly_diary(config: Config) {
    let today = Local::now();
    let file = format!(
        "{}/diary/{}/{}/month.md",
        config.core.note_dir,
        today.year(),
        today.format("%B"),
    );
    if path_exists(file.clone()) {
        utils::open_file(config.core.open_cmd.clone(), file.clone());
    }
    utils::ensure_all_dirs(file.clone());
    utils::save_and_open_file(file, config)
}
