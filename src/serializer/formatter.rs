// These are some of the option i want to add to the formatter
// - New line before '[' for lists and '{' for structs and for unions seperately
// - New line before # for unions when using standard notation
// - Space after ':'
// - Inline list and struct if they are small enough
// - Inline union if it is small enough

pub struct Formatter {
    options: Vec<FormatOption>,
}

enum FormatOption {
    NewLineBeforeList,
    NewLineBeforeStruct,
    NewLineBeforeUnion,
    SpaceAfterColon,
    SpaceAfterComma,
    InlineList(usize),
    InlineStruct(usize),
    InlineUnion(usize),
    JsonCompatibleUnions,
}

impl Formatter {
    pub fn sensible_default() -> Self {
        Self::new()
            .space_after_colon()
            .space_after_comma()
            .inline_list(30)
            .inline_struct(30)
            .inline_union(30)
    }

    pub fn new() -> Self {
        Self { options: Vec::new() }
    }

    pub fn new_line_before_list(mut self) -> Self {
        self.options.push(FormatOption::NewLineBeforeList);
        self
    }

    pub fn new_line_before_struct(mut self) -> Self {
        self.options.push(FormatOption::NewLineBeforeStruct);
        self
    }

    pub fn new_line_before_union(mut self) -> Self {
        self.options.push(FormatOption::NewLineBeforeUnion);
        self
    }

    pub fn space_after_colon(mut self) -> Self {
        self.options.push(FormatOption::SpaceAfterColon);
        self
    }

    pub fn space_after_comma(mut self) -> Self {
        self.options.push(FormatOption::SpaceAfterComma);
        self
    }

    pub fn inline_list(mut self, max_size: usize) -> Self {
        self.options.push(FormatOption::InlineList(max_size));
        self
    }

    pub fn inline_struct(mut self, max_size: usize) -> Self {
        self.options.push(FormatOption::InlineStruct(max_size));
        self
    }

    pub fn inline_union(mut self, max_size: usize) -> Self {
        self.options.push(FormatOption::InlineUnion(max_size));
        self
    }

    pub fn json_compatible_unions(mut self) -> Self {
        self.options.push(FormatOption::JsonCompatibleUnions);
        self
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self { options: Vec::default() }
    }
}
