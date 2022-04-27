use crate::Position;
use crate::Row;
use std::cmp::Ordering;
use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
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
            file_name: Some(filename.to_string()),
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
        if at.y > self.len() {
            return;
        }
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        let new_row = self.rows.get_mut(at.y).expect("Something unexpected happened while trying to get a mutable reference to the row index").split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    /// Inserts a character in the document that is being read, at the position
    /// where the cursor is.
    ///
    /// # Panics
    ///
    /// It will panic if we try to insert in a position that is greater
    /// than the length of the document.
    pub fn insert(&mut self, at: &Position, c: char) {
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        match at.y.cmp(&self.len()) {
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
    ///
    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.y >= len {
            return;
        }
        if at.x == self.rows.get_mut(at.y).expect("Something unexpected happened while trying to get a mutable reference to the row index").len() && at.y < len - 1 {
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
    pub fn save(&self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }
        Ok(())
    }
}
