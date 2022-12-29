use pulldown_cmark::{Options, Parser, html};


pub struct MarkdownBuilder {
    raw: String,
    options: Options
}

impl MarkdownBuilder {
    pub fn new(input: String) -> Self {
        Self { raw: input, options: Options::all() }

    }

    pub fn format_to_html(&mut self) -> String {
        let mut html_output = String::with_capacity(self.raw.len() * 3 / 2);
        let parser = Parser::new_ext(&self.raw, self.options);
        html::push_html(&mut html_output, parser);
        html_output
    }
}
