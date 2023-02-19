use std::{cmp::min, ffi::CString};

use ncurses::{
    endwin, getch, scrollok, stdscr, KEY_BACKSPACE, KEY_CTAB, KEY_DOWN, KEY_ENTER, KEY_UP,
};

use ncurses::{addstr, clear, keypad, newterm, noecho, refresh, set_term};

use super::algo::sort_by_score;

const ELEMS_TO_DISPLAY: i32 = 30;

#[derive(Debug)]
pub struct Picker {
    pub picks: Vec<Pick>,
    pub input: String,
    finished: bool,
    selection: usize,
}

#[derive(Debug)]
pub struct Pick {
    pub element: String,
    pub score: usize,
}

impl Pick {
    pub fn new(element: String) -> Pick {
        Pick { element, score: 0 }
    }
}

impl Picker {
    pub fn new(picks: Vec<String>) -> Picker {
        let read = CString::new("r+").unwrap();
        let stdin_str = CString::new("/dev/tty").unwrap();
        unsafe {
            let stdin = libc::fopen(stdin_str.as_ptr(), read.as_ptr());
            let window_ptr = newterm(None, stdin, stdin);
            set_term(window_ptr);
            keypad(stdscr(), true);
            scrollok(stdscr(), true);
        };
        refresh();

        noecho();

        return Picker {
            picks: picks.iter().map(|x| Pick::new(x.to_owned())).collect(),
            input: String::new(),
            finished: false,
            selection: 0,
        };
    }

    pub fn render(&mut self) {
        sort_by_score(self);
        // clear();
        let height = min(ELEMS_TO_DISPLAY as usize, self.picks.len());
        for i in 0..height {
            let pick = self.picks.get(height - i - 1).unwrap();
            if self.selection == (height - i - 1) {
                addstr(format!("{} > ", pick.score).as_str());
            } else {
                addstr(format!("{}   ", pick.score).as_str());
            }
            addstr(pick.element.as_str());
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
            // Alt
            27 => match getch() {
                // j
                106 => {
                    if self.selection > 0 {
                        self.selection -= 1;
                    }
                    self.render();
                }
                // k
                107 => {
                    let height = min(ELEMS_TO_DISPLAY as usize, self.picks.len());
                    self.selection = min(height, self.selection + 1);
                    self.render();
                }
                _ => {}
            },
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
        return &self
            .picks
            .get(self.selection)
            .expect("Invalid selection")
            .element;
    }
}
