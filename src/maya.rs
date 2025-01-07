use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, name = "maya")]
pub struct Maya {
  #[clap(subcommand)]
  pub maya: MayaSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum MayaSubcommand {
  /// Run or manage script files
  Sh,
  /// Create a file from a template
  Create,
}
