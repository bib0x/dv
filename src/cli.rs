use clap::{Parser, Subcommand};

use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "dvenv")]
#[command(about = "CLI for poping into Nix shell", long_about = None)]
pub struct Cli {

    /// Directory containing the flake.nix file (default: DVENV_FLAKE_DIR shell variable)
    #[arg(short, long, value_name = "DIR")]
    pub path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List available environments
    List,

    /// Run a command from a targeted development environment
    #[command(arg_required_else_help = true)]
    Run {
        /// Targeted development environment name
        #[arg(value_name = "NAME")]
        name: String,

        /// Targeted development environment name
        #[arg(value_name = "CMD")]
        cmd: String,
    },

    /// Use a targeted development environment
    #[command(arg_required_else_help = true)]
    Use {
        /// Targeted development environment name
        #[arg(value_name = "NAME")]
        name: String
    },
}
