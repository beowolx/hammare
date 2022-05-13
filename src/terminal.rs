use crate::Position;
use std::io::{self, stdout, Error, ErrorKind, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    /// Generates a default terminal size
    ///
    /// # Errors
    /// It will return `Err` if `termion::terminal_size()`
    /// fails to get the terminal's size
    pub fn default() -> Result<Self, std::io::Error> {
        if let Ok(size) = termion::terminal_size() {
            if let Ok(raw_terminal) = stdout().into_raw_mode() {
                Ok(Self {
                    size: Size {
                        width: size.0,
                        height: size.1.saturating_sub(2),
                    },
                    _stdout: raw_terminal,
                })
            } else {
                Err(Error::new(
                    ErrorKind::Other,
                    "Something unexpected happening while trying to enter raw mode",
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Something unexpected happening while trying to get the terminal size.",
            ))
        }
    }
    #[must_use]
    /// Returns a read-only reference to the internal `size`
    pub fn size(&self) -> &Size {
        &self.size
    }

    /// Clears the terminal screen
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    /// Set the cursor position on the terminal
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = *position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x.try_into().expect("Failed to convert to u16");
        let y = y.try_into().expect("Failed to convert to u16");
        print!("{}", termion::cursor::Goto(x, y));
    }

    /// Hide the cursor
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    /// Show the cursor
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    /// Clear the current line
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    /// Sets the background color
    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    /// Resets the background color
    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    /// Sets the foreground color
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    /// Resets the foreground color
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    /// Flush this output stream, ensuring that all intermediately buffered contents reach their destination.
    ///
    /// # Errors
    /// It is considered an error if not all bytes could be written due to I/O errors or EOF being reached.
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    /// Reads the keyboard keys pressed
    ///
    /// # Errors
    /// It will fail if not possible to read the keys from the keyboard
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
