pub struct MarkdownParser {
    raw: String
}

impl MarkdownParser {
    pub fn new(content: &str) -> MarkdownParser {
        MarkdownParser {
            raw: content.to_string()
        }
    }
}
