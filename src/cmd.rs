use std::process::Command;

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

    pub fn run_command(&self, command: &str) {
        let path = format!("{}#{}", self.path, self.name);
        let mut cmd = Command::new("nix");

        if let Ok(mut child) = cmd.args(&[
            "develop",
            &path,
            "--command",
            "bash", "-c",
            command]).spawn() {
            child.wait().expect("command wasn't running");
        }
        else {
            println!("Error: Could not run command");
        }
    }

}
