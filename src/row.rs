use crate::highlighting;
use crate::HighlightingOptions;
use crate::SearchDirection;
use std::cmp;
use termion::color;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    pub is_highlighted: bool,
    highlighting: Vec<highlighting::Type>,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            highlighting: Vec::new(),
            is_highlighted: false,
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
        self.is_highlighted = false;
        Self {
            string: splitted_row,
            len: splitted_length,
            is_highlighted: false,
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

    /// Highlight the matches found when user searchs for an element
    fn highlight_match(&mut self, word: &Option<String>) {
        if let Some(ref word) = *word {
            if word.is_empty() {
                return;
            }
            let mut index = 0;
            while let Some(search_match) = self.find(word, index, SearchDirection::Forward) {
                if let Some(next_index) = search_match.checked_add(word.get(..).expect("Failed while trying to get slice of `word`").graphemes(true).count()) {
                    for i in search_match..next_index {
                       if let Some(element) = self.highlighting.get_mut(i) {
                           *element = highlighting::Type::Match;
                       }
                    }
                    index = next_index;
                } else {
                    break;
                }
            }
        }
    }

    /// Highligh a substring with a given type.
    fn highlight_str(&mut self, index: &mut usize, substring: &str, chars: &[char], hl_type: highlighting::Type) -> bool {
        if substring.is_empty() {
            return false;
        }

        for (substring_index, c) in substring.chars().enumerate() {
            if let Some(next_char) = chars.get(index.saturating_add(substring_index)) {
                if *next_char != c {
                    return false;
                }
            } else {
                return false;
            }
        }
        for _ in 0..substring.len() {
            self.highlighting.push(hl_type);
            *index = index.saturating_add(1);
        }
        true
    }

    /// Highligh keywords present in the vector of language's special keywords
    fn highlight_keywords(&mut self, index: &mut usize, chars: &[char], keywords: &[String], hl_type: highlighting::Type) -> bool {
        if *index > 0 {
            let prev_char = chars.get(index.saturating_sub(1)).expect("Failed trying to index chars at `highlight_keywords` to get `prev_char`");
            if !is_separator(*prev_char) {
                return false;
            }
        }
        
        for word in keywords {
            if *index < chars.len().saturating_sub(word.len()) {
                let next_char = chars.get(index.saturating_add(word.len())).expect("Failed trying to index chars at `highlight_keywords` to get `next_char`");
                if !is_separator(*next_char) {
                    continue;
                }
            }
            if self.highlight_str(index, word, chars, hl_type) {
                return true;
            }
        }
        false
    }

    /// Highlight keywords present in the `highlight_primary_keywords` vector
    fn highlight_primary_keywords(&mut self, index: &mut usize, opts: &HighlightingOptions, chars: &[char]) -> bool {
        self.highlight_keywords(index, chars, opts.primary_keywords(), highlighting::Type::PrimaryKeywords)
    }

    /// Highlight keywords present in the `secondary_primary_keywords` vector
    fn highlight_secondary_keywords(&mut self, index: &mut usize, opts: &HighlightingOptions, chars: &[char]) -> bool {
        self.highlight_keywords(index, chars, opts.secondary_keywords(), highlighting::Type::SecondaryKeywords)
    }

    /// Returns a boolean and does the logic to highlight a `char`
    fn highlight_char(&mut self, index: &mut usize, opts: &HighlightingOptions, c: char, chars: &[char]) -> bool {
        if opts.characters() && c == '\'' {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                let closing_index = if *next_char == '\\' {
                    index.saturating_add(3)
                } else {
                    index.saturating_add(2)
                };
                if let Some(closing_char) = chars.get(closing_index) {
                    if *closing_char == '\'' {
                        for _ in 0..=closing_index.saturating_sub(*index) {
                            self.highlighting.push(highlighting::Type::Character);
                            *index = index.saturating_add(1);
                        }
                        return true;
                    }
                }
            }

        }
        false
    }

    /// Returns a boolean and does the logic to highlight a comment
    fn highlight_comment(&mut self, index: &mut usize, opts: &HighlightingOptions, c: char, chars: &[char]) -> bool {
        if opts.comments() && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '/' {
                    for _ in *index..chars.len() {
                        self.highlighting.push(highlighting::Type::Comment);
                        *index = index.saturating_add(1);
                    }
                    return true;
                }
            };
        }
        false
    }

    fn highlight_multiline_comment(&mut self, index: &mut usize, opts: &HighlightingOptions, c: char, chars: &[char]) -> bool {
        if opts.comments() && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '*' {
                    let indexed = self.string.get(index.saturating_add(2)..2).expect("Failed while trying to indexing string.").find("*/");

                    let closing_index = if let Some(closing_index) = indexed {
                        index.saturating_add(closing_index).saturating_add(4)
                    } else {
                        chars.len()
                    };

                    for _ in *index..closing_index {
                        self.highlighting.push(highlighting::Type::MultilineComment);
                        *index = index.saturating_add(1);
                    }
                    return true;
                }
            };
        }
        false
    }

    /// Returns a boolean and does the logic to highlight a `string`
    fn highlight_string(&mut self, index: &mut usize, opts: &HighlightingOptions, c: char, chars: &[char]) -> bool {
        if opts.strings() && c == '"' {
            loop {
                self.highlighting.push(highlighting::Type::String);
                *index = index.saturating_add(1);
                if let Some(next_char) = chars.get(*index) {
                    if *next_char == '"' {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.highlighting.push(highlighting::Type::String);
            *index = index.saturating_add(1);
            return true;
        }
        false
    }

    /// Returns a boolean and does the logic to highlight a number
    fn highlight_number(&mut self, index: &mut usize, opts: &HighlightingOptions, c: char, chars: &[char]) -> bool {
        if opts.numbers() && c.is_ascii_digit() {
            if *index > 0 {
                let prev_char = chars.get(index.saturating_sub(1)).expect("Failed trying to index `chars` at `highlight_number()`");
                if !is_separator(*prev_char) {
                    return false;
                }
            }
            loop {
                self.highlighting.push(highlighting::Type::Number);
                *index = index.saturating_add(1);
                if let Some(next_char) = chars.get(*index) {
                    if *next_char != '.' && !next_char.is_ascii_digit() {
                        break;
                    }
                } else {
                    break;
                }
            }
            return true;
        }
        false
    }

    /// Check if any of the `HighlightingOptions` applies and if not,
    /// pushes to the `highlighting` vec `None`
    pub fn highlight(&mut self, opts: &HighlightingOptions, word: &Option<String>, start_with_comment: bool) -> bool {
        let chars: Vec<char> = self.string.chars().collect();
        if self.is_highlighted && word.is_none() {
            if let Some(hl_type) = self.highlighting.last() {
                if *hl_type == highlighting::Type::MultilineComment && self.string.get(self.string.len().saturating_sub(2)..).expect("Failed while indexing string.") == "*/" {
                    return true;
                }
            }
            return false;
        }
        self.highlighting = Vec::new();
        let mut index = 0;
        let mut in_ml_comment = start_with_comment;
        if in_ml_comment {
            let closing_index = if let Some(closing_index) = self.string.find("*/") {
                closing_index.saturating_add(2)
            } else {
                chars.len()
            };
            for _ in 0..closing_index {
                self.highlighting.push(highlighting::Type::MultilineComment);
            }
            index = closing_index;
        }
        while let Some(c) = chars.get(index) {
            if self.highlight_multiline_comment(&mut index, opts, *c, &chars) {
                in_ml_comment = true;
                continue;
            }
            in_ml_comment = false;

            if self.highlight_char(&mut index, opts, *c, &chars) || self.highlight_comment(&mut index, opts, *c, &chars) || self.highlight_primary_keywords(&mut index, opts, &chars) || self.highlight_secondary_keywords(&mut index, opts, &chars) || self.highlight_string(&mut index, opts, *c, &chars) || self.highlight_number(&mut index, opts, *c, &chars) {
                continue;
            }

            // We reserve capacity in the highlighting array for at least one more
            // element, to check that the `.push()` is safe as we are using `usize`
            self.highlighting.reserve(1);
            self.highlighting.push(highlighting::Type::None);

            index = index.saturating_add(1);
        }
        self.highlight_match(word);
        let comment_range = self.string.get(self.string.len().saturating_sub(2)..).expect("Failed while trying to index string");
        
        if in_ml_comment && comment_range != "*/" {
            return true;
        }
        self.is_highlighted = true;
        false
    }

}

fn is_separator(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_ascii_whitespace()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_highlight_find() {
        let mut row = Row::from("1testtest");
        row.highlighting = vec![
            highlighting::Type::Number,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
        ];
        row.highlight_match(&Some("t".to_owned()));
        assert_eq!(
            vec![
                highlighting::Type::Number,
                highlighting::Type::Match,
                highlighting::Type::None,
                highlighting::Type::None,
                highlighting::Type::Match,
                highlighting::Type::Match,
                highlighting::Type::None,
                highlighting::Type::None,
                highlighting::Type::Match
            ],
            row.highlighting
        );
    }

    #[test]
    fn test_find() {
        let row = Row::from("1testtest");
        assert_eq!(row.find("t", 0, SearchDirection::Forward), Some(1));
        assert_eq!(row.find("t", 2, SearchDirection::Forward), Some(4));
        assert_eq!(row.find("t", 5, SearchDirection::Forward), Some(5));
    }

}