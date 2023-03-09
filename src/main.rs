use std::io::stdin;

use fuzz::fuzzyfinder::ui::Picker;

fn main() {
    let mut picks: Vec<String> = vec![];

    for line in stdin().lines() {
        let line = line.expect("Error reading stdin").trim().to_string();
        picks.push(line);
    }

    let mut picker = Picker::new(picks);

    picker.render();
    while !picker.finished() {
        picker.read_char();
    }

    match picker.get_selection().as_str() {
        "" => {}
        x => println!("{}", x),
    }
    // println!("{}", picker.get_selection());
}
