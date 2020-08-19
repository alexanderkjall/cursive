#![no_main]
use libfuzzer_sys::fuzz_target;

use cursive::views::{Dialog, TextView};
use std::str;

use std::thread;
use std::time::Duration;

fuzz_target!(|data: &[u8]| {
    let str = str::from_utf8(data);
    if let Ok(s) = str {
        // Creates the cursive root - required for every application.
        let mut siv = cursive::default();

        // Creates a dialog with a single "Quit" button
        siv.add_layer(Dialog::around(TextView::new(s))
                             .title("Cursive")
                             .button("Quit", |s| s.quit()));

        siv.step();
    }
});
