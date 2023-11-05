use colored::*;
use markdown::mdast;

fn join_children_with(join_fn: fn(String) -> String, children: Vec<mdast::Node>) -> String {
    let mut result = String::default();
    for child in children {
        if let Some(text) = visit_md_node(child) {
            let decorated_text = join_fn(text);
            result.push_str(&decorated_text);
        }
    }
    return result;
}

fn join_children(children: Vec<mdast::Node>) -> String {
    return join_children_with(|x| x, children);
}

// Recursively visit the mdast tree and return a string
fn visit_md_node(node: mdast::Node) -> Option<String> {
    match node {
        mdast::Node::Root(root) => {
            return Some(join_children(root.children));
        }

        mdast::Node::Paragraph(paragraph) => {
            let mut result = String::from("\n\n");
            result.push_str(&join_children(paragraph.children));
            Some(result)
        }

        mdast::Node::Text(text) => Some(text.value),
        mdast::Node::Heading(heading) => {
            let mut result = String::from("\n");
            result.push_str(&join_children_with(
                |s| s.bold().blue().to_string(),
                heading.children,
            ));
            Some(result)
        }
        mdast::Node::Emphasis(emphasis) => Some(join_children_with(
            |s| s.italic().to_string(),
            emphasis.children,
        )),

        mdast::Node::Strong(strong) => Some(join_children_with(
            |s| s.bold().to_string(),
            strong.children,
        )),

        mdast::Node::Link(link) => {
            let mut result = "[".purple().to_string();
            result.push_str(&join_children(link.children).bright_green());
            result.push_str(&"](".purple().to_string());
            result.push_str(&link.url.bright_cyan().to_string());
            result.push_str(&")".purple().to_string());
            Some(result)
        }

        mdast::Node::Code(code) => {
            let mut result = "`\n".to_string();
            result.push_str(&code.value.purple().to_string());
            result.push_str(&"`".to_string());
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
