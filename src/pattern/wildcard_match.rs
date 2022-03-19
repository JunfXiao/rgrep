use globset::{Glob, GlobMatcher};
use crate::pattern::{Pattern, Position};

pub struct WildcardMatch(GlobMatcher);

impl Pattern for WildcardMatch {
    fn check_rule(text: &str) -> bool {
        let result = Glob::new(text);
        return result.is_ok();
    }

    fn try_create(text: &str) -> Option<Self> where Self: Sized {
        let glob = Glob::new(text);
        if let Ok(glob) = glob {
            Some(WildcardMatch(glob.compile_matcher()))
        } else {
            None
        }
    }

    fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }

    fn match_result(&self, text: &str) -> Vec<Position> {
        unimplemented!("Wildcard can not be used for text matching...")
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn wilecard_match_should_work() {
        // 任意文本均可以作为规则
        assert!(WildcardMatch::check_rule(""));
        assert!(WildcardMatch::check_rule("abcde"));
        assert!(WildcardMatch::check_rule("12345"));
        assert!(WildcardMatch::check_rule("!@#$%^&*()."));

        let pattern = WildcardMatch::try_create("a?c*.txt").unwrap();
        assert!(pattern.is_match("abc.txt"));
        assert!(pattern.is_match("accd.txt"));
        assert!(pattern.is_match("accdddd.txt"));
        assert!(!pattern.is_match("acdddd.txt"));
    }
}