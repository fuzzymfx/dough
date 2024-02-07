use paris::Logger;
use regex::Regex;
use std::collections::HashMap;

pub fn calculate_length_of_longest_line(prettified: &String) -> usize {
    let lines: Vec<&str> = prettified.split('\n').collect();

    let lines_clone = lines.clone();

    // Calculate the length of the longest line
    let max_length = lines_clone
        .iter()
        .map(|s| {
            let leading_spaces = strip_ansi_codes(s)
                .chars()
                .take_while(|c| *c == ' ')
                .count();

            let s = strip_ansi_codes(s).replace("̶", "");
            s.chars().count() + leading_spaces
        })
        .max()
        .unwrap_or(0);

    max_length
}

pub fn store_colors(prettified: &Vec<String>) -> HashMap<usize, String> {
    let mut colors: HashMap<usize, String> = HashMap::new();
    let mut current_color = String::from("\x1b[0m"); // Initialize current color as default color
    let mut line_num = 0;

    let color_regex = Regex::new(r"\x1b\[\d+(;\d+)?m").unwrap(); // Regex to match color codes

    for line in prettified.iter() {
        let line_color = match color_regex.find(line) {
            Some(mat) => String::from(mat.as_str()),
            None => String::from("\x1b[0m"),
        };

        if line.trim().is_empty() {
            // If the line is empty, reset the current color to default
            current_color = String::from("\x1b[0m");
        } else if line_color != "\x1b[0m" {
            // If the color of the current line is not default, update the current color
            current_color = line_color;
        }

        // Store the current color for the current line number
        colors.insert(line_num, current_color.clone());

        line_num += 1; // Increment line number for the next iteration
    }

    colors
}

pub fn strip_ansi_codes(line: &str) -> String {
    let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    let result = re.replace_all(line, "");
    result.to_string()
}

pub fn remove_last_n_lines(text: &str, n: u32) -> String {
    let mut lines: Vec<&str> = text.lines().collect();

    // Check if there is at least one line
    if lines.is_empty() {
        return String::from("");
    } else {
        for _ in 0..n {
            lines.pop();
        }
    }
    return lines.join("\n");
}

pub fn remove_comments(text: &str) -> String {
    let re = Regex::new(r"(?s)<!--.*?-->").unwrap();
    let result = re.replace_all(text, "");
    result.to_string()
}

pub fn create_style(project: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut log = Logger::new();
    let style_path = project.join("style.yml");
    let verify_path = style_path.clone();

    if !style_path.exists() {
        std::fs::write(style_path, "
# This file contains the default style settings for the terminal markdown renderer.
# Markdown styles
h1: red
h2: yellow
h3: green
h4: cyan
h5: blue
h6: purple
code: black on white
blockquote: black on white
ordered_list_bullet: yellow
unordered_list_bullet: yellow
ordered_list: white
unordered_list: white
link_text: black
link_url: blue
thematic_break: white on black
        
# Terminal styles

# clear will clear the terminal before rendering, you would need to scroll down to render each line
clear: false

box: true
box_color: black on white

# vertical_alignment will vertically align the text to the middle of the terminal
vertical_alignment: true

# horizontal_alignment will horizontally align the text to the middle of the terminal
horizontal_alignment: true

# syntax_highlighting will highlight the code syntax
# this works well with the warp terminal, but not with the default Mac OS terminal

syntax_highlighting: true
syntax_theme: base16-ocean.light
#themes:[base16-ocean.dark,base16-eighties.dark,base16-mocha.dark,base16-ocean.light, Solarized (dark) and Solarized (light)]
syntax_bg: false

progress: true

# runtime map is used to store the runtimes for different languages
# you can add your own runtimes for different languages. Currently, the following runtimes are supported:

-runtime_map:
  python: python3
  sh: bash
  bash: bash
  javascript: node
  typescript: node
  ts: tsc
  c: gcc
  cpp: g++
  java: javac
  go: go run
  rust: cargo run
  ruby: ruby
  php: php
  swift: swift
  kotlin: kotlinc
")?;
        if verify_path.exists() {
            log.info("fin style.yml");
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create style.yml",
            )))
        }
    } else {
        log.warn("style.yml exists. Skipped.");
        Ok(())
    }
}
