use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
