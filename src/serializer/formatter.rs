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

    pub fn new_line_before(mut self, context: FormatContext) -> Self {
        match context {
            FormatContext::List => {
                self.options.insert(
                    format!("{}{}", stringify!(new_line_before), FormatContext::List),
                    FormatOption::NewLineBefore(FormatContext::List));
            },
            FormatContext::Struct => {
                self.options.insert(
                    format!("{}{}", stringify!(new_line_before), FormatContext::Struct),
                    FormatOption::NewLineBefore(FormatContext::Struct));
            }
            FormatContext::Union => {
                self.options.insert(
                    format!("{}{}", stringify!(new_line_before), FormatContext::Union),
                    FormatOption::NewLineBefore(FormatContext::Union));
            },
            FormatContext::All => {
                self = self.new_line_before(FormatContext::List);
                self = self.new_line_before(FormatContext::Struct);
                self = self.new_line_before(FormatContext::Union);
            },
        }

        self
    }

    pub fn new_line_after(mut self, context: FormatContext) -> Self {
        match context {
            FormatContext::List => {
                self.options.insert(
                    format!("{}{}", stringify!(new_line_after), FormatContext::List),
                    FormatOption::NewLineAfter(FormatContext::List));
            },
            FormatContext::Struct => {
                self.options.insert(
                    format!("{}{}", stringify!(new_line_after), FormatContext::Struct),
                    FormatOption::NewLineAfter(FormatContext::Struct));
            }
            FormatContext::Union => {
                self.options.insert(
                    format!("{}{}", stringify!(new_line_after), FormatContext::Union),
                    FormatOption::NewLineAfter(FormatContext::Union));
            },
            FormatContext::All => {
                self = self.new_line_after(FormatContext::List);
                self = self.new_line_after(FormatContext::Struct);
                self = self.new_line_after(FormatContext::Union);
            },
        }

        self
    }

    pub fn space_after_colon(mut self) -> Self {
        self.options.insert(format!("{}", stringify!(space_after_colon)), FormatOption::SpaceAfterColon);
        self
    }

    pub fn space_after_comma(mut self) -> Self {
        self.options.insert(format!("{}", stringify!(space_after_comma)), FormatOption::SpaceAfterComma);
        self
    }

    pub fn inline(mut self, context: FormatContext, max_size: usize) -> Self {
        match context {
            FormatContext::List => {
                self.options.insert(
                    format!("{}{}", stringify!(inline), FormatContext::List),
                    FormatOption::Inline(FormatContext::List, max_size));
            },
            FormatContext::Struct => {
                self.options.insert(
                    format!("{}{}", stringify!(inline), FormatContext::Struct),
                    FormatOption::Inline(FormatContext::Struct, max_size));
            }
            FormatContext::Union => {
                self.options.insert(
                    format!("{}{}", stringify!(inline), FormatContext::Union),
                    FormatOption::Inline(FormatContext::Union, max_size));
            },
            FormatContext::All => {
                self = self.inline(FormatContext::List, max_size);
                self = self.inline(FormatContext::Struct, max_size);
                self = self.inline(FormatContext::Union, max_size);
            },
        }

        self
    }

    pub fn json_compatible_unions(mut self) -> Self {
        self.options.insert(format!("{}", stringify!(json_compatible_unions)), FormatOption::JsonCompatibleUnions);
        self
    }

    pub fn trailing_comma(mut self, context: FormatContext, trailing_comma: bool) -> Self {
        match context {
            FormatContext::List => {
                self.options.insert(
                    format!("{}{}", stringify!(trailing_comma), FormatContext::List),
                    FormatOption::TrailingComma(FormatContext::List, trailing_comma));
            },
            FormatContext::Struct => {
                self.options.insert(
                    format!("{}{}", stringify!(trailing_comma), FormatContext::Struct),
                    FormatOption::TrailingComma(FormatContext::Struct, trailing_comma));
            }
            FormatContext::Union => {
                self.options.insert(
                    format!("{}{}", stringify!(trailing_comma), FormatContext::Union),
                    FormatOption::TrailingComma(FormatContext::Union, trailing_comma));
            },
            FormatContext::All => {
                self = self.trailing_comma(FormatContext::List, trailing_comma);
                self = self.trailing_comma(FormatContext::Struct, trailing_comma);
                self = self.trailing_comma(FormatContext::Union, trailing_comma);
            },
        }
        
        self
    }

    pub fn build(self) -> Formatter {
        Formatter { options: self.options.values().cloned().collect() }
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
