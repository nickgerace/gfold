//! This module provides a harness for non-trivial displays of information to `stdout`.

use libgfold::Status;
use std::io;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::config::ColorMode;

/// This harness provides methods to write to `stdout`. It maps the internal [`ColorMode`] type to
/// our dependency's [`ColorChoice`] type due to discrepancies in behavior and naming.
pub struct ColorHarness {
    color_choice: ColorChoice,
}

impl ColorHarness {
    pub fn new(color_mode: &ColorMode) -> Self {
        Self {
            color_choice: match &color_mode {
                ColorMode::Always => ColorChoice::Always,
                ColorMode::Compatibility => ColorChoice::Auto,
                ColorMode::Never => ColorChoice::Never,
            },
        }
    }

    /// Writes the [`Status`] of the Git repository to `stdout`.
    pub fn write_status(&self, status: &Status, status_width: usize) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(self.color_choice);
        stdout.set_color(ColorSpec::new().set_fg(Some(match status {
            Status::Bare | Status::Unknown => Color::Red,
            Status::Clean => Color::Green,
            Status::Unpushed => Color::Blue,
            Status::Unclean => Color::Yellow,
        })))?;
        write!(
            &mut stdout,
            "{:<status_width$}",
            status.as_str(),
            status_width = status_width,
        )?;
        stdout.reset()
    }

    /// Writes the input [`&str`] to `stdout` in bold.
    pub fn write_bold(&self, input: &str, newline: bool) -> io::Result<()> {
        self.write_color(input, newline, ColorSpec::new().set_bold(true))
    }

    /// Writes the input [`&str`] to `stdout` in gray (or cyan if in compatibility mode).
    pub fn write_gray(&self, input: &str, newline: bool) -> io::Result<()> {
        // FIXME(nick): check why Color::Rg(128, 128, 128) breaks in tmux on macOS Terminal.app.
        self.write_color(
            input,
            newline,
            ColorSpec::new().set_fg(Some(match &self.color_choice {
                ColorChoice::Auto => Color::Cyan,
                _ => Color::Ansi256(242),
            })),
        )
    }

    fn write_color(
        &self,
        input: &str,
        newline: bool,
        color_spec: &mut ColorSpec,
    ) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(self.color_choice);
        stdout.set_color(color_spec)?;
        match newline {
            true => writeln!(&mut stdout, "{input}")?,
            false => write!(&mut stdout, "{input}")?,
        }
        stdout.reset()
    }
}
