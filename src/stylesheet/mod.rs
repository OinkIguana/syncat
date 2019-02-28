use std::collections::BTreeMap;

use tree_sitter::{Tree, Node};
use regex::Regex;

use crate::error::Error;
use crate::style::{Colour, StyleBuilder};
use crate::language::Lang;

mod resolver;
mod parser;

pub use resolver::Context;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum SelectorSegment {
    Kind(String),
    Token(String),
    TokenPattern(String),
    NoChildren(Box<SelectorSegment>),
    DirectChild(Box<SelectorSegment>),
    BranchCheck(Vec<SelectorSegment>),
}

#[derive(Default, Debug)]
pub struct Stylesheet {
    style: StyleBuilder,
    scopes: BTreeMap<SelectorSegment, Stylesheet>,
}
