use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn to_words<'a>(text: &'a str) -> impl Iterator<Item = &'a str> {
    text.split(' ')
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let brew_commands: [&str; 7] = [
        "update",
        "upgrade",
        "cleanup",
        "cask cleanup",
        "doctor",
        "missing",
        "prune",
    ];

    let bar = ProgressBar::new(8);
    bar.set_style(ProgressStyle::default_bar().template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
    ));

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(253, 117, 50))))?;
    print!("Executing Brew ");
    for command in bar.wrap_iter(brew_commands.iter()) {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(253, 117, 50))))?;
        println!("{:?}", command);
        let output = Command::new("Brew")
            .args(to_words(command))
            .stderr(Stdio::null())
            .output()
            .expect("Brew command failed");

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(30, 151, 70))))?;
        io::stdout().write_all(&output.stdout).unwrap();
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(207, 0, 15))))?;
        io::stderr().write_all(&output.stderr).unwrap();
    }
    bar.finish_and_clear();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(99, 192, 223))))?;
    println!("Done in {}", HumanDuration(started.elapsed()));
    Ok(())
}
