use regex::Regex;
use std::collections::HashMap;
use std::io::Write;

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

pub fn run_code(lang: String, code: String) -> Result<String, Box<dyn std::error::Error>> {
    let output = match lang.as_str() {
        "python" | "py" => run_python_code(code),
        "javascript" | "js" => run_javascript_code(code),
        "ruby" | "rb" => run_ruby_code(code),
        "c" | "c++" | "cpp" => run_c_code(code),
        "java" => run_java_code(code),
        "rust" => run_rust_code(code),
        _ => Ok(String::from("")),
    };
    output
}

pub fn run_python_code(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("temp.py")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new("python3")
        .arg("temp.py")
        .output()?;

    std::fs::remove_file("temp.py")?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_javascript_code(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("temp.js")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new("node").arg("temp.js").output()?;

    std::fs::remove_file("temp.js")?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_ruby_code(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("temp.rb")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new("ruby").arg("temp.rb").output()?;

    std::fs::remove_file("temp.rb")?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_c_code(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("temp.c")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new("gcc")
        .arg("temp.c")
        .arg("-o")
        .arg("temp")
        .output()?;

    if output.status.success() {
        let output = std::process::Command::new("./temp").output()?;
        std::fs::remove_file("temp.c")?;
        std::fs::remove_file("temp")?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        std::fs::remove_file("temp.c")?;
        std::fs::remove_file("temp")?;
        Ok(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn run_java_code(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("Main.java")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new("javac")
        .arg("Main.java")
        .output()?;

    if output.status.success() {
        let output = std::process::Command::new("java").arg("Main").output()?;
        std::fs::remove_file("Main.java")?;
        std::fs::remove_file("Main.class")?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        std::fs::remove_file("Main.java")?;
        std::fs::remove_file("Main.class")?;
        Ok(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn run_rust_code(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("temp.rs")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new("rustc")
        .arg("temp.rs")
        .output()?;

    if output.status.success() {
        let output = std::process::Command::new("./temp").output()?;
        std::fs::remove_file("temp.rs")?;
        std::fs::remove_file("temp")?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        std::fs::remove_file("temp.rs")?;
        std::fs::remove_file("temp")?;
        Ok(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
