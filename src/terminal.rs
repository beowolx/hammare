use crate::Position;
use std::io::{self, stdout, Write};
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
        let size = termion::terminal_size()
            .expect("Something unexpected happened while trying to get terminal's size.");
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout()
                .into_raw_mode()
                .expect("Something unexpected happening while entering raw mode."),
        })
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
    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
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
