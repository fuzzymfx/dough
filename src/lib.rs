mod prettify;
extern crate termion;
mod ramen;
mod utils;
use crate::ramen::run_code;
use crate::utils::{remove_comments, remove_last_n_lines};

use std::error::Error;
use std::fmt;
use std::fs;
use std::thread;

use paris::Logger;

use std::collections::HashMap;
use std::io::{stdin, stdout, Result, Write};
use std::process::exit;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// All the possible navigation actions when presenting a project.
enum NavigationAction {
    Next,
    Previous,
    Exit,
    None,
    ScrollUp,
    ScrollDown,
    Refresh,
    ToggleHighlight,
}

// Define a struct to hold project information.
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
        write!(f, "{}", self.0)
    }
}

// Implement `Error` for `CustomError`.
impl std::error::Error for DoughError {
    fn description(&self) -> &str {
        &self.0
    }
}

// Implement `Project` methods.
impl Project {
    /// Create a new project.
    /// # Arguments
    /// * `name` - The name of the project.
    /// * `workdir` - The working directory of the project.
    /// * `template` - The template to use for the project.
    /// # Returns
    /// A new project instance.

    pub fn new(name: &str, workdir: &str, template: &str) -> Project {
        // Use the template provided to copy the template files into the project directory.
        Project {
            fs_path: std::path::Path::new(&workdir).join(name).to_path_buf(),
            template: std::path::Path::new(workdir)
                .join("templates")
                .join(template)
                .to_path_buf(),
        }
    }
    /// Initialize a project.
    /// # Arguments
    /// * `self` - The project instance.
    /// # Returns
    /// A result indicating whether the project was initialized successfully or not.

    pub fn init_project(self: &Self) -> Result<()> {
        let mut log = Logger::new();
        // Create the project directory under `./projects` using the project name.
        let create_dir_result = fs::create_dir(&self.fs_path);

        if create_dir_result.is_err() {
            return create_dir_result;
        }
        // Copy the template files into the project directory.
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

            // If the template does not contain a style.yml file, copy the default style.yml file
            fs::copy(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("templates")
                    .join("default")
                    .join("style.yml"),
                self.template.join("style.yml"),
            )?;
        }

        // Copy the style map used to describe the style of the slides.
        fs::copy(
            self.template.join("style.yml"),
            self.fs_path.join("style.yml"),
        )?;

        // Copy all the slides from the template into the project directory.
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
    /// Renders the project in terminal mode.
    /// # Arguments
    /// * `self` - The project instance.
    /// file_contents - The contents of the file to be rendered.
    /// style_map - The style map used to describe the style of the slides.
    /// render - A boolean indicating whether to render the slide or not.
    /// lines - The number of lines to be rendered.
    /// # Returns
    /// A result indicating whether the project was rendered successfully or not.

    fn render_term(
        self: &Self,
        file_contents: &str,
        style_map: &HashMap<String, String>,
        highlight: bool,
        render: bool,
        lines: &mut u32,
        current_slide: u32,
    ) -> std::result::Result<(NavigationAction, u32), Box<dyn Error>> {
        // Used to check whether all the lines will be rendered or will it be rendered one by one.
        let mut clear: bool = false;

        let mut log = Logger::new();

        if style_map.get("clear").unwrap() == "true" {
            clear = true;
        }
        let slide;
        let mut lines_value = *lines;

        if highlight {
            slide = prettify::prettify(&file_contents.to_string(), &style_map, lines_value)?;

            if file_contents.lines().count() as u32 <= lines_value {
                lines_value = file_contents.lines().count() as u32;
            }
            print!("{}", slide);
        } else {
            slide = prettify::prettify(&file_contents.to_string(), &style_map, 0)?;
            // The upper and lower bounds are used to determine the number of lines to be rendered.
            let (upper_bound, lower_bound) = prettify::get_bounds();

            // The range of scroll is determined by the upper and lower bounds.
            // The code handles for different terminal types.
            // Lines value is used to determine the number of lines to be rendered.
            // Scrolling can be done using the up and down arrow keys, or the j and k keys.
            // It controlls the number of lines to be rendered.

            if upper_bound <= lines_value {
                lines_value = upper_bound;
            } else if lower_bound > 2 && lower_bound - 2 > lines_value {
                lines_value = lower_bound - 2;
            }

            if render {
                if clear {
                    lines_value = slide.lines().count() as u32;
                    print!("{}", remove_last_n_lines(&slide, lines_value));
                } else {
                    print!("{}", slide);
                }
            } else {
                print!("{}", remove_last_n_lines(&slide, lines_value));
            }
        }

        match style_map.get("progress").unwrap().as_str() {
            "true" => {
                print!("\r");
                log.info(format!(
                    "[{}/{}]",
                    current_slide,
                    fs::read_dir(&self.fs_path)?.count()
                ));
            }
            _ => {}
        }

        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode()?;

        stdout.flush()?;

        // The navigation actions are handled here.
        // The navigation actions are:
        // 1. Next - Move to the next slide.
        // 2. Previous - Move to the previous slide.
        // 3. Exit - Exit the presentation.
        // 4. ScrollUp - Scroll up the slide.
        // 5. ScrollDown - Scroll down the slide.
        // 6. None - Do nothing.

        // Add a watcher here, any changes will call NavigationAction::Refresh

        for c in stdin.keys() {
            match c? {
                Key::Right | Key::Char('l') | Key::Char('L') => {
                    return Ok((NavigationAction::Next, lines_value))
                }
                Key::Left | Key::Char('h') | Key::Char('H') => {
                    return Ok((NavigationAction::Previous, lines_value));
                }
                //add escape and ctrl + c here
                Key::Char('q') | Key::Char('Q') => {
                    return Ok((NavigationAction::Exit, lines_value))
                }
                Key::Esc | Key::Ctrl('c') => return Ok((NavigationAction::Exit, lines_value)),
                Key::Up | Key::Char('k') | Key::Char('K') => {
                    return Ok((NavigationAction::ScrollUp, lines_value))
                }
                Key::Down | Key::Char('j') | Key::Char('J') => {
                    return Ok((NavigationAction::ScrollDown, lines_value))
                }
                Key::Char('t') => return Ok((NavigationAction::ToggleHighlight, lines_value)),
                Key::Ctrl('r') => return Ok((NavigationAction::Refresh, lines_value)),
                Key::Char(c) if ('0'..='9').contains(&c) => {
                    let mut log = Logger::new();
                    let style_map_clone = style_map.clone(); // Clone the style_map for the new thread
                    let c_num = (c as u8 - '0' as u8) as usize;
                    thread::Builder::new()
                        .name("ramen:".to_string())
                        .spawn(move || {
                            let output = Self::run_code(c_num, style_map_clone);
                            match output {
                                Ok(output) => {
                                    log.success(format!("\r{}:", c_num));
                                    output.lines().for_each(|line| println!("\r{}", line));
                                    print!("\n");
                                }
                                Err(e) => {
                                    log.error(format!("\r{} : {} ", c_num, e.to_string()));
                                }
                            }
                        })
                        .expect("Failed to spawn thread");
                    continue;
                }
                _ => continue,
            }
        }

        drop(stdout);

        return Ok((NavigationAction::None, lines_value));
    }

