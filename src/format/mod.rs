mod colored_formatter;

use std::fmt::Display;
use crate::pattern::Position;
pub use colored_formatter::*;

#[derive(Debug)]
pub enum Notice<'a>{
    FileNotice(FileNotice<'a>),
    LineMatchNotice(LineMatchNotice<'a>),
    ErrorNotice(ErrorNotice<'a>),
    PlainNotice(&'a str)
}

#[derive(Debug)]
pub struct FileNotice<'a>{
    pub path: &'a str
}

#[derive(Debug)]
pub struct LineMatchNotice<'a>{
    pub line_number:&'a str,
    pub content: &'a str,
    pub colored_positions:&'a Vec<Position>
}

#[derive(Debug)]
pub struct ErrorNotice<'a>{
    pub error: &'a str,
    pub line_number:Option<&'a str>,
    pub path: Option<&'a str>
}



pub trait MatchOutput<T>
    where T: Display {
    fn create_notice(&self, notice: Notice) ->  T;
}