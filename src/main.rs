use std::path::PathBuf;

use clap::Parser;

use maya::{Maya, MayaSubcommand};
use sh::Sh;

mod config;
mod create;
mod maya;
mod sh;

fn main() {
  let args = Maya::parse();
  let config = config::Config::get();

  match args.maya {
    MayaSubcommand::Sh => {
      let sh = Sh::new(&PathBuf::from(&config.scripts_dir));
      sh.run().unwrap();
    }
    MayaSubcommand::Create => {
      create::Create::run().unwrap();
      println!("{}", ansi_term::Color::Green.paint("File created successfully"));
    }
  }
}

