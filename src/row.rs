use crate::highlighting;
use crate::HighlightingOptions;
use crate::SearchDirection;
use std::cmp;
use termion::color;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    highlighting: Vec<highlighting::Type>,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            highlighting: Vec::new(),
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
        let mut current_highlighting = &highlighting::Type::None;
        #[allow(clippy::integer_arithmetic)]
        #[allow(clippy::string_slice)]
        for (index, grapheme) in self.string[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            if let Some(c) = grapheme.chars().next() {
                let highlighting_type = self
                    .highlighting
                    .get(index)
                    .unwrap_or(&highlighting::Type::None);

                if highlighting_type != current_highlighting {
                    current_highlighting = highlighting_type;
                    let start_highlight =
                        format!("{}", termion::color::Fg(highlighting_type.to_color()));
                    result.push_str(&*start_highlight);
                }

                if c == '\t' {
                    result.push(' ');
                } else {
                    result.push(c);
                }
            }
        }
        let end_highlight = format!("{}", termion::color::Fg(color::Reset));
        #[allow(clippy::string_slice)]
        result.push_str(&*end_highlight);
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
            highlighting: Vec::new(),
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
        if at > self.len || query.is_empty() {
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

    /// Goes through the characters of the `string` and highligh them by setting
    /// each value in the `highlighting` vector
    pub fn highlight(&mut self, opts: HighlightingOptions, word: Option<&str>) {
        let mut highlighting = Vec::new();
        let chars: Vec<char> = self.string.chars().collect();
        let mut matches = Vec::new();
        let mut search_index = 0;

        if let Some(word) = word {
            while let Some(search_match) = self.find(word, search_index, SearchDirection::Forward) {
                matches.push(search_match);
                if let Some(next_index) = search_match.checked_add(
                    word.get(..)
                        .expect("Failed while getting word slices")
                        .graphemes(true)
                        .count(),
                ) {
                    search_index = next_index;
                } else {
                    break;
                }
            }
        }

        let mut prev_is_separator = true;
        let mut in_string = false;
        let mut index = 0;
        while let Some(c) = chars.get(index) {
            if let Some(word) = word {
                if matches.contains(&index) {
                    for _ in word
                        .get(..)
                        .expect("Failed while getting word slices")
                        .graphemes(true)
                    {
                        index = index.saturating_add(1);
                        highlighting.push(highlighting::Type::Match);
                    }
                    continue;
                }
            }

            let previous_highlight = if index > 0 {
                highlighting
                    .get(index.saturating_sub(1))
                    .unwrap_or(&highlighting::Type::None)
            } else {
                &highlighting::Type::None
            };
            if opts.strings() {
                if in_string {
                    highlighting.push(highlighting::Type::String);

                    if *c == '\\' && index < self.len().saturating_sub(1) {
                        highlighting.push(highlighting::Type::String);
                        index = index.saturating_add(2);
                        continue;
                    }

                    if *c == '"' {
                        in_string = false;
                        prev_is_separator = true;
                    } else {
                        prev_is_separator = false;
                    }
                    index = index.saturating_add(1);
                    continue;
                } else if prev_is_separator && *c == '"' {
                    highlighting.push(highlighting::Type::String);
                    in_string = true;
                    prev_is_separator = true;
                    index = index.saturating_add(1);
                    continue;
                }
            }

            if opts.numbers() {
                if (c.is_ascii_digit()
                    && (prev_is_separator || *previous_highlight == highlighting::Type::Number))
                    || (*c == '.' && *previous_highlight == highlighting::Type::Number)
                {
                    highlighting.push(highlighting::Type::Number);
                } else {
                    highlighting.push(highlighting::Type::None);
                }
            } else {
                highlighting.push(highlighting::Type::None);
            }

            prev_is_separator = c.is_ascii_punctuation() || c.is_ascii_whitespace();
            index = index.saturating_add(1);
        }

        self.highlighting = highlighting;
    }
}
