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
use krecik::products::story::Story;
use log::*;
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

    if env::args().len() == 0 {
        Err(Error::new(
            ErrorKind::InvalidData,
            "You have to specify path(s) to json check file!",
        ))
    } else {
        for check_file in env::args().skip(1).collect::<Vec<String>>() {
            if Path::new(&check_file).exists() {
                info!("Loading check from: {}", &check_file);
                let history = execute_checks_from_file(&check_file);
                debug!("History: {:?}", history);
            } else {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Check file not found: {}", check_file),
                ));
            }
        }
        Ok(())
    }
}
