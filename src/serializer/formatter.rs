// These are some of the option i want to add to the formatter
// - New line before '[' for lists and '{' for structs and for unions seperately
// - New line before # for unions when using standard notation
// - Space after ':'
// - Inline list and struct if they are small enough
// - Inline union if it is small enough

use std::{collections::{HashMap, HashSet}, fmt::Display};

pub struct Formatter {
    options: HashSet<FormatOption>,
}

impl Formatter {
    pub fn has_new_line_before(&self, context: FormatContext) -> bool {
        self.options.contains(&FormatOption::NewLineBefore(context))
    }

    pub fn has_new_line_after(&self, context: FormatContext) -> bool {
        self.options.contains(&FormatOption::NewLineAfter(context))
    }

    pub fn has_space_after_colon(&self) -> bool {
        self.options.contains(&FormatOption::SpaceAfterColon)
    }

    pub fn has_space_after_comma(&self) -> bool {
        self.options.contains(&FormatOption::SpaceAfterComma)
    }

    pub fn has_inline(&self, context: FormatContext) -> Option<usize> {
        self.options.iter()
            .filter_map(|option| {
                match option {
                    FormatOption::Inline(c, max_size) if *c == context => Some(*max_size),
                    _ => None,
                }
            })
            .next()
    }

    pub fn has_json_compatible_unions(&self) -> bool {
        self.options.contains(&FormatOption::JsonCompatibleUnions)
    }

    pub fn has_trailing_comma(&self, context: FormatContext) -> Option<bool> {
        self.options.iter()
            .filter_map(|option| {
                match option {
                    FormatOption::TrailingComma(c, trailing_comma) if *c == context => Some(*trailing_comma),
                    _ => None,
                }
            })
            .next()
    }

    pub fn has_indented(&self, context: FormatContext) -> bool {
        self.options.contains(&FormatOption::Indented(context))
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self { options: HashSet::default() }
    }
}

pub struct FormatBuilder {
    options: HashMap<String, FormatOption>,
}

impl FormatBuilder {
    pub fn sensible_default() -> Self {
        Self::new()
            .new_line_after(FormatContext::All)
            .space_after_colon()
            .space_after_comma()
            .inline(FormatContext::List, 30)
            .trailing_comma(FormatContext::All, false)
    }

    pub fn json_like() -> Self {
        Self::new()
            .new_line_before(FormatContext::All)
            .new_line_after(FormatContext::All)
            .space_after_colon()
            .space_after_comma()
            .inline(FormatContext::List, 30)
            .trailing_comma(FormatContext::All, false)
            .json_compatible_unions()
    }

    pub fn new() -> Self {
        Self { options: HashMap::new() }
    }

    pub fn new_line_before(self, context: FormatContext) -> Self {
        self.match_context(context, |mut b: FormatBuilder, c| {
            b.options.insert(
                format!("{}{}", stringify!(new_line_before), c),
                FormatOption::NewLineBefore(c));

            b
        })
    }

    pub fn new_line_after(self, context: FormatContext) -> Self {
        self.match_context(context, |mut b: FormatBuilder, c| {
            b.options.insert(
                format!("{}{}", stringify!(new_line_after), c),
                FormatOption::NewLineAfter(c));

            b
        })
    }

    pub fn space_after_colon(mut self) -> Self {
        self.options.insert(format!("{}", stringify!(space_after_colon)), FormatOption::SpaceAfterColon);
        self
    }

    pub fn space_after_comma(mut self) -> Self {
        self.options.insert(format!("{}", stringify!(space_after_comma)), FormatOption::SpaceAfterComma);
        self
    }

    pub fn inline(self, context: FormatContext, max_size: usize) -> Self {
        self.match_context(context, |mut b, c| {
            b.options.insert(
                format!("{}{}", stringify!(inline), c),
                FormatOption::Inline(c, max_size));

            b
        })
    }

    pub fn json_compatible_unions(mut self) -> Self {
        self.options.insert(format!("{}", stringify!(json_compatible_unions)), FormatOption::JsonCompatibleUnions);
        self
    }

    pub fn trailing_comma(self, context: FormatContext, trailing_comma: bool) -> Self {
        self.match_context(context, |mut b, c| {
            b.options.insert(
                format!("{}{}", stringify!(trailing_comma), c),
                FormatOption::TrailingComma(c, trailing_comma));

            b
        })
    }

    pub fn indented(self, context: FormatContext) -> Self {
        self.match_context(context, |mut b, c| {
            b.options.insert(
                format!("{}{}", stringify!(indented), c),
                FormatOption::Indented(c));

            b
        })
    }

    pub fn build(self) -> Formatter {
        Formatter { options: self.options.values().cloned().collect() }
    }

    fn match_context<F: Fn(Self, FormatContext) -> Self>(mut self, context: FormatContext, func: F) -> Self {
        match context {
            FormatContext::List => {
                self = func(self, FormatContext::List);
            },
            FormatContext::Struct => {
                self = func(self, FormatContext::Struct);
            }
            FormatContext::Union => {
                self = func(self, FormatContext::Union);
            },
            FormatContext::All => {
                self = func(self, FormatContext::List);
                self = func(self, FormatContext::Struct);
                self = func(self, FormatContext::Union);
            },
        }

        self
    }
}

impl Default for FormatBuilder {
    fn default() -> Self {
        Self { options: HashMap::default() }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum FormatOption {
    NewLineBefore(FormatContext),
    NewLineAfter(FormatContext),
    SpaceAfterColon,
    SpaceAfterComma,
    Inline(FormatContext, usize),
    JsonCompatibleUnions,
    TrailingComma(FormatContext, bool),
    Indented(FormatContext),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FormatContext {
    List,
    Struct,
    Union,
    All,
}

impl Display for FormatContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatContext::List => write!(f, "list"),
            FormatContext::Struct => write!(f, "struct"),
            FormatContext::Union => write!(f, "union"),
            FormatContext::All => {
                write!(f, "{}", FormatContext::List)?;
                write!(f, "{}", FormatContext::Struct)?;
                write!(f, "{}", FormatContext::Union)?;
                Ok(())
            },
        }
    }
}
