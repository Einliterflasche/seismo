mod args;
mod tectonic;

use std::{env::current_dir, path::PathBuf};

use clap::Parser;
use hotwatch::{blocking::{Hotwatch, Flow}, Event};

use crate::tectonic::TectonicConfig;

fn main() {
    let args = args::MainArgs::parse();

    let curr_dir = current_dir().expect("couldn't access current working directory");
    // The root of the project dir
    let root_dir = if let Some(path) = args.path {
        if path.is_relative() { curr_dir.join(path) } 
        else { path }
    } else {
        curr_dir
    }.canonicalize().expect("error canonilazing the path");

    // The source folder of the project
    let src_dir = root_dir.join("src");
    // Load the config file and output path
    let config = TectonicConfig::load(&root_dir).unwrap();
    let output_path = config.get_output_path();

    println!("Started watching `{}`", src_dir.display());
    let mut hotwatch = Hotwatch::new().expect("Failed to initialize hotwatch");

    // Build and open once, then start listening for changes
    cmd::build_and_open(&output_path).unwrap();
    hotwatch.watch(
        &src_dir, 
        create_handler(output_path)
    ).expect("Failed to watch path");

    hotwatch.run();
}

/// Create the handler function which is called upon a file change
fn create_handler(output_path: PathBuf) -> impl FnMut(Event) -> Flow {  
    move |_: Event| {
        cmd::build_and_open(&output_path).unwrap();
        Flow::Continue
    }
}

/// This module holds functions for running commands 
/// like opening the output file or building the project.
mod cmd {
    use std::{process::{Command, Output}, path::PathBuf, io};

    pub fn build_and_open(output_path: &PathBuf) -> Result<(), anyhow::Error> {
        xdg_open(output_path)?;
        tectonic_build()?;
        Ok(())
    }

    fn xdg_open(output_path: &PathBuf) -> io::Result::<Output> {
        Command::new("xdg-open").arg(&output_path).output()
    }

    fn tectonic_build() -> io::Result<Output> {
        Command::new("tectonic").arg("-X").arg("build").output()
    }
}