    fn run_code(
        num: usize,
        env_map: HashMap<String, String>,
    ) -> std::result::Result<String, Box<dyn Error>> {
        let res = prettify::get_code(num);
        match res {
            Ok((lang, code)) => {
                let res = run_code(lang, code, &env_map);
                match res {
                    Ok(output) => Ok(output),
                    Err(e) => Err(Box::new(DoughError(e.to_string()))),
                }
            }
            Err(e) => Err(Box::new(DoughError(e.to_string()))),
        }
    }

    /// This clears the terminal.
    fn clear() {
        let mut stdout = stdout();
        write!(stdout, "\x1B[2J\x1B[1;1H").unwrap();
    }

    // Implement a watcher for the project directory.

    /// Present a project in terminal mode.
    /// # Arguments
    /// * `self` - The project instance.
    /// # Returns
    /// A result indicating whether the project was presented successfully or not.

    pub fn present_term(self: &Self) -> std::result::Result<(), Box<dyn Error>> {
        let mut log = Logger::new();
        // The highlight variable is used to determine whether to highlight the code or scroll.
        let mut highlight = true;
        // The render variable is used to determine whether to render a new slide or not. Used for scrolling.
        let mut render = true;
        let mut current_slide = 1;
        // The lines variable is used to determine the number of lines to be rendered.
        let mut lines: u32 = 2;

        // Check if the project directory has style.yml file
        let style_path = self.fs_path.join("style.yml");
        if !style_path.exists() {
            log.warn("Style config not found. Using default styles");
            let res = utils::create_style(self.fs_path.clone());
            match res {
                Ok(_) => {}
                Err(e) => println!("Error creating style file: {}", e),
            }
        }

        // The loop is used to present the slides one by one.
        // The loop is exited when the user exits the presentation.
        // It handles the navigation actions.

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
                    print!("{}", termion::cursor::Show);
                    print!("{}", termion::clear::All);
                    print!("Thank you :)\n");
                    exit(0)
                }
            }

            let file_contents = fs::read_to_string(&file_path)?;
            let contents = remove_comments(&file_contents);

            // The style map is used to describe the style of the slides.
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

            // The navigation actions are handled here.
            // Scrolling up decreases the number of lines to be rendered.
            // Scrolling down increases the number of lines to be rendered.
            // Next and previous move to the next and previous slides respectively.
            // Exit exits the presentation.

            match Self::render_term(
                self,
                &contents,
                &style_map,
                highlight,
                render,
                &mut lines,
                current_slide,
            )? {
                (NavigationAction::Next, _new_lines_value) => {
                    render = true;
                    current_slide += 1;
                    lines = 2;
                }
                (NavigationAction::Previous, _new_lines_value) => {
                    render = true;
                    if current_slide > 1 {
                        current_slide -= 1;
                    }
                    lines = 2;
                }
                (NavigationAction::ScrollUp, new_lines_value) => {
                    render = false;
                    lines = new_lines_value;
                    if highlight {
                        if lines < contents.lines().count() as u32 {
                            lines += 1;
                        }
                    } else {
                        lines += 1
                    }
                }
                (NavigationAction::ScrollDown, new_lines_value) => {
                    render = false;
                    lines = new_lines_value;
                    if lines > 2 {
                        lines -= 1;
                    }
                }
                (NavigationAction::ToggleHighlight, new_lines_value) => {
                    render = true;
                    if highlight {
                        lines = 2;
                    } else {
                        lines = new_lines_value;
                    }
                    highlight = !highlight;
                }
                (NavigationAction::Refresh, _new_lines_value) => {
                    render = true;
                    lines = 2;
                }
                (NavigationAction::Exit, _new_lines_value) => {
                    exit(0);
                }
                (NavigationAction::None, _new_lines_value) => {}
            }
        }
    }
}
