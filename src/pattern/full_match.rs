use crate::pattern::{Pattern, Position};

pub struct FullMatch(String);

impl Pattern for FullMatch {
    // FullMatch匹配所有的字符串
    fn check_rule(_: &str) -> bool {
        true
    }

    fn try_create(text: &str) -> Option<Self> where Self: Sized {
        Some(FullMatch(text.to_string()))
    }

    fn is_match(&self, text: &str) -> bool {
        text == self.0
    }

    fn match_result(&self, text: &str) -> Vec<Position> {
        let mut positions: Vec<Position> = vec!();

        let source_len = text.len();
        let target_len = self.0.len();
        let mut cursor = 0;
        let mut part_text = text;
        while let Some(start_index) = part_text.find(&self.0) {
            positions.push(Position {
                start: cursor + start_index,
                end: cursor + start_index + target_len,
            });
            cursor = cursor + start_index + target_len;
            println!("{}", cursor);
            if cursor >= source_len {
                break;
            }
            part_text = &text[cursor..];
        }
        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_match_rule_should_work() {
        // 任意文本均可以作为规则
        assert!(FullMatch::check_rule(""));
        assert!(FullMatch::check_rule("abcde"));
        assert!(FullMatch::check_rule("12345"));
        assert!(FullMatch::check_rule("!@#$%^&*()."));
    }

    #[test]
    fn full_match_match_should_work() {
        let pattern = FullMatch::try_create("hello").unwrap();
        assert!(pattern.is_match("hello"));
        assert!(!pattern.is_match("hell"));
        assert!(!pattern.is_match("helloworld"));
    }

    #[test]
    fn full_match_position_should_work() {
        let pattern = FullMatch::try_create("234").unwrap();
        let text = "012345623434";
        let result = pattern.match_result(text);
        assert_eq!(
            result,
            vec!(
                Position { start: 2, end: 5 },
                Position { start: 7, end: 10 }
            )
        )
    }
}