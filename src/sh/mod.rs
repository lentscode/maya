use ansi_term::Color::Red;
use inquire::{ui::RenderConfig, Select};
use std::io::Result;
use std::{fs, path::PathBuf, process::Command};

type Filename = String;

pub struct Sh {
  files: Vec<String>,
  dir: PathBuf,
}

impl Sh {
  pub fn new(dir: &PathBuf) -> Self {
    Sh {
      files: Self::get_sh_files(dir).unwrap_or(Vec::new()),
      dir: dir.clone(),
    }
  }

  pub fn run(&self) -> Result<()> {
    self.display_choices()
  }

  fn get_sh_files(dir: &PathBuf) -> Result<Vec<Filename>> {
    let mut files: Vec<Filename> = Vec::new();

    for entry in fs::read_dir(dir)? {
      let entry = entry?;

      if entry.path().extension().unwrap() == "sh" {
        files.push(
          entry
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        );
      }
    }

    files.sort_unstable();

    Ok(files)
  }

  fn run_script_file(&self, file: &str) -> Result<()> {
    Command::new("sh")
      .arg(self.dir.join(file))
      .status()
      .unwrap();
    Ok(())
  }

  fn display_choices(&self) -> Result<()> {
    if self.files.is_empty() {
      eprintln!(
        "{}",
        Red.paint(format!("No script files found in '{}'", self.dir.display()))
      );
      return Ok(());
    }

    let config = RenderConfig::default_colored();
    let file_to_run = Select::new("Select a script file to run:", self.files.clone())
      .with_render_config(config)
      .prompt()
      .unwrap();

    self.run_script_file(&file_to_run)?;
    Ok(())
  }
}
