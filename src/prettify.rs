extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

use colored::*;
use markdown::mdast::{self};
// use termion::color;
use regex::Regex;
// use termion::style;


use lazy_static::lazy_static;

lazy_static! {
    static ref STYLES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

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
            let text_start = &join_children(paragraph.children.clone());
            let mut result = String::from("\n");

            let re = Regex::new(r"~~(.*?)~~").unwrap();
            if re.is_match(text_start) {
                for cap in re.captures_iter(text_start) {
                    let matched_text = &cap[1];
                    let strikethrough_text = matched_text
                        .chars()
                        .map(|c| format!("{}{}", c, '\u{0336}'))
                        .collect::<String>();
                    let text_to_replace = format!("~~{}~~", matched_text);
                    let replaced_text = text_start.replace(&text_to_replace, &strikethrough_text);
                    result.push_str(&replaced_text);
                }
            } else {
                result.push_str(text_start);
            }

            result.push('\n');
            Some(result)
        }

        mdast::Node::Text(text) => Some(text.value),

        mdast::Node::Heading(heading) => {
            let level = heading.depth;
            let mut result = String::from("\n");
            match level {
                1 => result.push_str(&join_children_with(
                    |s| s.bold().red().to_string(),
                    heading.children,
                )),
                2 => result.push_str(&join_children_with(
                    |s| s.bold().yellow().to_string(),
                    heading.children,
                )),
                3 => result.push_str(&join_children_with(
                    |s| s.bold().green().to_string(),
                    heading.children,
                )),
                4 => result.push_str(&join_children_with(
                    |s| s.bold().cyan().to_string(),
                    heading.children,
                )),
                5 => result.push_str(&join_children_with(
                    |s| s.bold().blue().to_string(),
                    heading.children,
                )),
                6 => result.push_str(&join_children_with(
                    |s| s.bold().purple().to_string(),
                    heading.children,
                )),
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

        mdast::Node::ThematicBreak(_) => Some("\n---\n".to_string()),

        mdast::Node::BlockQuote(blockquote) => {
            let mut result = String::from(">").replace(">", "");
            result.push_str(
                &join_children(blockquote.children)
                    .on_white()
                    .black()
                    .to_string(),
            );
            result.push('\n');

            Some(result)
        }

        mdast::Node::List(list) => {
            let mut result = String::new();
            let mut item_number = list.start.unwrap_or(1);
            result.push_str("\n");

            for item in list.children {
                let mut item_text = String::new();
                if list.ordered {
                    item_text.push_str(&format!(" {}. ", item_number).bright_green().to_string());
                } else {
                    item_text.push_str(&format!(" • ").bright_green().to_string());
                }

                if let mdast::Node::ListItem(list_item) = item {
                    for child in list_item.children {
                        if let mdast::Node::Paragraph(paragraph) = child {
                            item_text.push_str(&join_children(paragraph.children));
                        } else {
                            item_text.push_str(&join_children(vec![child]));
                        }
                    }
                }

                item_text.push('\n');
                result.push_str(&item_text);
                item_number += 1;
            }

            result.push('\n');

            Some(result)
        }

        _ => None,
    }
}

// pub fn parse_front_matter(front_matter: &[String]) {
//     for child in front_matter.iter() {
//         let mut result = String::new();
//         result.push_str(&child);
//         result.push('\n');
//         println!("{}", result);
//     }
// }

pub fn prettify(md_text: &str, style_map: HashMap< String, String>) -> Result<String, String> {

    let mut global_styles = STYLES.lock().unwrap();
    *global_styles = style_map;

    print!("{:?}", global_styles);

    let mut lines = md_text.lines();
    let mut front_matter = Vec::new();

    let mut first_line = lines.next();

    if let Some(line) = first_line {
        if line == "---" {
            while let Some(line) = lines.next() {
                if line == "---" {
                    break;
                } else {
                    front_matter.push(line.to_string());
                }
            }
            first_line = lines.next();
        }
    }

    let md_text = if let Some(line) = first_line {
        // If there are lines left, join them and add a newline at the end
        std::iter::once(line)
            .chain(lines)
            .collect::<Vec<&str>>()
            .join("\n")
            + "\n"
    } else {
        // If there are no lines left, return an empty string
        String::new()
    };

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
