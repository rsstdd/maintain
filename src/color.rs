use std::io::{self, Write};
use std::error::Error;
use std::str::FromStr;

// pub static COLOR_STATUS: &Color = &Color::Term(TermColor::Green);
// pub static COLOR_ERROR: &Color = &Color::Term(TermColor::Red);
// pub static COLOR_INFO: &Color = &Color::Term(TermColor::Cyan);
// pub static COLOR_WARN: &Color = &Color::Term(TermColor::Yellow);
// pub static COLOR_VERBOSE: &Color = &Color::Term(TermColor::Blue);

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod color {
  pub fn write_color(color: <> ) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::color)))?;
    io::stdout().write_all(&output.stdout).unwrap();
  }
}
