mod args;
mod tectonic;

use std::{env::current_dir, path::PathBuf};

use clap::Parser;
use hotwatch::{blocking::{Hotwatch, Flow}, Event};

use crate::tectonic::TectonicConfig;

fn main() {
    let args = args::MainArgs::parse();

    let curr_dir = current_dir().expect("couldn't access current working directory");

    let root_dir = if let Some(path) = args.path {
        if path.is_relative() { curr_dir.join(path) } 
        else { path }
    } else {
        curr_dir
    };

    let src_dir = root_dir.join("src");

    let config = TectonicConfig::load(&root_dir).unwrap();

    let output_path = config.get_output_path();

    println!("Started watching `{}`", src_dir.display());

    let mut hotwatch = Hotwatch::new().expect("Failed to initialize hotwatch");
    hotwatch.watch(
        &src_dir, 
        create_handler(output_path)
    ).expect("Failed to watch path");

    hotwatch.run();
}

fn create_handler(output_path: PathBuf) -> impl FnMut(Event) -> Flow {  
    cmd::xdg_open(&output_path).unwrap();
    move |_: Event| {
        cmd::tectonic_build().unwrap();
        cmd::xdg_open(&output_path).unwrap();
        Flow::Continue
    }
}

mod cmd {
    use std::{process::{Command, Output}, path::PathBuf, io};

    pub fn xdg_open(output_path: &PathBuf) -> io::Result::<Output> {
        Command::new("xdg-open").arg(&output_path).output()
    }

    pub fn tectonic_build() -> io::Result<Output> {
        Command::new("tectonic").arg("-X").arg("build").output()
    }
}