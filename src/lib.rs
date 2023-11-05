extern crate termion;

use std::error::Error;
use std::fmt;
use std::fs;

use std::io::{stdin, stdout, Result, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Project {
    fs_path: std::path::PathBuf,
}

// Define a custom error type.
#[derive(Debug)]
struct DoughError(String);

// Implement `Display` for `CustomError`.
impl fmt::Display for DoughError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom Error: {}", self.0)
    }
}

// Implement `Error` for `CustomError`.
impl std::error::Error for DoughError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl Project {
    pub fn new(name: &str, workdir: &str) -> Project {
        Project {
            fs_path: std::path::Path::new(workdir).join(name).to_path_buf(),
        }
    }

    pub fn init_project(self: &Self) -> Result<()> {
        let create_dir_result = fs::create_dir(&self.fs_path);
        if create_dir_result.is_err() {
            return create_dir_result;
        }

        let md_path = self.fs_path.join("1.md");
        let md_content = "# Hello, world!";
        return fs::write(md_path, md_content);
    }

    fn render(file_contents: &str) -> std::result::Result<(), Box<dyn Error>> {
        print!("{}", file_contents);
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode()?;
        stdout.flush().unwrap();

        // wait for a keypress
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Left => break,
                _ => {}
            }
        }

        drop(stdout);

        // let md_ast = markdown::to_mdast(file_contents, &markdown::ParseOptions::default());

        return Ok(());
    }

    fn clear() {
        let mut stdout = stdout();
        write!(stdout, "\x1B[2J\x1B[1;1H").unwrap();
    }

    pub fn present(self: &Self) -> std::result::Result<(), Box<dyn Error>> {
        for i in 1u64.. {
            Self::clear();
            let file_path =
                std::path::Path::new(self.fs_path.as_path()).join(i.to_string() + ".md");
            if !file_path.exists() {
                return Ok(());
            }

            let read_result = fs::read_to_string(&file_path);
            if let Ok(contents) = read_result {
                _ = Self::render(&contents);
                continue;
            }

            return Err(Box::new(DoughError(format!(
                "Could not read file '{}'",
                file_path.to_str().unwrap()
            ))));
        }
        return Ok(());
    }
}
