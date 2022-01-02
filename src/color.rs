use crate::status::Status;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn write_status(
    status: &Status,
    status_as_string: &str,
    status_width: usize,
) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(match status {
        Status::Bare => Color::Red,
        Status::Clean => Color::Green,
        _ => Color::Yellow,
    })))?;
    write!(
        &mut stdout,
        "{:<status_width$}",
        status_as_string,
        status_width = status_width,
    )?;
    stdout.reset()
}

pub fn write_bold(input: &str, newline: bool) -> io::Result<()> {
    write_color(input, newline, ColorSpec::new().set_bold(true))
}

pub fn write_gray(input: &str, newline: bool) -> io::Result<()> {
    write_color(
        input,
        newline,
        ColorSpec::new().set_fg(Some(Color::Rgb(128, 128, 128))),
    )
}

fn write_color(input: &str, newline: bool, color_spec: &mut ColorSpec) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(color_spec)?;
    match newline {
        true => writeln!(&mut stdout, "{}", input)?,
        false => write!(&mut stdout, "{}", input)?,
    }
    stdout.reset()
}
