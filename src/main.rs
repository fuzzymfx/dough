extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Options};
use pulldown_cmark::html;

fn main() {
    let markdown_input = "# Hello World\n\nThis is a paragraph";
    let mut html_output = String::new();
    let parser = Parser::new_ext(markdown_input, Options::all());
    html::push_html(&mut html_output, parser);
    println!("{}", html_output);
}
