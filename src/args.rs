use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MainArgs {
    pub path: Option<PathBuf>
}
