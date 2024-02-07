/// Ramen is a simple code runner for various languages.
/// It is designed to be used in a terminal env to run code in a sandboxed environment.
/// It takes in the code and the language and returns the output of the code.
/// The runtimes are defined in a separare environment variable, dependent on the host system.
use std::collections::HashMap;
use std::io::Write;

pub fn run_code(
    lang: String,
    code: String,
    runtime_map: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let output = match lang.as_str() {
        "python" | "py" => run_python_code(code, runtime_map.get("python").unwrap().as_str()),
        "javascript" | "js" => {
            run_javascript_code(code, runtime_map.get("javascript").unwrap().as_str())
        }
        "ruby" | "rb" => run_ruby_code(code, runtime_map.get("ruby").unwrap().as_str()),
        "c" | "c++" | "cpp" => run_c_code(code, runtime_map.get("c").unwrap().as_str()),
        "java" => run_java_code(code, runtime_map.get("java").unwrap().as_str()),
        "rust" => run_rust_code(code, runtime_map.get("rust").unwrap().as_str()),
        _ => Err("Language not supported".to_string())?,
    };
    output
}

pub fn run_python_code(code: String, runtime: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("temp.py")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new(runtime)
        .arg("temp.py")
        .output()?;

    std::fs::remove_file("temp.py")?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_javascript_code(
    code: String,
    runtime: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("temp.js")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new(runtime)
        .arg("temp.js")
        .output()?;

    std::fs::remove_file("temp.js")?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_ruby_code(code: String, runtime: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("temp.rb")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new(runtime)
        .arg("temp.rb")
        .output()?;

    std::fs::remove_file("temp.rb")?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_c_code(code: String, runtime: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("temp.c")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new(runtime)
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

pub fn run_java_code(code: String, runtime: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("Main.java")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new(runtime)
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

pub fn run_rust_code(code: String, runtime: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("temp.rs")?;
    file.write_all(code.as_bytes())?;

    let output = std::process::Command::new(runtime)
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
