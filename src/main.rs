mod cli;
mod cmd;
mod data;
mod errors;

use crate::data::NixFlakeData;
use crate::cli::{Cli, Commands};
use crate::cmd::NixFlake;
use crate::errors::FlakePathError;

use clap::Parser;

use serde_json;

use std::env;
use std::path::PathBuf;

fn get_flake_pathdir(args: &Cli) -> Result<PathBuf, FlakePathError> {
    let flake_envpath = env::var("DV_FLAKE_DIR");

    if flake_envpath.is_err() && args.path.is_none() {
        return Err(FlakePathError::Empty);
    }

    let path = if flake_envpath.is_ok() {
        PathBuf::from(flake_envpath.unwrap())
    } else {
        args.path.clone().unwrap()
    };

    if ! path.exists() {
        return Err(FlakePathError::NotFound(path));
    }

    Ok(path)
}

fn main() {
    let archi = "x86_64-linux";

    let args = Cli::parse();

    let path = match get_flake_pathdir(&args) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let path_str = path.display().to_string();

    match args.command {
        Commands::Use { name } => {
            let flake = NixFlake::new(&archi, &path_str, &name);
            let json_data = flake.to_json();
            let data : NixFlakeData = serde_json::from_str(&json_data).unwrap();

            if data.shell_exists(&archi, &name) {
                flake.spawn_shell();
            }
        },
        Commands::List => {
            let flake = NixFlake::new(&archi, &path_str, "");
            let json_data = flake.to_json();
            let data : NixFlakeData = serde_json::from_str(&json_data).unwrap();
            data.print_shells(&archi);
        }
    }

}
