pub mod errors;
pub mod project;
pub mod sway;

use errors::Error;
use project::Project;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

#[derive(Default, StructOpt)]
#[structopt(global_settings = &[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])]
pub struct Options {
    #[structopt(long)]
    pub contract_files: Vec<PathBuf>,
}

fn main() {
    if let Err(e) = translate_project() {
        eprintln!("{e}");
    }
}

fn translate_project() -> Result<(), Error> {
    let options = Options::from_args_safe()
        .map_err(|e| Error::Wrapped(Box::new(e)))?;

    let mut project = Project::try_from(&options)?;
    project.translate()
}