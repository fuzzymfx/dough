mod prettify;
extern crate termion;

use paris::Logger;
use std::error::Error;
use std::fmt;
use std::fs;

use std::collections::HashMap;
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
    ScrollUp,
    ScrollDown,
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
            fs_path: std::path::Path::new(&project_dir).join(name).to_path_buf(),
            template: std::path::Path::new(workdir)
                .join("templates")
                .join(template)
                .to_path_buf(),
        }
    }

    pub fn init_project(self: &Self) -> Result<()> {
        let mut log = Logger::new();

        let create_dir_result = fs::create_dir(&self.fs_path);
        if create_dir_result.is_err() {
            return create_dir_result;
        }

        if !self.template.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Template not found",
            ));
        } else if !self.template.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Template is not a directory",
            ));
        } else if !self.template.join("style.yml").exists() {
            log.warn("Template does not contain a style.yml file, using default style");

            fs::copy(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("templates")
                    .join("default")
                    .join("style.yml"),
                self.template.join("style.yml"),
            )?;
        }

        fs::copy(
            self.template.join("style.yml"),
            self.fs_path.join("style.yml"),
        )?;

        for i in 1u64.. {
            let file_path = self.template.join(i.to_string() + ".md");
            if !file_path.exists() {
                break;
            }
            let output_path = self.fs_path.join(i.to_string() + ".md");
            fs::copy(file_path, output_path)?;
        }

        return Ok(());
    }

    fn remove_last_n_lines(text: &str, n: u32) -> String {

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

    fn render_term(
        file_contents: &str,
        style_map: &HashMap<String, String>,
        render: bool,
        lines: &u32,

    ) ->std::result::Result<(NavigationAction, u32), Box<dyn Error>>  {

        // let (width, height) = termion::terminal_size()?;

        // TODO: ADD FEATURE TO RENDER A FRAME OVER THE RENDERING AREA

        //use this to draw a frame over the rendering area.
        //use the style map to determine the color of the frame, and the position of the frame: center, left, right, top, bottom
        // subtract the cordinates of the frame from the width and height of the terminal to get the rendering area

        let mut clear: bool = false;

        if style_map.get("clear").unwrap() == "true" {
            clear = true;
        }

        let slide = prettify::prettify(file_contents, &style_map)?;
        
        let mut lines_value = *lines;

        if (slide.lines().count() as u32) < lines_value {

            //DEBUG
            // print!("\n{}\n", lines_value.to_string());
            // print!("{}\n", slide.lines().count() as u32);

            lines_value = slide.lines().count() as u32;
        }
        
        if render {
            if clear {
                lines_value = slide.lines().count() as u32;
                print!(
                    "{}",
                    Self::remove_last_n_lines(&slide, lines_value)
                );
            } else {
                lines_value =0;
                print!("{}", slide);
            }
        } else {
            print!("{}", Self::remove_last_n_lines(&slide, lines_value));
        }
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode()?;
        if render {
            stdout.flush()?;
        }

        for c in stdin.keys() {
            match c? {
                Key::Right | Key::Char('l') | Key::Char('L') => return Ok((NavigationAction::Next, lines_value)),
                Key::Left | Key::Char('h') | Key::Char('H') => {
                    return Ok((NavigationAction::Previous, lines_value));
                }
                //add escape and ctrl + c here
                
                Key::Char('q') | Key::Char('Q') => return Ok((NavigationAction::Exit, lines_value)), 
                Key::Up => return Ok((NavigationAction::ScrollUp, lines_value)),
                Key::Down => return Ok((NavigationAction::ScrollDown, lines_value)),
                _ => continue,
            }
        }

        drop(stdout);

        return Ok((NavigationAction::None, lines_value));

    }

    fn clear() {
        let mut stdout = stdout();
        write!(stdout, "\x1B[2J\x1B[1;1H").unwrap();
    }

    pub fn present_term(self: &Self) -> std::result::Result<(), Box<dyn Error>> {
        let mut render = true;
        let mut current_slide = 1;
        let mut lines: u32 = 0;


        loop {
            Self::clear();
            print!("{}", termion::cursor::Hide);
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
            let style_content = fs::read_to_string(self.fs_path.join("style.yml"))?;
            let style_map: HashMap<String, String> = style_content
                .lines()
                .filter_map(|line| {
                    let mut parts = line.splitn(2, ':');
                    Some((
                        parts.next()?.trim().to_string(),
                        parts.next()?.trim().to_string(),
                    ))
                })
                .collect();

            match Self::render_term(&contents, &style_map, render, &lines)? {
                (NavigationAction::Next, _new_lines_value) => {
                    render = true;
                    current_slide += 1;
                }
                (NavigationAction::Previous, _new_lines_value) => {
                    render = true;
                    if current_slide > 1 {
                        current_slide -= 1;
                    }
                }
                (NavigationAction::ScrollUp, new_lines_value) => {
                    render = false;
                    lines = new_lines_value;
                    lines += 1;
                }
                (NavigationAction::ScrollDown, new_lines_value) => {
                    render = false;
                    lines = new_lines_value;
                    if lines > 0 {
                        lines -= 1;
                    }
                }
                (NavigationAction::Exit, _new_lines_value) => {
                    
                    exit(0);
                }
                (NavigationAction::None, _new_lines_value) => {
                }
            }
        }
    }
}
