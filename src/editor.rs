use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout()
            .into_raw_mode()
            .expect("Something went wrong while entering raw_mode");

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?}\r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }
                    }
                    Key::Ctrl('t') => break,
                    _ => println!("{:?}\r", key),
                },
                Err(err) => die(&err),
            }
        }
    }

    pub fn default() -> Self {
        Self {}
    }
}

fn die(e: &std::io::Error) {
    panic!("{e}");
}
