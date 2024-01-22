use colored::*;
use markdown::mdast::{self, List};
use termion::color;

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
            let mut result = String::default();
            result.push_str(&join_children(root.children));
            result.push('\n');
            Some(result)
        }

        mdast::Node::Paragraph(paragraph) => {
            let mut result = String::from("\n");
            result.push_str(&join_children(paragraph.children));
            result.push_str("\n\n");
            Some(result)
        }

        mdast::Node::Text(text) =>  Some(text.value.replace(" ", " ")),
        
        mdast::Node::Heading(heading) => {
            let level = heading.depth;
            let mut result = String::from("\n");
            match level {
                1 => result.push_str(&join_children_with(|s| s.bold().red().to_string(), heading.children)),
                2 => result.push_str(&join_children_with(|s| s.bold().yellow().to_string(), heading.children)),
                3 => result.push_str(&join_children_with(|s| s.bold().green().to_string(), heading.children)),
                4 => result.push_str(&join_children_with(|s| s.bold().cyan().to_string(), heading.children)),
                5 => result.push_str(&join_children_with(|s| s.bold().blue().to_string(), heading.children)),
                6 => result.push_str(&join_children_with(|s| s.bold().purple().to_string(), heading.children)),
                _ => result.push_str(&join_children(heading.children)),
            }
            result.push('\n');
            Some(result)
        }

        mdast::Node::Code(code) => {
            let mut result = String::from("```\n").replace("```", "");
            result.push_str(&code.value.on_black().white().to_string());
            result.push_str("\n```\n".replace("```", "").as_str());
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

        mdast::Node::ThematicBreak(_) => {
            Some("\n---\n".to_string())
        }

        mdast::Node::BlockQuote(blockquote) => {
            let mut result = String::from(">").replace(">", "\n");
            
            result.push_str("   >");
            result.push_str(&join_children(blockquote.children).replace("\n", &format!("{}", color::Fg(color::LightBlack))));

        
            result.push_str(&format!("{}", color::Fg(color::Reset)));
            result.push('\n');
        
            Some(result)
        }       

        mdast::Node::List(list) => {
    if list.ordered {
        let mut result = String::new();
        let mut item_number = list.start.unwrap_or(1);
        result.push_str("\n");

        for item in list.children {
            let mut item_text = String::new();
            item_text.push_str(&format!("{}.", item_number).bright_green().to_string());

            if let mdast::Node::ListItem(list_item) = item {
                for child in list_item.children {
                    if let mdast::Node::Paragraph(paragraph) = child {
                        item_text.push_str(&join_children(paragraph.children));
                    }
                    // Handle other types of Nodes here
                }
            }

            item_text.push('\n');
            result.push_str(&item_text);
            item_number += 1;
        }

        result.push('\n');

        Some(result)
    } else {
        None
    }
}
 

        _ => None,
    }
}

pub fn parse_front_matter(front_matter: &[String]) {
    for child in front_matter.iter() {
        let mut result = String::new();
        result.push_str(&child);
        result.push('\n');
        println!("{}", result);
    }

}

pub fn prettify(md_text: &str) -> Result<String, String> {
    let mut lines = md_text.lines();
    let mut front_matter = Vec::new();

    if let Some(first_line) = lines.next() {
        if first_line == "---" {
            while let Some(line) = lines.next() {
                if line == "---" {
                    break;
                }
                front_matter.push(line.to_string());
            }
        }
    }
    // parse_front_matter(&front_matter);

    
    let md_text = lines.collect::<Vec<&str>>().join("\n");
    let parsed = markdown::to_mdast(&md_text, &markdown::ParseOptions::default());
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
