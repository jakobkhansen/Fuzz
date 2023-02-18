use std::io::{self, stdin, BufRead};

use fuzz::fuzzyfinder::ui::Picker;
use pancurses::endwin;

fn main() {
    let mut picks: Vec<String> = vec![];

    for line in stdin().lines() {
        let line = line.expect("Error reading stdin");
        picks.push(line);
    }

    let mut picker = Picker::new(picks);

    picker.render();
    while !picker.finished() {
        picker.read_char();
    }
    endwin();
}
