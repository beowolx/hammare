use crate::SearchDirection;
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    #[must_use]
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        #[allow(clippy::integer_arithmetic)]
        #[allow(clippy::string_slice)]
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push(' ');
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }

    /// Gets the length of a row
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks if a row is empty or not
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Inserts a new character into the selected row
    #[allow(clippy::string_slice)]
    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len = self.len.saturating_add(1);
            return;
        }
        let mut result: String = String::new();
        let mut length: usize = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length = length.saturating_add(1);
            if index == at {
                length = length.saturating_add(1);
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.string = result;
    }

    /// Deletes a row at a given position
    #[allow(clippy::string_slice)]
    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }
        let mut result: String = String::new();
        let mut length: usize = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != at {
                length = length.saturating_add(1);
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.string = result;
    }

    /// Appends a row to a given `Row`
    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len = self.len.saturating_add(new.len);
    }

    /// Splits a row by its given position
    #[must_use]
    #[allow(clippy::string_slice)]
    pub fn split(&mut self, at: usize) -> Self {
        let mut row: String = String::new();
        let mut length: usize = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length: usize = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                length = length.saturating_add(1);
                row.push_str(grapheme);
            } else {
                splitted_length = splitted_length.saturating_add(1);
                splitted_row.push_str(grapheme);
            }
        }

        self.string = row;
        self.len = length;
        Self {
            string: splitted_row,
            len: splitted_length,
        }
    }

    /// Returns a byte slice of the Row's `String`'s contents
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    /// Returns the elements that correponds to the search query and direction
    /// passed
    #[must_use]
    #[allow(clippy::string_slice)]
    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len {
            return None;
        }

        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };

        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };

        let substring: String = self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end.saturating_sub(start))
            .collect();

        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };

        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in
                substring[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    let result = start.saturating_add(grapheme_index);
                    return Some(result);
                }
            }
        }
        None
    }
}
