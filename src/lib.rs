mod prettify;
extern crate termion;
mod utils;
use crate::utils::remove_last_n_lines;

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

// All the possible navigation actions when presenting a project.
enum NavigationAction {
    Next,
    Previous,
    Exit,
    None,
    ScrollUp,
    ScrollDown,
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
        // The project directory is located at `./projects`. It is the parent directory of all the projects.
        // If the directory does not exist, create it.
        let project_dir = workdir.to_owned() + "/projects";

        if !std::path::Path::new(&project_dir).exists() {
            fs::create_dir(&project_dir).expect("Could not create projects directory");
        }

        // Use the template provided to copy the template files into the project directory.
        Project {
            fs_path: std::path::Path::new(&project_dir).join(name).to_path_buf(),
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
        file_contents: &str,
        style_map: &HashMap<String, String>,
        render: bool,
        lines: &u32,
    ) -> std::result::Result<(NavigationAction, u32), Box<dyn Error>> {
        // Used to check whether all the lines will be rendered or will it be rendered one by one.
        let mut clear: bool = false;

        if style_map.get("clear").unwrap() == "true" {
            clear = true;
        }

        // The slide is styled using the prettify module.
        let slide = prettify::prettify(file_contents, &style_map)?;
        // The upper and lower bounds are used to determine the number of lines to be rendered.
        let (upper_bound, lower_bound) = prettify::get_bounds();

        let mut lines_value = *lines;

        // The range of scroll is determined by the upper and lower bounds.
        // The code handles for different terminal types.
        // Lines value is used to determine the number of lines to be rendered.
        // Scrolling can be done using the up and down arrow keys, or the j and k keys.
        // It controlls the number of lines to be rendered.

        if let Some(terminal_style) = style_map.get("terminal") {
            if terminal_style == "warp" {
                if upper_bound < lines_value {
                    lines_value = upper_bound;
                } else if lower_bound > 2 && lower_bound - 2 > lines_value {
                    lines_value = lower_bound - 2;
                }
            } else {
                if upper_bound < lines_value {
                    lines_value = upper_bound;
                } else if lower_bound > lines_value {
                    lines_value = lower_bound;
                }
            }
        }

        // render is used to determine whether to render a new slide or not.
        // if render is false, the slide is being scrolled.
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

        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode()?;
        if render {
            stdout.flush()?;
        }
        // The navigation actions are handled here.
        // The navigation actions are:
        // 1. Next - Move to the next slide.
        // 2. Previous - Move to the previous slide.
        // 3. Exit - Exit the presentation.
        // 4. ScrollUp - Scroll up the slide.
        // 5. ScrollDown - Scroll down the slide.
        // 6. None - Do nothing.

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
                _ => continue,
            }
        }

        drop(stdout);

        return Ok((NavigationAction::None, lines_value));
    }

    /// This clears the terminal.
    fn clear() {
        let mut stdout = stdout();
        write!(stdout, "\x1B[2J\x1B[1;1H").unwrap();
    }

    /// Present a project in terminal mode.
    /// # Arguments
    /// * `self` - The project instance.
    /// # Returns
    /// A result indicating whether the project was presented successfully or not.

    pub fn present_term(self: &Self) -> std::result::Result<(), Box<dyn Error>> {
        // The render variable is used to determine whether to render a new slide or not.
        let mut render = true;
        let mut current_slide = 1;
        // The lines variable is used to determine the number of lines to be rendered.
        let mut lines: u32 = 0;

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

            let contents = fs::read_to_string(&file_path)?;
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
                (NavigationAction::None, _new_lines_value) => {}
            }
        }
    }
}
