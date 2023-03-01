use std::{path::PathBuf, fs::File, io::Read};

use anyhow::Result;
use serde::Deserialize;

const CONFIG_NAME: &str = "Tectonic.toml";

#[derive(Deserialize, Debug)]
pub struct TectonicConfig {
    doc: Doc,
    #[serde(rename = "output")]
    outputs: Vec<Output>
}

#[derive(Deserialize, Debug)]
struct Doc {
    name: String
}

#[derive(Deserialize, Debug)]
struct Output {
    name: String,
    #[serde(rename = "type")]
    file_type: String
}

impl TectonicConfig {
    pub fn load(path: &PathBuf) -> Result<TectonicConfig> {
        // Open the config file
        let mut file = File::open(path.join(CONFIG_NAME)).unwrap();
        // Read it to a string
        let mut config_string = String::new();
        file.read_to_string(&mut config_string).expect("couldn't read `Tectonic.toml`");

        // Parse to config struct
        let res: TectonicConfig = toml::from_str(&config_string)?;
        Ok(res)
    }

    /// Get a path to the output file _relative_ to the `Tectonic.toml` config file.
    pub fn get_output_path(&self) -> PathBuf {
        let output = &self.outputs[0];
        PathBuf::from("build")
            .join(&self.doc.name)
            .join(format!("{}.{}", output.name, output.file_type))
    }
}