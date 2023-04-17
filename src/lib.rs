//! Simple library for creating "command line presentations".
//!
//! ![`clp_presentation` example video](https://user-images.githubusercontent.com/35977727/232160088-d7951189-af00-4eab-a912-be090da3e243.mp4)
//!
//! Presentations are composed with the [`slide`] macro, like so:
//!
//! ```no_run
//! use clp::{crossterm, slide, TypewriterPrint, TypewriterPrintStyledContent};
//! use crossterm::style::{Print, Stylize};
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
//! This package has one feature: `spin_sleep`. It enables the [`spin_sleep`](https://docs.rs/spin_sleep/) dependency,
//! which is a more accurate drop-in replacement for the [`sleep`] function.
//! It's particularly useful on Windows, which has a notoriously inaccurate `sleep` function.
//! If you notice that [`TypewriterPrint`] or [`TypewriterPrintStyledContent`] is slower than expected,
//! you should enable the `spin_sleep` feature.
//!
//! ```bash
//! cargo add clp -F spin_sleep
//! ```

#![warn(missing_docs)]

pub use crossterm;

use crossterm::event::{self, Event, KeyCode};
use crossterm::style::{PrintStyledContent, StyledContent, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled};
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
/// use clp::{crossterm, slide, TypewriterPrint, TypewriterPrintStyledContent};
/// use crossterm::style::{Print, Stylize};
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
    ($($command:expr),* $(,)?) => {{
        use $crate::crossterm::execute;
        use $crate::crossterm::terminal::{Clear, ClearType};
        use $crate::WaitForInteraction;
        use std::io::stdout;

        execute!(stdout(), Clear(ClearType::All), $($command,)* WaitForInteraction)
    }}
}

/// A command that prints the given displayable type, one character at a time.
///
/// # Examples
///
/// ```no_run
/// use clp::{slide, TypewriterPrint};
/// use std::time::Duration;
///
/// slide!(TypewriterPrint("Hello, world!", Duration::from_millis(25)))
///     .expect("each character of \"Hello, world!\" should be printed in 25ms intervals");
/// ```
///
/// # Notes
///
/// Commands must be executed/queued for execution
/// (which [`TypewriterPrint`] is when in [`slide`])
/// otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypewriterPrint<T: Display>(pub T, pub Duration);

impl<T: Display> Command for TypewriterPrint<T> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        for char in self.0.to_string().chars() {
            f.write_char(char)?;
            stdout()
                .flush()
                .expect("standard output stream should flush");

            if !is_raw_mode_enabled().expect("should check if raw mode is enabled") {
                enable_raw_mode().expect("raw mode should enable");
            }

            sleep(self.1);

            if is_raw_mode_enabled().expect("should check if raw mode is enabled") {
                disable_raw_mode().expect("raw mode should disable");
            }
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
/// # Examples
///
/// ```no_run
/// use clp::{crossterm, slide, TypewriterPrintStyledContent};
/// use crossterm::style::Stylize;
/// use std::time::Duration;
///
/// slide!(TypewriterPrintStyledContent(
///     "Hello, world!".bold(),
///     Duration::from_millis(25)
/// ))
/// .expect("each character of \"Hello, world!\" should be printed in 25ms intervals");
/// ```
///
/// # Notes
///
/// Commands must be executed/queued for execution
/// (which [`TypewriterPrintStyledContent`] is when in [`slide`])
/// otherwise they do nothing.
#[derive(Debug, Clone, Copy)]
pub struct TypewriterPrintStyledContent<D: Display>(pub StyledContent<D>, pub Duration);

impl<D: Display> Command for TypewriterPrintStyledContent<D> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        for char in self.0.to_string().chars() {
            PrintStyledContent(char.stylize()).write_ansi(f)?;
            stdout()
                .flush()
                .expect("standard output stream should flush");

            if !is_raw_mode_enabled().expect("should check if raw mode is enabled") {
                enable_raw_mode().expect("raw mode should enable");
            }

            sleep(self.1);

            if is_raw_mode_enabled().expect("should check if raw mode is enabled") {
                disable_raw_mode().expect("raw mode should disable");
            }
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

/// A command that waits for user interaction before executing subsequent commands.
///
/// # Examples
///
/// ```no_run
/// use clp::{crossterm, slide};
/// use crossterm::style::Print;
///
/// slide!(
///     Print("This will appear immediately.\n"),
///     WaitForInteraction, // <- This command is used within the macro, so it does not need to be binded again
///     Print("This will appear after an interaction."),
/// )
/// .expect("one message should print, then the other should print after an interaction");
/// ```
///
/// # Notes
///
/// Commands must be executed/queued for execution
/// (which [`TypewriterPrint`] is when in [`slide`])
/// otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitForInteraction;

impl Command for WaitForInteraction {
    fn write_ansi(&self, _f: &mut impl fmt::Write) -> fmt::Result {
        stdout()
            .flush()
            .expect("standard output stream should flush");

        if !is_raw_mode_enabled().expect("should check if raw mode is enabled") {
            enable_raw_mode().expect("raw mode should enable");
        }

        loop {
            if let Event::Key(key) = event::read().expect("should read event") {
                if let KeyCode::Enter | KeyCode::Right | KeyCode::Char(' ') = key.code {
                    break;
                }
            }
        }

        if is_raw_mode_enabled().expect("should check if raw mode is enabled") {
            disable_raw_mode().expect("raw mode should disable");
        }

        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> crossterm::Result<()> {
        Ok(())
    }
}

/// A command that waits for the specified duration before executing subsequent commands.
///
/// # Examples
///
/// ```no_run
/// use clp::{crossterm, slide, WaitFor};
/// use crossterm::style::Print;
/// use std::time::Duration;
///
/// slide!(
///     Print("This will appear immediately.\n"),
///     WaitFor(Duration::from_secs(5)),
///     Print("This will appear after 5 seconds."),
/// )
/// .unwrap();
/// ```
///
/// # Notes
///
/// Commands must be executed/queued for execution
/// (which [`TypewriterPrint`] is when in [`slide`])
/// otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitFor(pub Duration);

impl Command for WaitFor {
    fn write_ansi(&self, _f: &mut impl fmt::Write) -> fmt::Result {
        stdout()
            .flush()
            .expect("standard output stream should flush");

        if !is_raw_mode_enabled().expect("should check if raw mode is enabled") {
            enable_raw_mode().expect("raw mode should enable");
        }

        sleep(self.0);

        if is_raw_mode_enabled().expect("should check if raw mode is enabled") {
            disable_raw_mode().expect("raw mode should disable");
        }

        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> crossterm::Result<()> {
        Ok(())
    }
}
