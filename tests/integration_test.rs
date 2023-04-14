use clp::crossterm::style::{Print, Stylize};
use clp::{slide, TypewriterPrint, TypewriterPrintStyledContent};
use std::time::Duration;

#[test]
fn presentation() {
    slide!(
        TypewriterPrint("Welcome to my presentation on ", Duration::from_millis(25)),
        TypewriterPrintStyledContent(
            "command line presentations".bold(),
            Duration::from_millis(50),
        ),
        Print("."),
    )
    .expect("the first slide should appear");

    slide!(TypewriterPrint(
        "...there isn't much content on these slides.",
        Duration::from_millis(25),
    ))
    .expect("the second slide should appear");
}
