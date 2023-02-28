use std::{path::PathBuf, fs::File, io::Read};

use anyhow::Result;
use serde::Deserialize;

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
    pub fn load() -> Result<TectonicConfig> {
        let mut file = File::open("Tectonic.toml")?;
        let mut config_string = String::new();
        file.read_to_string(&mut config_string).expect("Couldn't read config file `Tectonic.toml`");

        let res = toml::from_str(&config_string);
        if res.is_err() {
            let err = res.unwrap_err();
            panic!("Couln't parse `Tectonic.toml`: {}", err.message());
        }
        Ok(res.unwrap())
    }

    /// Get a path to the output file _relative_ to the `Tectonic.toml` config file.
    pub fn get_output_path(&self) -> PathBuf {
        let output = &self.outputs[0];
        PathBuf::from("build").join(&self.doc.name).join(format!("{}.{}", output.name, output.file_type))
    }
}