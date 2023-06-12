use clap::{Parser, Subcommand};

/// The CLI.
#[derive(Debug, Parser, Clone)]
#[command(
    name = clap::crate_name!(),
    author = clap::crate_authors!(),
    version = clap::crate_version!(),
    about = clap::crate_description!(),
)]
pub struct Cli {
    /// CLI Commands.
    #[command(subcommand)]
    pub command: Commands,
}

/// Enum of all commands.
#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    RetrieveBlock {
        /// The ID of the block to retrieve.
        #[clap(long)]
        block_id: String,
    },
}
