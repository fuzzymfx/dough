use regex::Regex;
use std::collections::HashMap;

pub fn calculate_length_of_longest_line(prettified: String) -> usize {
    let mut longest_line = 0;

    for line in prettified.lines() {
        print!("{:?}\n", line);
        print!("{:?}\n", line.len());
        if line.len() > longest_line {
            longest_line = line.len();
        }
    }

    longest_line
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
