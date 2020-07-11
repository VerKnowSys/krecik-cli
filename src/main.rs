//! Crate docs

#![forbid(unsafe_code)]
#![deny(
    missing_docs,
    unstable_features,
    unsafe_code,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
// For development:
#![allow(dead_code, unused_imports, unused_variables)]

#[macro_use]
extern crate log;

use chrono::*;
use colored::Colorize;
use krecik::api::*;
use lockfile::Lockfile;
use log::*;
use std::fs;
use std::io::{Error, ErrorKind};
use std::{env, env::var, path::Path};


fn setup_logger(level: LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}


fn main() -> Result<(), Error> {
    let logger_level = match var("DEBUG") {
        Ok(value) => LevelFilter::Debug,
        Err(_) => LevelFilter::Info,
    };
    setup_logger(logger_level).unwrap_or_default();

    const LOCK_PATH: &str = "/tmp/krecik-remote-cli.lock";
    ctrlc::set_handler(|| {
        println!("Interrupted! Removing lock file: {}", LOCK_PATH);
        fs::remove_file(LOCK_PATH).unwrap_or_default();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    Lockfile::create(LOCK_PATH)
        .and_then(|lockfile| {
            if env::args().len() == 1 {
                error!("You have to specify path(s) to json check file!")
            } else {
                for check_file in env::args().skip(1).collect::<Vec<String>>() {
                    if Path::new(&check_file).exists() {
                        info!("Loading check from: {}", &check_file.green());
                        let history = execute_checks_from_file(&check_file);
                        debug!("History: {:?}", &history);
                    } else {
                        error!("Check file not found: {}", &check_file.red())
                    }
                }
            }
            lockfile.release()?; // or just let the lockfile be dropped
            Ok(())
        })
        .map_err(|err| {
            error!(
                "Lock file acquired. Other instance is already running? Error: {}",
                err
            );
            err
        })
}
