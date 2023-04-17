# clp

Simple library for creating "command line presentations".

![`clp_presentation` example video](https://user-images.githubusercontent.com/35977727/232160088-d7951189-af00-4eab-a912-be090da3e243.mp4)

Presentations are composed with the [`slide`](https://docs.rs/clp/latest/clp/macro.slide.html) macro, like so:

```rust
use clp::{crossterm, slide, TypewriterPrint, TypewriterPrintStyledContent};
use crossterm::style::{Print, Stylize};
use std::time::Duration;

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
    "\n...there isn't much content on these slides.",
    Duration::from_millis(25),
))
.expect("the second slide should appear");
```

## Features

This package has one feature: `spin_sleep`. It enables the [`spin_sleep`](https://docs.rs/spin_sleep/) dependency,
which is a more accurate drop-in replacement for the [`sleep`](https://doc.rust-lang.org/1.68.2/std/thread/fn.sleep.html) function.
It's particularly useful on Windows, which has a notoriously inaccurate `sleep` function.
If you notice that [`TypewriterPrint`](https://docs.rs/clp/latest/clp/struct.TypewriterPrint.html)
or [`TypewriterPrintStyledContent`](https://docs.rs/clp/latest/clp/struct.TypewriterPrintStyledContent.html)
is slower than expected,
you should enable the `spin_sleep` feature.

```bash
cargo add clp -F spin_sleep
```
