pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[derive(Default, Copy, Clone)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightingOptions::default(),
        }
    }
}

impl FileType {
    /// Returns the name of the file
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// Highligh the options for the document's language/type
    #[must_use]
    pub fn highlighting_options(&self) -> HighlightingOptions {
        self.hl_opts
    }
    /// Gets the documents extenstion type
    #[must_use]
    pub fn from(file_name: &str) -> Self {
        if file_name
            .rsplit('.')
            .next()
            .map(|ext| ext.eq_ignore_ascii_case("rs"))
            == Some(true)
        {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                },
            };
        }
        Self::default()
    }
}

impl HighlightingOptions {
    /// Return a boolean indicating if we should highlight numbers or not
    #[must_use]
    pub fn numbers(self) -> bool {
        self.numbers
    }

    /// Return a boolean indicating if we should highlight strings or not
    #[must_use]
    pub fn strings(self) -> bool {
        self.strings
    }

    /// Return a boolean indicating if we should highlight characters or not
    #[must_use]
    pub fn characters(self) -> bool {
        self.characters
    }
}
