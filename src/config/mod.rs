use ansi_term::Color::Yellow;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Error, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub scripts_dir: String,
}

impl Config {
  pub fn get() -> Self {
    let local_config_file = get_local_config_file();
    let global_config_file = get_global_config_file();

    match local_config_file {
      Ok(file) => parse_config_file(file),
      Err(_) => {
        println!(
          "{}",
          Yellow.paint("No local config file found, using global config file")
        );

        match global_config_file {
          Ok(file) => parse_config_file(file),
          Err(_) => {
            println!(
              "{}",
              Yellow.paint("No global config file found, using default config\n")
            );
            Config::default()
          }
        }
      }
    }
  }

  fn default() -> Self {
    Config {
      scripts_dir: String::from("scripts"),
    }
  }
}

fn get_local_config_file() -> Result<File, Error> {
  let config_file_path = Path::new(".maya.json");
  File::open(config_file_path)
}

fn get_global_config_file() -> Result<File, Error> {
  let config_file_path = Path::new("/.maya.json");
  File::open(config_file_path)
}

fn parse_config_file(file: File) -> Config {
  let config = serde_json::from_reader::<File, Config>(file);

  if let Ok(config) = config {
    config
  } else {
    println!(
      "{}",
      Yellow.paint("Failed to parse config file, using default config")
    );
    Config::default()
  }
}
