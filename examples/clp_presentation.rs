use clp::crossterm::style::Stylize;
use clp::{slide, TypewriterPrint, TypewriterPrintStyledContent};
use core::num::NonZeroU32;
use figlet_rs::FIGfont;
use std::time::Duration;

fn main() {
    let roman_font = FIGfont::from_content(include_str!("../resources/roman.flf"))
        .expect("the roman figlet font should be loaded from resource");

    slide!(
        TypewriterPrint("Introducing...\n\n", Duration::from_millis(100)),
        TypewriterPrint(
            roman_font
                .convert("clp")
                .expect("\"clp\" should be converted to a figlet"),
            Duration::from_millis(10),
        ),
        TypewriterPrint(
            "A simple library for creating \"command line presentations\".\n",
            Duration::from_millis(50),
        ),
        TypewriterPrintStyledContent(
            "(Press enter to go to the next slide.)".italic(),
            Duration::from_millis(10),
        ),
    )
    .expect("should play introductory slide");

    slide!(
        TypewriterPrint(
            "
Command line presentations, like this one, are like presentations you'd make in Keynote or PowerPoint or Google Slides, \
except it all runs in a terminal!

Since this is a terminal, you can only print text, however, you can easily do something like this:\n",
            Duration::from_millis(20),
        ),
        WaitForInteraction,
        TypewriterPrint(
            artem::convert(
                image::load_from_memory(include_bytes!("../resources/valentinegb_avatar.jpeg"))
                    .expect("`valentinegb_avatar.jpeg` image should open"),
                artem::options::OptionBuilder::new()
                    .target_size(NonZeroU32::new(59).expect("non-zero number should be created"))
                    .build(),
            ),
            Duration::from_micros(10),
        ),
    )
    .expect("should play image demonstration slide");

    slide!(TypewriterPrint(
        "
In case you couldn't tell, that was ASCII art of my GitHub avatar, converted at runtime. \
In terminals that support it, it should've been colored, too!

The ASCII art was created with a library called artem, \
and the text art on the first slide was made with figlet-rs.",
        Duration::from_millis(20),
    ),)
    .expect("should play image explanation slide");
}
