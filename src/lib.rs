mod prettify;

extern crate termion;

use std::error::Error;
use std::fmt;
use std::fs;

use std::io::{stdin, stdout, Result, Write};
use std::process::exit;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

enum NavigationAction {
    Next,
    Previous,
    Exit,
    None,
}

pub struct Project {
    fs_path: std::path::PathBuf,
    template: std::path::PathBuf,
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
    pub fn new(name: &str, workdir: &str, template: &str) -> Project {
        let project_dir = workdir.to_owned() + "/projects";

        if !std::path::Path::new(&project_dir).exists() {
            fs::create_dir(&project_dir).expect("Could not create projects directory");
        }

        Project {
            fs_path: std::path::Path::new(& project_dir).join(name).to_path_buf(),
            template: std::path::Path::new(workdir)
                .join("templates")
                .join(template)
                .to_path_buf(),
        }
    }

    pub fn init_project(self: &Self) -> Result<()> {
        let create_dir_result = fs::create_dir(&self.fs_path);
        if create_dir_result.is_err() {
            return create_dir_result;
        }

        let md_path = self.fs_path.join("1.md");
        let md_content = fs::read_to_string(self.template.join("template.md"))?;
        return fs::write(md_path, md_content);
    }

    fn render_term(file_contents: &str) -> std::result::Result<NavigationAction, Box<dyn Error>> {
        let slide = prettify::prettify(file_contents)?;
        print!("{}", slide);
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode()?;
        stdout.flush()?;

        for c in stdin.keys() {
            match c? {
                Key::Right | Key::Char('l') | Key::Char('L') => return Ok(NavigationAction::Next),
                Key::Left | Key::Char('h') | Key::Char('H') => return Ok(NavigationAction::Previous),
                Key::Char('q') | Key::Char('Q') => return Ok(NavigationAction::Exit),
                _ => continue,
            }
        }

        drop(stdout);

        Ok(NavigationAction::None)
    }

    // fn render_html(file_contents: &str) -> std::result::Result<(), Box<dyn Error>> {
    //     let slide = prettify::prettify(file_contents)?;
    //     print!("{}", slide);
    //     let stdin = stdin();
    //     let mut stdout = stdout().into_raw_mode()?;
    //     stdout.flush().unwrap();

    //     // wait for a keypress
    //     for c in stdin.keys() {
    //         match c? {
    //             Key::Left => break,
    //             Key::Right => break,
    //             Key::Char('q') => {
    //                 exit(0);
    //             }
    //             _ => {}
    //         }
    //     }

    //     drop(stdout);

    //     // let md_ast = markdown::to_mdast(file_contents, &markdown::ParseOptions::default());

    //     return Ok(());
    // }

    fn clear() {
        let mut stdout = stdout();
        write!(stdout, "\x1B[2J\x1B[1;1H").unwrap();
    }

    pub fn present_term(self: &Self) -> std::result::Result<(), Box<dyn Error>> {
        let mut current_slide = 1;

        loop {
            Self::clear();
            let file_path = self.fs_path.join(format!("{}.md", current_slide));

            if !file_path.exists() {
                if current_slide == 1 {
                    return Err(Box::new(DoughError(
                        "No slides found in the project".into(),
                    )));
                } else {
                    exit(0)
                }
            }

            let contents = fs::read_to_string(&file_path)?;
            match Self::render_term(&contents)? {
                NavigationAction::Next => {
                    current_slide += 1;
                }
                NavigationAction::Previous => {
                    if current_slide > 1 {
                        current_slide -= 1;
                    }
                }
                NavigationAction::Exit => {
                    exit(0);
                }
                NavigationAction::None => {}
            }
        }
    }

    // pub fn present_html(self: &Self) -> std::result::Result<(), Box<dyn Error>> {
    //     for i in 1u64.. {
    //         Self::clear();
    //         let file_path =
    //             std::path::Path::new(self.fs_path.as_path()).join(i.to_string() + ".md");
    //         if !file_path.exists() {
    //             return Ok(());
    //         }

    //         let read_result = fs::read_to_string(&file_path);
    //         if let Ok(contents) = read_result {
    //             _ = Self::render_html(&contents);
    //             continue;
    //         }

    //         return Err(Box::new(DoughError(format!(
    //             "Could not read file '{}'",
    //             file_path.to_str().unwrap()
    //         ))));
    //     }
    //     return Ok(());
    // }
}
