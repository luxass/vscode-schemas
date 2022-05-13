pub mod parser;

use parser::{MarkdownParser};

pub fn parse_markdown_file(content: &str) {
    info!("Running Markdown Parser");
    let markdown_parser = MarkdownParser::new(content);


}
