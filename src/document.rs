use crate::FileType;
use crate::Position;
use crate::Row;
use crate::SearchDirection;
use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
    file_type: FileType,
}

impl Document {
    /// Opens a file in the editor
    ///
    /// # Errors
    /// It will return `Err` if it fails to open the file
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let file_type = FileType::from(filename);
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_owned()),
            dirty: false,
            file_type,
        })
    }

    /// Gets the name of the file that we are opening on the editor
    #[must_use]
    pub fn file_type(&self) -> String {
        self.file_type.name()
    }

    /// Gets the row based on an `index`
    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    /// Check if `rows` is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use]
    /// Get the length of `rows`
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

        let current_row = self
            .rows
            .get_mut(at.y)
            .expect("Something unexpected happened while trying to index rows.");

        let new_row = current_row.split(at.x);

        self.rows.insert(at.y.saturating_add(1), new_row);
    }

    /// Inserts a character in the document that is being read, at the position
    /// where the cursor is.
    ///
    /// # Panics
    ///
    /// It will panic if we try to insert in a position that is greater
    /// than the length of the document.
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
        } else if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }
        self.unhighlight_rows(at.y);
    }

    fn unhighlight_rows(&mut self, start: usize) {
        let start = start.saturating_sub(1);
        for row in self.rows.iter_mut().skip(start) {
            row.is_highlighted = false;
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
        self.unhighlight_rows(at.y);
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
            self.file_type = FileType::from(file_name);
            for row in &mut self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    /// Loop over the rows and highligh the words that correspond
    /// the word that was passed as a parameter.
    pub fn highlight(&mut self, word: &Option<String>, until: Option<usize>) {
        let mut start_with_comment = false;
        let until = if let Some(until) = until {
            if until.saturating_add(1) < self.rows.len() {
                until.saturating_add(1)
            } else {
                self.rows.len()
            }
        } else {
            self.rows.len()
        };

        let rows_indexed = self.rows.get_mut(..until).expect("Failed while trying to index rows");
        for row in rows_indexed {
            start_with_comment = row.highlight(self.file_type.highlighting_options(), word, start_with_comment);
        
        }
    }

    /// Returns a boolean indicating if the document has been changed or not
    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Returns an option with the elements that corresponds to a certain
    /// search query passed
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
