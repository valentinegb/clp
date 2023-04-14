//! Simple library for creating "command line presentations".
//!
//! Presentations are composed with the [`slide`] macro, like so:
//!
//! ```no_run
//! use clp::crossterm::style::{Print, Stylize};
//! use clp::{slide, TypewriterPrint, TypewriterPrintStyledContent};
//! use std::time::Duration;
//!
//! slide!(
//!     TypewriterPrint("Welcome to my presentation on ", Duration::from_millis(25)),
//!     TypewriterPrintStyledContent(
//!         "command line presentations".bold(),
//!         Duration::from_millis(50),
//!     ),
//!     Print("."),
//! )
//! .expect("the first slide should appear");
//!
//! slide!(TypewriterPrint(
//!     "...there isn't much content on these slides.",
//!     Duration::from_millis(25),
//! ))
//! .expect("the second slide should appear");
//! ```
//!
//! # Features
//!
//! This package has one feature: `spin_sleep`. It enables the `spin_sleep` dependency,
//! which is a more accurate drop-in replacement for the [`sleep`] function.
//! It's particularly useful on Windows, which has a notoriously inaccurate `sleep` function.
//! If you notice that [`TypewriterPrint`] or [`TypewriterPrintStyledContent`] is slower than expected,
//! you should enable the `spin_sleep` feature.

#![warn(missing_docs)]

pub use crossterm;

use crossterm::style::{PrintStyledContent, StyledContent, Stylize};
use crossterm::Command;
#[cfg(feature = "spin_sleep")]
use spin_sleep::sleep;
use std::fmt::{self, Display, Formatter};
use std::io::{stdout, Write as _};
#[cfg(not(feature = "spin_sleep"))]
use std::thread::sleep;
use std::time::Duration;

/// Defines a slide and shows it.
///
/// Takes any number of [`crossterm::Command`]s as arguments.
///
/// # Examples
///
/// ```no_run
/// use clp::crossterm::style::{Print, Stylize};
/// use clp::{slide, TypewriterPrint, TypewriterPrintStyledContent};
/// use std::time::Duration;
///
/// slide!(
///     TypewriterPrint("Welcome to my presentation on ", Duration::from_millis(25)),
///     TypewriterPrintStyledContent(
///         "command line presentations".bold(),
///         Duration::from_millis(50),
///     ),
///     Print("."),
/// )
/// .expect("the first slide should appear");
///
/// slide!(TypewriterPrint(
///     "...there isn't much content on these slides.",
///     Duration::from_millis(25),
/// ))
/// .expect("the second slide should appear");
/// ```
#[macro_export]
macro_rules! slide {
    ($($command:tt)*) => {{
        use clp::crossterm::execute;
        use clp::crossterm::terminal::{Clear, ClearType};
        use clp::crossterm::event::{read, Event, KeyCode};
        use clp::crossterm::ErrorKind;
        use std::io::stdout;

        let mut result = Ok(());

        match execute!(stdout(), Clear(ClearType::All), $($command)*) {
            Ok(_) => loop {
                match read() {
                    Ok(event) => if let Event::Key(key) = event {
                        if let KeyCode::Enter | KeyCode::Right | KeyCode::Char(' ') = key.code {
                            break;
                        }
                    },
                    Err(error) => result = Err(error),
                }
            },
            Err(error) => result = Err(error),
        }

        result
    }}
}

/// A command that prints the given displayable type, one character at a time.
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypewriterPrint<T: Display>(pub T, pub Duration);

impl<T: Display> Command for TypewriterPrint<T> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        for char in self.0.to_string().chars() {
            f.write_char(char)?;
            stdout()
                .flush()
                .expect("standard output stream should flush");
            sleep(self.1);
        }

        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> crossterm::Result<()> {
        panic!("tried to execute Print command using WinAPI, use ANSI instead");
    }

    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        true
    }
}

impl<T: Display> Display for TypewriterPrint<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A command that prints styled content, one character at a time.
///
/// See [`StyledContent`] for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy)]
pub struct TypewriterPrintStyledContent<D: Display>(pub StyledContent<D>, pub Duration);

impl<D: Display> Command for TypewriterPrintStyledContent<D> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        for char in self.0.to_string().chars() {
            PrintStyledContent(char.stylize()).write_ansi(f)?;
            stdout()
                .flush()
                .expect("standard output stream should flush");
            sleep(self.1);
        }

        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> crossterm::Result<()> {
        Ok(())
    }
}

impl Display for TypewriterPrintStyledContent<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        PrintStyledContent(self.0.clone()).fmt(f)
    }
}

impl Display for TypewriterPrintStyledContent<&'static str> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        PrintStyledContent(self.0.clone()).fmt(f)
    }
}
