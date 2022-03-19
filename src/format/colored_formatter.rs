use colored::{ColoredString, Colorize, Style};
use crate::format::{MatchOutput, Notice};
use crate::pattern::Position;

enum ColorType {
    Verbose,
    Info,
    Title,
    Error,
    Highlight,
}

pub struct ColoredFormatter();

impl ColoredFormatter {
    fn colorize_text(&self, text: &str, level: ColorType) -> String {
        match level {
            ColorType::Verbose => text.to_string(),
            ColorType::Title => text.on_bright_white().blue().to_string(),
            ColorType::Info => text.on_truecolor(0, 139, 139).white().to_string(),
            ColorType::Error => text.on_red().bright_white().bold().to_string(),
            ColorType::Highlight => text.on_truecolor(139, 105, 20).bold().underline().to_string()
        }
    }

    fn file_notice(&self, path: &str) -> String {
        let mut notice = " * File: ".to_string() + path;
        notice = self.colorize_text(&notice, ColorType::Title);

        notice
    }
    fn line_match_notice(&self, line_number: &str, content: &str, colored_positions: &Vec<Position>) -> String {
        let mut final_str = String::new();

        // Line number
        final_str.push_str(&self.colorize_text(format!("{:^8}", line_number).as_str(), ColorType::Info));

        final_str.push_str("| ");

        // Line content
        let mut cursor: usize = 0;
        for position in colored_positions {
            final_str.push_str(&content[cursor..position.start]);
            final_str.push_str(
                &self.colorize_text(
                    &content[position.start..position.end],
                    ColorType::Highlight,
                )
            );
            cursor = position.end;
        }
        final_str.push_str(&content[cursor..]);

        final_str
    }
    fn plain_notice(&self, text: &str) -> String {
        self.colorize_text(&(" ".to_string() + text), ColorType::Info)
    }

    fn error_notice(&self, error: &str, line_number: Option<&str>, path: Option<&str>) -> String {
        let mut final_str = String::new();
        final_str.push_str(
            &self.colorize_text(
                format!(" An Error Occurred: {}\t\t", error)
                    .as_str(),
                ColorType::Error,
            )
        );
        final_str.push_str("\n");

        if line_number.is_some() || path.is_some() {
            final_str.push_str(
                self.colorize_text(
                    " Details:",
                    ColorType::Error,
                ).as_str()
            );
        }

        if let Some(line_number) = line_number {
            final_str.push_str(
                format!(
                    "  ... At line: {}",
                    self.colorize_text(
                        &line_number.to_string(),
                        ColorType::Highlight,
                    )
                ).as_str()
            );
            final_str.push_str("\n");
        }

        if let Some(path) = path {
            final_str.push_str(
                format!("  ... At file: {}", self.colorize_text(
                    path,
                    ColorType::Highlight,
                )).as_str()
            );
            final_str.push_str("\n");
        }


        final_str
    }
}

impl MatchOutput<String> for ColoredFormatter {
    fn create_notice(&self, notice: Notice) -> String {
        match notice {
            Notice::FileNotice(notice) => self.file_notice(notice.path),
            Notice::LineMatchNotice(notice) => self.line_match_notice(notice.line_number, notice.content, &notice.colored_positions),
            Notice::ErrorNotice(notice) => self.error_notice(notice.error, notice.line_number, notice.path),
            Notice::PlainNotice(str) => self.plain_notice(str),
            _ => self.error_notice(&format!("Unknown Notice Command: {:?}", notice), None, None)
        }
    }
}