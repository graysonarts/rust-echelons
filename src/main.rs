mod options;

use std::{
    fs::create_dir,
    path::{Path, PathBuf},
};

use echelons::{
    self,
    errors::{UserError, UserFacingResult},
};
use log::{error, info, LevelFilter};
use options::Options;
use structopt::StructOpt;
use structopt_flags::LogLevel;

use crate::options::find_config;

fn init_logger(log_level: LevelFilter) {
    pretty_env_logger::formatted_builder()
        .filter_level(log_level)
        .init();
}

fn ensure_directory(p: &PathBuf) -> UserFacingResult<()> {
    if !p.exists() {
        create_dir(p).map_err(|e| UserError::from(e))?;
    }

    Ok(())
}

fn run() -> UserFacingResult<()> {
    let opts = Options::from_args();
    let log_level = opts.verbose.get_level_filter();
    init_logger(log_level);

    if !opts.root_directory.exists() {
        return Err(UserError::DirectoryDoesNotExist(opts.root_directory));
    }

    let config_filename = opts.config.or_else(|| find_config(&opts.root_directory));
    let config_filename = config_filename.ok_or(UserError::ConfigNotFound)?;
    if !config_filename.exists() {
        return Err(UserError::DirectoryDoesNotExist(config_filename));
    }

    let target_directory = opts
        .project_name
        .map(|pn| opts.root_directory.join(pn))
        .unwrap_or(opts.root_directory);
    ensure_directory(&target_directory)?;

    info!(
        "Config: {:?}, Target Directory: {:?}",
        config_filename, target_directory
    );
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(err) => error!("Unable to continue: {}", err),
    };
}
