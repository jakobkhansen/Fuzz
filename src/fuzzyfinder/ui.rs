use std::{cmp::min, ffi::CString};

use ncurses::ll::{attron, set_escdelay};
use ncurses::{
    addnstr, attroff, curs_set, endwin, getch, init_pair, mvaddstr, nodelay, scrollok, start_color,
    stdscr, COLOR_CYAN, COLOR_PAIR, KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_UP, LINES,
};

use ncurses::{addstr, clear, keypad, newterm, noecho, refresh, set_term};

use super::algo::sort_by_score;

const ELEMS_TO_DISPLAY: i32 = i32::MAX;

const CURSOR_PAIR: i16 = 1;

#[derive(Debug)]
pub enum PickerState {
    RUNNING,
    FINISHED,
    ABORTED,
}
#[derive(Debug)]
pub struct Picker {
    pub picks: Vec<Pick>,
    pub input: String,
    state: PickerState,
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
            curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

            init_pair(CURSOR_PAIR, COLOR_CYAN, 0);

            let mut picker = Picker {
                picks: picks.iter().map(|x| Pick::new(x.to_owned())).collect(),
                input: String::new(),
                state: PickerState::RUNNING,
                selection: 0,
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
                mvaddstr(
                    (LINES()) - (index) - 2,
                    0,
                    format!("> {}\n", pick.element).as_str(),
                );
                attroff(COLOR_PAIR(CURSOR_PAIR));
            } else {
                mvaddstr(
                    (LINES()) - (index) - 2,
                    0,
                    format!("  {}\n", pick.element).as_str(),
                );
            }
        }
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
                self.state = PickerState::FINISHED;
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
                        self.state = PickerState::ABORTED;
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
        match self.state {
            PickerState::RUNNING => false,
            PickerState::FINISHED => true,
            PickerState::ABORTED => true,
        }
    }

    pub fn get_selection(&self) -> Option<String> {
        match self.state {
            PickerState::ABORTED => None,
            _ => match self.picks.get(self.selection) {
                Some(pick) => Some(pick.element.to_owned()),
                None => None,
            },
        }
    }
}
