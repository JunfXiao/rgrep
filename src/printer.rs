use std::fmt::Display;
use std::marker::PhantomData;
use crate::{ErrorNotice, FileNotice, LineMatchNotice, MatchOutput, Notice, Position};


pub struct Printer<M: Display, T: MatchOutput<M>>
{
    // To pass the compiler
    // Otherwise: error[E0392]: parameter `M` is never used
    _m : PhantomData<M>,
    pub formatter: T,
}

impl<M:Display,T:MatchOutput<M>> Printer<M, T> {

    pub fn new(formatter: T) -> Printer<M,T>{
        Self{
            _m: Default::default(),
            formatter
        }
    }

    pub fn print_text(&self,text: &str)
    {
        println!("{}", text);
    }

    pub fn print_notice(&self,text: &str)
    {
        println!("{}", self.formatter.create_notice(Notice::PlainNotice(text)));
    }

    pub fn print_error(&self, error: &str, line_number: Option<&str>, path: Option<&str>)
    {
        println!("{}", self.formatter.create_notice(Notice::ErrorNotice(ErrorNotice {
            error,
            line_number,
            path,
        })));
    }

    pub fn print_file_meta(&self, path: &str)
    {
        println!("\n{:-^100}\n","");
        println!("{:<100}", self.formatter.create_notice(Notice::FileNotice(FileNotice {
            path
        })));
    }

    pub fn print_line_match(&self,line_number: &str, content: &str, colored_positions: &Vec<Position>)
    {
        println!("{}", self.formatter.create_notice(Notice::LineMatchNotice(LineMatchNotice {
            line_number,
            content,
            colored_positions,
        })));
    }

    pub fn print_line_gap(&self){
        println!("{:^8}", "...");
    }

}

