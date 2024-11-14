use clap::{Parser, Subcommand};

use serde::{Serialize, Deserialize};
use serde_json;

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShellInfo {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NixFlakeData {
    #[serde(flatten)]
    pub devshells: HashMap<String, HashMap<String, HashMap<String, ShellInfo>>> // Dirty hack
}

impl NixFlakeData {

    pub fn shell_exists(&self, archi: &str, shell_name: &str) -> bool {
        let devshells = self.devshells.get("devShells").unwrap(); // safe: used on unserialized data

        if let Some(shell) = devshells.get(archi) {
            if let Some(_shellinfo) = shell.get(shell_name) {
                true
            } else {
                false
            }
        } else {
            false
        }

    }

    pub fn print_shells(&self, archi: &str) {
        let devshells = self.devshells.get("devShells").unwrap(); // safe: used on unserialized data

        if let Some(shell) = devshells.get(archi) {
            for sh in shell.keys() {
                println!("{sh}");
            }
        } else {
            println!("No devshells found for {archi}");
        }
    }

}

#[derive(Debug, Clone)]
pub struct NixFlake {
    pub archi: String,
    pub path: String,
    pub name: String,
}

impl NixFlake {

    pub fn new(archi: &str, path: &str, name: &str) -> Self {
        Self {
            archi: archi.to_string(),
            path: path.to_string(),
            name: name.to_string()
        }
    }

    pub fn to_json(&self) -> String {
        let path = format!("path:{}", self.path);

        let output = Command::new("nix")
            .arg("flake")
            .arg("show")
            .arg(path)
            .arg("--json")
            .output()
            .expect("Could not retrieve devshells json metadata.");

        String::from_utf8(output.stdout).expect("Could not convert stdout to sting type.")
    }

    pub fn spawn_shell(&self) {
        let path = format!("{}#{}", self.path, self.name);
        let mut cmd = Command::new("nix");

        if let Ok(mut child) = cmd.arg("develop").arg(&path).spawn() {
            child.wait().expect("command wasn't running");
            println!("Nix DevShell: Bye! Leaving {}", path);
        } else {
            println!("Nix DevShell didn't start");
        }
    }

}

#[derive(Debug, Parser)]
#[command(name = "dvenv")]
#[command(about = "CLI for poping into Nix shell", long_about = None)]
struct Cli {

    /// Directory containing the flake.nix file (default: DVENV_FLAKE_DIR shell variable)
    #[arg(short, long, value_name = "DIR")]
    path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List available environments
    List,

    /// Use a targeted development environment
    #[command(arg_required_else_help = true)]
    Use {
        /// Targeted development environment name
        #[arg(value_name = "NAME")]
        name: String
    },
}

fn main() {
    let archi = "x86_64-linux";

    let args = Cli::parse();

    let flake_envpath = env::var("DVENV_FLAKE_DIR");
    let path = if flake_envpath.is_ok() {
        PathBuf::from(flake_envpath.unwrap())
    } else {
        args.path.unwrap_or_else(|| {
            eprintln!("Empty Flake path. Use --path or DVEN_FLAKE_DIR environment variable");
            std::process::exit(1);
        })
    };

    if ! path.exists() {
        eprintln!("Flake path {:?} not found", path);
        std::process::exit(1);
    }

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
