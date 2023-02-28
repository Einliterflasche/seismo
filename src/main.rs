mod tectonic;

use std::{env::current_dir, path::PathBuf};

use hotwatch::{blocking::{Hotwatch, Flow}, Event};

fn main() {

    let curr_dir = current_dir().expect("couldn't access current working directory");
    let src_dir = curr_dir.join("src");

    let config = tectonic::TectonicConfig::load().unwrap();

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
    use std::{process::{Child, Command}, path::PathBuf, io};

    pub fn xdg_open(output_path: &PathBuf) -> io::Result::<Child> {
        Command::new("xdg-open").arg(&output_path).spawn()
    }

    pub fn tectonic_build() -> io::Result<Child> {
        Command::new("tectonic").arg("-X").arg("build").spawn()
    }
}