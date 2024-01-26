extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

use colored::*;
use markdown::mdast::{self};
use regex::Regex;

use lazy_static::lazy_static;
// use termion::style;

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
    let style_map = STYLES.lock().unwrap();

    let styles = style_map.clone();

    drop(style_map);

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

            let color: &str;
            let mut item_text = String::new();

            match level {
                1 => {
                    color = styles.get("h1").map(|s| s.as_str()).unwrap_or("red");
                    item_text.push_str(
                        &format!("{}", join_children(heading.children))
                            .color(color)
                            .to_string(),
                    );
                    result.push_str(&item_text);
                }
                2 => {
                    color = styles.get("h2").map(|s| s.as_str()).unwrap_or("yellow");
                    item_text.push_str(
                        &format!("{}", join_children(heading.children))
                            .color(color)
                            .to_string(),
                    );
                    result.push_str(&item_text);
                }
                3 => {
                    color = styles.get("h3").map(|s| s.as_str()).unwrap_or("green");
                    item_text.push_str(
                        &format!("{}", join_children(heading.children))
                            .color(color)
                            .to_string(),
                    );
                    result.push_str(&item_text);
                }
                4 => {
                    color = styles.get("h4").map(|s| s.as_str()).unwrap_or("blue");
                    item_text.push_str(
                        &format!("{}", join_children(heading.children))
                            .color(color)
                            .to_string(),
                    );
                    result.push_str(&item_text);
                }
                5 => {
                    color = styles.get("h5").map(|s| s.as_str()).unwrap_or("magenta");
                    item_text.push_str(
                        &format!("{}", join_children(heading.children))
                            .color(color)
                            .to_string(),
                    );
                    result.push_str(&item_text);
                }

                6 => {
                    color = styles.get("h6").map(|s| s.as_str()).unwrap_or("cyan");
                    item_text.push_str(
                        &format!("{}", join_children(heading.children))
                            .color(color)
                            .to_string(),
                    );
                    result.push_str(&item_text);
                }
                _ => result.push_str(&join_children(heading.children)),
            }
            result.push('\n');
            Some(result)
        }

        mdast::Node::Code(code) => {
            let color: &str = styles.get("code").map(|s| s.as_str()).unwrap_or("white");
            let mut result = String::from("```\n").replace("```", "");
            result.push_str(&code.value.on_black().color(color).to_string());
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
            let bullet_color: &str = match list.ordered {
                true => styles
                    .get("ordered_list_bullet")
                    .map(|s| s.as_str())
                    .unwrap_or("green"),
                false => styles
                    .get("unordered_list_bullet")
                    .map(|s| s.as_str())
                    .unwrap_or("green"),
            };

            let text_color: &str = match list.ordered {
                true => styles
                    .get("ordered_list")
                    .map(|s| s.as_str())
                    .unwrap_or("blue"),
                false => styles
                    .get("unordered_list")
                    .map(|s| s.as_str())
                    .unwrap_or("blue"),
            };

            let mut result = String::new();
            let mut item_number = list.start.unwrap_or(1);
            result.push_str("\n");

            for item in list.children {
                let mut item_text = String::new();
                if list.ordered {
                    item_text.push_str(
                        &format!(" {}. ", item_number)
                            .color(bullet_color)
                            .to_string(),
                    );
                } else {
                    item_text.push_str(&format!(" • ").color(bullet_color).to_string());
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
                result.push_str(&item_text.color(text_color).to_string());
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

pub fn prettify(md_text: &str, style_map: &HashMap<String, String>) -> Result<String, String> {
    let map = style_map.clone();
    let mut global_styles = STYLES.lock().unwrap();
    *global_styles = map;
    drop(global_styles);

    let mut lines = md_text.lines();
    // let mut front_matter = Vec::new();

    let mut first_line = lines.next();

    // if let Some(line) = first_line {
    //     if line == "---" {
    //         while let Some(line) = lines.next() {
    //             if line == "---" {
    //                 break;
    //             } else {
    //                 front_matter.push(line.to_string());
    //             }
    //         }
    //         first_line = lines.next();
    //     }
    // }

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
    // Add the required number of blank lines
    let mut blank_lines = 0;
    let (_width, height) = termion::terminal_size().unwrap();

    if style_map.get("vertical_alignment").unwrap() == "false" {
        blank_lines = 0;
    } else {
        if height > prettified.lines().count() as u16 {
            blank_lines = (height - prettified.lines().count() as u16) as u32 / 2;
        } else {
            blank_lines = 0;
        }
    }
    if let Some(terminal_style) = style_map.get("terminal") {
        if terminal_style == "warp" {
            // In case of "warp", add blank lines at the end
            for _ in 0..blank_lines - 2 {
                prettified.push('\n');
            }
        } else {
            // In all other cases, add blank lines at the beginning
            let mut new_prettified = String::new();
            if blank_lines > 2 {
                for _ in 0..blank_lines - 2 {
                    new_prettified.push('\n');
                }
                new_prettified.push_str(&prettified);
                prettified = new_prettified;
            }
        }
    }

    return Ok(prettified);
}
