use std::io::{self, Write};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Command::new("brew update");
    let brew_commands = [
        "update",
        "upgrade",
        "cleanup",
        "cask cleanup",
        "cask cleanup",
        "doctor",
        "missing",
        "prune",
    ];

    for command in brew_commands.iter() {
        Command::new("brew {}", command);
        let output = Command::new("brew cleanup")
            .output()
            .expect("failed to execute process");
    }

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
    Ok(())
}
