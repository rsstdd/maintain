use indicatif::{ProgressBar, ProgressStyle};
use std::process::{Command, Stdio};

#[macro_use]
extern crate log;

use env_logger::Env;

fn to_words<'a>(text: &'a str) -> impl Iterator<Item = &'a str> {
    text.split(' ')
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(
        Env::default()
            .filter_or("MY_LOG_LEVEL", "info")
            .write_style_or("MY_LOG_STYLE", "always"),
    );

    let linux_commands: [&str; 5] = [
        "update upgrade -y",
        "dist-upgrade -y",
        "dist-upgrade -y",
        "autoremove -y",
        "autoclean -y",
    ];
    let brew_commands: [&str; 7] = [
        "update",
        "upgrade",
        "missing",
        "cleanup",
        "cask cleanup",
        "doctor",
        "prune",
    ];

    println!("\n");
    println!("\n");
    info!("Executing Brew...");
    println!("\n");
    println!("\n");

    let pb_style = ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}), ETA {eta}",
    );

    let pb = ProgressBar::new(7);
    pb.set_style(pb_style.clone());
    for command in brew_commands.iter() {
        let output = Command::new("brew")
            .args(to_words(command))
            .stderr(Stdio::null())
            .output()
            .expect("brew command failed");

        pb.println(format!("[+] {}", command));
        pb.inc(1);
        info!("{:?}", output);
    }
    pb.finish_with_message("Complete Brew Commands");

    let pb_linux = ProgressBar::new(5);
    pb_linux.set_style(pb_style.clone());
    for command in linux_commands.iter() {
        let command_output = Command::new("apt-get")
            .args(to_words(command))
            .status()
            .unwrap()
            .success();

        pb_linux.println(format!("[+] {}", command));
        pb_linux.inc(1);
        info!("{:?}", command_output);
    }

    Ok(())
}
