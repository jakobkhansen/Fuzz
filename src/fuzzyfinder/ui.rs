use std::{cmp::min, ffi::CString};

use ncurses::{endwin, getch, stdscr, KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_UP};

use ncurses::{addstr, clear, keypad, newterm, noecho, refresh, set_term};

const ELEMS_TO_DISPLAY: i32 = 20;

pub struct Picker {
    picks: Vec<String>,
    input: String,
    finished: bool,
    selection: usize,
}

impl Picker {
    pub fn new(picks: Vec<String>) -> Picker {
        let read = CString::new("r+").unwrap();
        let stdin_str = CString::new("/dev/tty").unwrap();
        let stderr_str = CString::new("/dev/tty").unwrap();
        unsafe {
            let stdin = libc::fopen(stdin_str.as_ptr(), read.as_ptr());
            let stderr = libc::fopen(stderr_str.as_ptr(), read.as_ptr());
            let window_ptr = newterm(None, stdin, stderr);
            set_term(window_ptr);
            keypad(stdscr(), true);
        };
        refresh();

        noecho();

        return Picker {
            picks,
            input: String::new(),
            finished: false,
            selection: 0,
        };
    }

    pub fn insert_pick(&mut self, pick: String) {
        self.picks.push(pick);
    }

    pub fn render(&mut self) {
        clear();
        let height = min(ELEMS_TO_DISPLAY as usize, self.picks.len());
        for i in 0..height {
            let pick = self.picks.get(i).expect("wtf");
            if self.selection == (height - i - 1) {
                addstr("> ");
            } else {
                addstr("  ");
            }
            addstr(pick);
            addstr("\n");
        }
        addstr("\n> ");
        addstr(&self.input);
        refresh();
    }

    pub fn read_char(&mut self) {
        match getch() {
            KEY_BACKSPACE | 127 => {
                self.input.pop();
                self.render();
            }
            KEY_ENTER | 13 | 10 => {
                self.finished = true;
                clear();
                endwin();
                self.render();
            }
            KEY_UP => {
                let height = min(ELEMS_TO_DISPLAY as usize, self.picks.len());
                self.selection = min(height, self.selection + 1);
                self.render();
            }
            KEY_DOWN => {
                if self.selection > 0 {
                    self.selection -= 1;
                }
                self.render();
            }
            other => {
                addstr(format!("{}", other).as_str());
                let char = other as u8;
                self.input.push(char as char);
                self.render();
            }
        };
    }

    pub fn finished(&self) -> bool {
        return self.finished;
    }

    pub fn get_selection(&self) -> &String {
        let height = min(ELEMS_TO_DISPLAY as usize, self.picks.len());
        return self
            .picks
            .get(height - self.selection - 1)
            .expect("Invalid selection");
    }
}
