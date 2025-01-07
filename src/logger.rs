//
// logger.rs
//
// Created by Zacharyliu-CS on 01/07/2025.
// Copyright (c) 2025 liuzhenm@mail.ustc.edu.cn.
//
use colored::*;
use env_logger::{Builder, Env};
use log::LevelFilter;
use std::io::Write;

pub fn init_logger(level: &str) {
    Builder::from_env(Env::default().default_filter_or(level))
        .format(|buf, record| {
            let level = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".cyan(),
                log::Level::Trace => "TRACE".magenta(),
            };
            writeln!(
                buf,
                "[{}][{}:{}] {}",
                level,
                record.file().unwrap_or("unkown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter(Some("actix"), LevelFilter::Error)
        .init();
}
