use regex::{Captures, Regex};
use crate::pattern::{Pattern, Position};

pub struct RegexMatch(Regex);


impl<'a> From<Captures<'a>> for Position{
    fn from(c: Captures) -> Self {
        let capture = c.get(0).unwrap();
        Position{
            start: capture.start(),
            end: capture.end()
        }
    }
}

impl Pattern for RegexMatch {
    fn check_rule(text: &str) -> bool {
        let re = Regex::new(text);
        return re.is_ok();
    }

    fn try_create(text: &str) -> Option<Self> where Self: Sized {
        let re = Regex::new(text);
        if let Ok(re) = re {
            return Some(RegexMatch(re));
        }
        return None;
    }

    fn is_match(&self, text: &str) -> bool {
        return self.0.is_match(text);
    }

    fn match_result(&self, text: &str) -> Vec<Position> {
       self.0.captures_iter(text)
           .map(|c|c.into())
           .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_rule_should_work() {
        // 任意文本均可以作为规则
        assert!(RegexMatch::check_rule(r""));
        assert!(RegexMatch::check_rule(r"abcde"));
        assert!(RegexMatch::check_rule(r"12345"));
        assert!(RegexMatch::check_rule(r"[0-9]+\w"));
        assert!(RegexMatch::check_rule(r"\((\))"));

        assert!(!RegexMatch::check_rule(r"["));
        assert!(!RegexMatch::check_rule(r"())"));

    }

    #[test]
    fn regex_match_should_work() {
        let pattern = RegexMatch::try_create(r"hyp{1,3}er").unwrap();
        assert!(pattern.is_match("hyper"));
        assert!(pattern.is_match("hyppper"));
        assert!(!pattern.is_match("hyer"));
        assert!(!pattern.is_match("hypppppper"));
    }

    #[test]
    fn regex_position_should_work() {
        let pattern = RegexMatch::try_create(r"hyp{1,3}er").unwrap();
        let text = "hyper & hypppper & hyppper";
        let result = pattern.match_result(text);
        assert_eq!(
            result,
            vec!(
                Position { start: 0, end: 5 },
                Position { start: 19, end: 26 }
            )
        )
    }
}