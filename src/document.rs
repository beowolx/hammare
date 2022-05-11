use crate::Position;
use crate::Row;
use crate::SearchDirection;
use std::cmp::Ordering;
use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
}

impl Document {
    /// Opens a file in the editor
    ///
    /// # Errors
    /// It will return `Err` if it fails to open the file
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_owned()),
            dirty: false,
        })
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.rows.len() {
            return;
        }
        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }

        let new_row = self.rows.get_mut(at.y).expect("Something unexpected happened while trying to get a mutable reference to the row index").split(at.x);

        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(at.y + 1, new_row);
    }

    /// Inserts a character in the document that is being read, at the position
    /// where the cursor is.
    ///
    /// # Panics
    ///
    /// It will panic if we try to insert in a position that is greater
    /// than the length of the document.
    #[allow(clippy::panic)]
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        match at.y.cmp(&self.rows.len()) {
            Ordering::Equal => {
                let mut row = Row::default();
                row.insert(0, c);
                self.rows.push(row);
            }
            Ordering::Less => {
                let row = self.rows.get_mut(at.y).expect("Something unexpected happened while trying to get a mutable reference to the row index");
                row.insert(at.x, c);
            }
            Ordering::Greater => {
                panic!("Insert characters pass the document's length is not possible.")
            }
        }
    }

    /// Deletes a single or multiple characters in the document
    #[allow(clippy::integer_arithmetic)]
    pub fn delete(&mut self, at: &Position) {
        let len = self.rows.len();
        if at.y >= len {
            return;
        }
        self.dirty = true;
        if at.x == self.rows.get_mut(at.y).expect("Something unexpected happened while trying to get a mutable reference to the row index").len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).expect("Something unexpected happened while trying to get a mutable reference to the row index");
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).expect("Something unexpected happened while trying to get a mutable reference to the row index");
            row.delete(at.x);
        }
    }

    /// Saves the changes in the document
    ///
    /// # Errors
    ///
    /// It will return `Err` if `file_name` does not exist or the user
    /// does not have the permission to write to it
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(ref file_name) = self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[must_use]
    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }

        let mut position = Position { x: at.x, y: at.y };

        let start = if direction == SearchDirection::Forward {
            at.y
        } else {
            0
        };

        let end = if direction == SearchDirection::Forward {
            self.rows.len()
        } else {
            at.y.saturating_add(1)
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                } else {
                    position.y = position.y.saturating_sub(1);
                    if let Some(r) = self.rows.get(position.y) {
                        position.x = r.len();
                    }
                }
            } else {
                return None;
            }
        }
        None
    }
}
