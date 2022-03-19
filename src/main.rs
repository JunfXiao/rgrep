mod fs;
mod pattern;
mod format;
mod printer;

use std::fmt::Display;
use std::path::Path;
use clap::{ArgEnum, Parser, Subcommand};
use crate::fs::{FsOperation, MDirReader, MLineReader, TokioFsOperation};
use crate::pattern::{Pattern, Position, RegexMatch, WildcardMatch};
use crate::format::*;
use crate::printer::Printer;

#[derive(Parser, Debug)]
#[clap(name = "rust-grep", author = "J. X.", version = "0.1.0", about = "A simple grep tool powered by rust")]
struct Args {
    #[clap(help = "The pattern to search for")]
    pattern: String,
    #[clap(help = "The name of the file to search")]
    file: String,
}


#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    let formatter = ColoredFormatter();

    let printer = Printer::new(formatter);

    let fs = TokioFsOperation();


    if !RegexMatch::check_rule(args.pattern.as_str()) {
        printer.print_error("Invalid regex!", None, None);
        std::process::exit(-1);
    }
    let regex = RegexMatch::try_create(args.pattern.as_str()).unwrap();

    let wildcard = WildcardMatch::try_create(args.file.as_str()).unwrap();

    // Loading Files
    // printer.print_notice("Init: Loading Files...");

    let files = fs.read_dir(Path::new(".")).await;

    if let Err(err) = files {
        printer.print_error(&format!("Cannot access filesystem! {}", err), None, Some("."));
        std::process::exit(-2);
    }

    // Read files
    let mut reader = files.unwrap();
    while let Some(file) = reader.next_file().await {
        // Check if it's a file
        if !file.is_dir {
            let path = file.path;

            // Check filename
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if !wildcard.is_match(file_name) {
                continue;
            }

            // Init
            let mut header_printed = false;
            let mut continuous_line_found = true;
            let mut lines = fs.read_lines(path.as_path()).await.unwrap();
            let mut line_number: usize = 0;
            // Loop to read lines
            while let Some(content) = lines.read_line().await {
                line_number = line_number + 1;
                let positions = regex.match_result(&content);
                if positions.is_empty() {
                    continuous_line_found = false;
                } else {
                    // Print header only if there are matches in a file
                    if !header_printed {
                        header_printed = true;
                        continuous_line_found = true;
                        printer.print_file_meta(path.to_str().unwrap());
                    }
                    if !continuous_line_found {
                        printer.print_line_gap()
                    }
                    continuous_line_found = true;

                    printer.print_line_match(&line_number.to_string(), &content, &positions);
                }
            }
        }
    }
}
