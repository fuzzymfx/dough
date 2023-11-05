use colored::*;
use markdown::mdast;

fn visit_md_node(node: mdast::Node) -> Option<String> {
    match node {
        mdast::Node::Root(root) => {
            let mut result = String::new();
            for child in root.children {
                if let Some(text) = visit_md_node(child) {
                    result.push_str(&text);
                }
            }
            Some(result)
        }

        mdast::Node::Paragraph(paragraph) => {
            let mut result = String::new();
            for child in paragraph.children {
                if let Some(text) = visit_md_node(child) {
                    result.push_str(&text);
                }
            }
            Some(result)
        }

        mdast::Node::Text(text) => Some(text.value),
        mdast::Node::Heading(heading) => {
            let mut result = String::new();
            for child in heading.children {
                if let Some(text) = visit_md_node(child) {
                    let colored_text = text.bold().blue().to_string();
                    result.push_str(&colored_text);
                }
            }
            Some(result)
        }

        _ => None,
    }
}

pub fn prettify(md_text: &str) -> Result<String, String> {
    let parsed = markdown::to_mdast(md_text, &markdown::ParseOptions::default());
    let mut prettified = String::new();

    match parsed {
        Err(err) => return Err(format!("Could not prettify markdown, error: {}", err)),
        Ok(node) => {
            let result = visit_md_node(node);
            if let Some(text) = result {
                prettified.push_str(&text);
            }
        }
    }
    return Ok(prettified);
}
