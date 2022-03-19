
mod full_match;
mod wildcard_match;
mod regex_match;

pub use full_match::FullMatch;
pub use wildcard_match::WildcardMatch;
pub use regex_match::RegexMatch;

#[derive(Debug,Eq, PartialEq)]
// start为开始的字符的index，end为结束字符后一位的索引。
pub struct Position {
    pub start: usize,
    pub end: usize,
}

pub trait Pattern:Sized {
    // 检查text所提供的pattern是否符合该pattern的规则且无语法错误
    fn check_rule(text: &str) -> bool{
        let instance = Self::try_create(text);
        instance.is_some()
    }

    // 尝试以给定规则创建Pattern
    fn try_create(text: &str) -> Option<Self>;

    // 检查
    fn is_match(&self, text: &str) -> bool;


    fn match_result(&self, text: &str) -> Vec<Position>;

}

