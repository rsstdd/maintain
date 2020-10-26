use std::io::{self, Write};
use std::process::{Command, Stdio};

fn to_words<'a>(text: &'a str) -> impl Iterator<Item = &'a str> {
    text.split(' ')
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let brew_commands: [&str; 7] = [
        "update",
        "upgrade",
        "missing",
        "cleanup",
        "cask cleanup",
        "doctor",
        "prune",
    ];

    println!("executing brew ");

    for command in brew_commands.iter() {
        let output = Command::new("brew")
            .args(to_words(command))
            .stderr(Stdio::null())
            .output()
            .expect("brew command failed");

        io::stderr().write_all(&output.stderr).unwrap();
    }

    Ok(())
}
