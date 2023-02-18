use nix::libc::dup2;
use pancurses::{cbreak, echo, half_delay, initscr, noecho, Input, Window};
use std::{cmp::min, fs::File, os::unix::prelude::FromRawFd};

pub struct Picker {
    picks: Vec<String>,
    input: String,
    window: Window,
    finished: bool,
    selection: usize,
}

impl Picker {
    pub fn new(picks: Vec<String>) -> Picker {
        unsafe { dup2(1, 0) };
        let window = initscr();
        noecho();
        return Picker {
            picks,
            input: String::new(),
            window,
            finished: false,
            selection: 0,
        };
    }

    pub fn insert_pick(&mut self, pick: String) {
        self.picks.push(pick);
    }

    pub fn render(&mut self) {
        self.window.clear();
        for i in 0..(min(10, self.picks.len())) {
            let pick = self.picks.get(i).expect("wtf");
            self.window.printw(" ");
            self.window.printw(pick);
            self.window.printw("\n");
        }
        self.window.printw("\n > ");
        self.window.printw(&self.input);
        self.window.refresh();
    }

    pub fn read_char(&mut self) {
        match self.window.getch() {
            Some(Input::Character(x)) => {
                match x {
                    '\u{7f}' => {
                        self.input.pop();
                        self.render();
                    }
                    '\n' => {
                        self.window.printw("enter");
                        self.finished = true;
                        self.render();
                    }
                    _ => {
                        self.input.push(x);
                        self.render();
                    }
                };
            }
            None => {}
            _ => {}
        }
    }

    pub fn finished(&self) -> bool {
        return self.finished;
    }

    pub fn get_selection(&self) -> &String {
        return self
            .picks
            .get(self.picks.len() - self.selection)
            .expect("Invalid selection");
    }
}
