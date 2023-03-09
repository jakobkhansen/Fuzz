use std::{cmp::min, ffi::CString};

use ncurses::ll::{attron, initscr, set_escdelay};
use ncurses::{
    addnstr, assume_default_colors, attroff, endwin, getch, init_pair, mvaddstr, mvwaddstr,
    nodelay, scrollok, start_color, stdscr, COLOR_BLUE, COLOR_CYAN, COLOR_PAIR, COLOR_RED,
    COLOR_WHITE, KEY_BACKSPACE, KEY_CTAB, KEY_DOWN, KEY_ENTER, KEY_UP, LINES,
};

use ncurses::{addstr, clear, keypad, newterm, noecho, refresh, set_term};

use super::algo::sort_by_score;

const ELEMS_TO_DISPLAY: i32 = i32::MAX;

const CURSOR_PAIR: i16 = 1;

#[derive(Debug)]
pub struct Picker {
    window_ptr: *mut i8,
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
            set_escdelay(100);
            keypad(stdscr(), true);
            scrollok(stdscr(), true);
            refresh();
            start_color();
            noecho();

            init_pair(CURSOR_PAIR, COLOR_CYAN, 0);

            let mut picker = Picker {
                picks: picks.iter().map(|x| Pick::new(x.to_owned())).collect(),
                input: String::new(),
                finished: false,
                selection: 0,
                window_ptr,
            };

            sort_by_score(&mut picker);
            return picker;
        };
    }

    pub fn render(&mut self) {
        clear();
        let height: u32 = (ELEMS_TO_DISPLAY as usize)
            .min(LINES() as usize)
            .min(self.picks.len())
            .try_into()
            .unwrap();

        mvaddstr(LINES() - 1, 0, "\n> ");
        addnstr(&self.input, LINES());
        for i in 0..height as usize {
            let index = i as i32;
            let pick = self.picks.get(i).unwrap();
            if self.selection == (i) {
                unsafe {
                    attron(COLOR_PAIR(CURSOR_PAIR));
                };
                mvaddstr((LINES()) - (index) - 2, 0, format!("> ").as_str());
            } else {
                mvaddstr((LINES()) - (index) - 2, 0, format!("  ").as_str());
            }
            addstr(pick.element.as_str());
            attroff(COLOR_PAIR(CURSOR_PAIR));
            addstr("\n");
        }
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        refresh();
    }

    pub fn read_char(&mut self) {
        match getch() {
            KEY_BACKSPACE | 127 => {
                if self.input.len() == 0 {
                    return;
                }
                self.input.pop();
                sort_by_score(self);
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
            27 => {
                nodelay(stdscr(), true);
                match getch() {
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
                    -1 => {
                        self.finished = true;
                        clear();
                        endwin();
                    }
                    x => {
                        addstr(format!("{}", x).as_str());
                    }
                }
                nodelay(stdscr(), false);
            }
            other => {
                addstr(format!("{}", other).as_str());
                let char = other as u8;
                self.input.push(char as char);
                sort_by_score(self);
                self.render();
            }
        };
    }

    pub fn finished(&self) -> bool {
        return self.finished;
    }

    pub fn get_selection(&self) -> String {
        match self.picks.get(self.selection) {
            Some(pick) => pick.element.to_owned(),
            None => "".to_string(),
        }
    }
}
