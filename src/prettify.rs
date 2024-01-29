extern crate lazy_static;
use crate::utils::{calculate_length_of_longest_line, store_colors, strip_ansi_codes};

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
            let color_url = styles
                .get("link_url")
                .map(|s| s.as_str())
                .unwrap_or("green");
            let color_text = styles
                .get("link_text")
                .map(|s| s.as_str())
                .unwrap_or("blue");

            let mut result = String::from("[");
            result = result.replace("[", "");

            result.push_str(&join_children(link.children).color(color_text).to_string());

            result.push_str(" :(");
            result.push_str(&link.url.color(color_url).to_string());
            result.push(')');
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

pub fn draw_box(content: &str, line_color_map: &HashMap<usize, String>) -> String {
    let lines: Vec<&str> = content.split('\n').collect();

    let max_length = lines
        .iter()
        .map(|s| strip_ansi_codes(s).len())
        .max()
        .unwrap_or(0);

    let horizontal_border: String = "─".repeat(max_length + 6); // 6 for box corners and sides
    let mut boxed_content = format!("┌{}┐\n", strip_ansi_codes(&horizontal_border)); // top border

    for (i, line) in lines.iter().enumerate() {
        let original_color = match line_color_map.get(&i) {
            Some(color) => color,
            None => "\x1B[0m",
        };

        let padding_length = if strip_ansi_codes(line).contains("•") {
            (max_length - strip_ansi_codes(line).len()) + 4 // +4 to ensure space at the end and after bullet
        } else {
            (max_length - strip_ansi_codes(line).len()) + 2 // +2 to ensure space at the end
        };

        let padding = " ".repeat(padding_length);

        boxed_content.push_str(&format!(
            "│{}{}{}{}    │\n",
            "\x1B[0m", original_color, line, padding
        )); // content with side borders
    }

    boxed_content.push_str(&format!("└{}┘\n", strip_ansi_codes(&horizontal_border))); // bottom border

    boxed_content
}

pub fn align_vertical(
    mut prettified: String,
    style_map: &HashMap<String, String>,
    height: u16,
) -> String {
    let blank_lines;

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
            if blank_lines > 2 {
                for _ in 0..blank_lines - 2 {
                    prettified.push('\n');
                }
            } else {
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
    return prettified;
}

pub fn align_horizontal(
    prettified: String,
    style_map: &HashMap<String, String>,
    width: u16,
    text: &str,
    line_color_map: HashMap<usize, String>,
) -> String {
    let blank_chars;

    if style_map.get("horizontal_alignment").unwrap() == "false" {
        blank_chars = 0;
    } else {
        let longest_line = calculate_length_of_longest_line(text.to_string());

        if width > longest_line as u16 {
            blank_chars = (width - longest_line as u16) as usize / 2;
        } else {
            blank_chars = 0;
        }
    }

    let mut new_prettified = String::new();

    if blank_chars > 0 {
        // for each line, add blank_chars spaces at the beginning
        let reset_colored_line_ref: &str = "\x1B[0m";
        for (i, line) in prettified.lines().enumerate() {
            let original_color = match line_color_map.get(&i) {
                Some(color) => color,
                None => reset_colored_line_ref,
            };
            let new_line = format!(
                "{}{}{}{}",
                " ".repeat(blank_chars),
                original_color,
                line,
                "\x1B[0m"
            );
            new_prettified.push_str(&new_line);
            new_prettified.push('\n'); // Add newline after each line
        }
        return new_prettified; // Return the modified string
    }

    return prettified; // Return the original string if no alignment needed
}

pub fn align_content(
    mut prettified: String,
    style_map: &HashMap<String, String>,
    lines: &str,
) -> String {
    let (_width, height) = termion::terminal_size().unwrap();

    let mut content_lines: Vec<String> = prettified.lines().map(|s| s.to_string()).collect();
    let mut line_color_map = store_colors(&content_lines);

    if style_map.get("box").unwrap() == "true" {
        prettified = draw_box(&prettified, &line_color_map);
    }
    content_lines = prettified.lines().map(|s| s.to_string()).collect();
    line_color_map = store_colors(&content_lines);
    if style_map.get("horizontal_alignment").unwrap() == "true" {
        prettified = align_horizontal(prettified, style_map, _width, lines, line_color_map);
    }
    if style_map.get("vertical_alignment").unwrap() == "true" {
        prettified = align_vertical(prettified, style_map, height);
    }

    return prettified;
}

pub fn prettify(md_text: &str, style_map: &HashMap<String, String>) -> Result<String, String> {
    let map = style_map.clone();
    let mut global_styles = STYLES.lock().unwrap();
    *global_styles = map;
    drop(global_styles);

    let mut lines = md_text.lines();
    // let mut front_matter = Vec::new();

    let first_line = lines.next();

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
    let lines: Vec<String> = prettified.lines().map(|s| s.to_string()).collect();
    let line_color_map = store_colors(&lines);

    return Ok(align_content(prettified, style_map, &md_text));
}
