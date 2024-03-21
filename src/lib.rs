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
        // The project directory is the root directory of the project.
        // You can create your own directories and add the path:

        // ```dough new my_folder/my_project``` -> creates your project in the my_folder directory.
        // ```dough new my_project``` -> creates your project in the current directory.

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
    /// A result containing a tuple of the navigation action and the number of lines to be rendered.

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
        // Based on the config in the style map
        let clear = if style_map.get("clear").unwrap() == "true" {
            true
        } else {
            false
        };

        let boxed = if style_map.get("box").unwrap() == "true" {
            true
        } else {
            false
        };

        // The custom Paris logger, used to log the progress of the presentation.
        let mut log = Logger::new();

        let slide;

        // The number of lines to be rendered.
        let mut line_number = *lines;

        // There are two modes of rendering according to the navigation action:
        // 1. Highlight - The lines are highlighted. All contents are rendered at once.
        // 2. Scroll - The lines are scrolled. The number of lines to be rendered is controlled by the user.

        // The highlight mode
        if highlight {
            // The slide is rendered according to the rendered lines.
            slide = prettify::prettify(&file_contents.to_string(), &style_map, line_number)?;

            // The bounds are used to determine the number of lines to be rendered.
            // This code implements infinte scrolling while highlighting.

            let (upper_bound, lower_bound) = prettify::get_bounds();

            if (upper_bound - lower_bound as u32) - 1 < line_number {
                line_number = 0;
            } else if line_number < 1 {
                line_number = upper_bound - lower_bound - 1;
            }
            // The slide is rendered here
            print!("{}", slide);

            // if clear is true, the slide is cleared after rendering, enabling users to scroll down lines one by one
            if render && clear {
                return Ok((NavigationAction::ToggleHighlight, line_number));
            }
        }
        // The scroll mode
        else {
            // The slide is rendered  with `0 lines` lines to be highlighted.
            slide = prettify::prettify(&file_contents.to_string(), &style_map, 0)?;
            // The bounds are used to determine the number of lines to be rendered, and the scrolling range.
            let (upper_bound, lower_bound) = prettify::get_bounds();

            // The range of scroll is determined by the upper and lower bounds.
            // Scrolling can be done using the up and down arrow keys, or the j and k keys.
            if boxed {
                // If the slide is boxed, the scrolling range is reduced by 1.
                if upper_bound - 1 < line_number {
                    line_number = upper_bound - 1;
                } else if lower_bound > 2 && lower_bound - 2 > line_number {
                    line_number = lower_bound - 2;
                }
            } else {
                // If the slide is not boxed, the scrolling range is reduced by 2.
                if upper_bound - 2 < line_number {
                    line_number = upper_bound - 2;
                } else if lower_bound > 2 && lower_bound - 2 > line_number {
                    line_number = lower_bound - 2;
                }
            }

            // the render condition implies that a fresh slide is rendered.
            if render {
                if clear {
                    // If clear is true, all the lines are removed while rendering, enabling users to scroll down lines one by one.
                    line_number = slide.lines().count() as u32;
                    print!("{}", remove_last_n_lines(&slide, line_number));
                } else {
                    // If clear is false, the entire slide is rendered.
                    print!("{}", slide);
                }
            } else {
                // if the render is false, the slide is being scrolled and the last n lines are removed.
                print!("{}", remove_last_n_lines(&slide, line_number));
            }
        }
        // The progress implies the number of slides that have been rendered/ the total number of slides.
        // It is rendered based on the config in the style map.
        match style_map.get("progress").unwrap().as_str() {
            "true" => {
                print!("\r");
                log.info(format!(
                    "[{}/{}]",
                    current_slide,
                    fs::read_dir(&self.fs_path)?.count() - 1
                ));
            }
            _ => {}
        }

        // The stdout is flushed to ensure that the slide is rendered properly.
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
        // 6. ToggleHighlight - Toggle between highlighting and scrolling modes.
        // 7. Refresh - Refresh the slide.
        // 8. RunCode - Run the code block in the slide acc to the order of thei appearance.
        // 9. None - Do nothing.

        // TODO: Add a watcher here, any changes will call NavigationAction::Refresh

        for c in stdin.keys() {
            match c? {
                Key::Right | Key::Char('l') | Key::Char('L') => {
                    return Ok((NavigationAction::Next, line_number))
                }
                Key::Left | Key::Char('h') | Key::Char('H') => {
                    return Ok((NavigationAction::Previous, line_number));
                }
                //add escape and ctrl + c here
                Key::Char('q') | Key::Char('Q') => {
                    return Ok((NavigationAction::Exit, line_number))
                }
                Key::Esc | Key::Ctrl('c') => return Ok((NavigationAction::Exit, line_number)),
                Key::Up | Key::Char('k') | Key::Char('K') => {
                    return Ok((NavigationAction::ScrollUp, line_number))
                }
                Key::Down | Key::Char('j') | Key::Char('J') => {
                    return Ok((NavigationAction::ScrollDown, line_number))
                }
                Key::Char('t') => return Ok((NavigationAction::ToggleHighlight, line_number)),
                Key::Ctrl('r') => return Ok((NavigationAction::Refresh, line_number)),
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

        return Ok((NavigationAction::None, line_number));
    }

    /// Run the code block in the slide.
    /// # Arguments
    /// * `num` - The number of the code block in the slide.
    /// * `env_map` - The environment map used to run the code.
    /// # Returns
    /// A result containing the output of the code block.

    fn run_code(
        num: usize,
        env_map: HashMap<String, String>,
    ) -> std::result::Result<String, Box<dyn Error>> {
        // The langugage and the code are obtained from the slide.
        let res = prettify::get_code(num);
        match res {
            // If the code block is found, the code is run.
            Ok((lang, code)) => {
                let res = run_code(lang, code, &env_map);
                match res {
                    // The output of the code block is returned.
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

    /// Present a project in terminal mode.
    /// # Arguments
    /// * `self` - The project instance.
    /// # Returns
    /// A result indicating whether the project was presented successfully or not.

    pub fn present_term(self: &Self) -> std::result::Result<(), Box<dyn Error>> {
        // The custom Paris logger, used to log the progress of the presentation.
        let mut log = Logger::new();
        // Used to determine whether to highlight the code or scroll.
        let mut highlight = true;
        // Used to determine whether to render a new slide or not. Used for scrolling.
        let mut render = true;
        let mut current_slide = 1;

        // The number of lines to be rendered.
        let mut lines: u32 = 1;

        // Check if the project directory has style.yml file
        let style_path = self.fs_path.join("style.yml");
        if !style_path.exists() {
            // If the style.yml file is not found, the default style config is used to build the style.yml file.
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
                }
                print!("{}", termion::cursor::Show);
                print!("{}", termion::clear::All);
                println!("Thank you :)");
                exit(0)
            }
            // The contents of the file are read and the comments are removed.
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
            // TODO: Pass a mutable reference of the lines to be rendered to the render_term function, instead of returning it and updating it here.

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
                    // A fresh slide is rendered.
                    render = true;
                    current_slide += 1;
                    // scrolling starts from the bottom
                    lines = 1;
                }
                (NavigationAction::Previous, _new_lines_value) => {
                    // A fresh slide is rendered.
                    render = true;
                    if current_slide > 1 {
                        current_slide -= 1;
                    }
                    // scrolling starts from the bottom
                    lines = 1;
                }
                (NavigationAction::ScrollUp, new_lines_value) => {
                    // The slide is scrolled up, or the lines are highlighted.
                    render = false;
                    lines = new_lines_value;
                    lines += 1
                }
                (NavigationAction::ScrollDown, new_lines_value) => {
                    // The slide is scrolled down, or the lines are highlighted.
                    render = false;
                    lines = new_lines_value;
                    if lines != 0 {
                        lines -= 1;
                    }
                }
                (NavigationAction::ToggleHighlight, new_lines_value) => {
                    if highlight {
                        lines = 1;
                    } else {
                        lines = new_lines_value;
                    }

                    // The slide is toggled between highlighting and scrolling.
                    highlight = !highlight;
                }
                (NavigationAction::Refresh, _new_lines_value) => {
                    // Refreshes the slide after a change is made to the MD file.
                    render = true;
                    lines = 1;
                }
                (NavigationAction::Exit, _new_lines_value) => {
                    // The presentation is exited.
                    print!("{}", termion::cursor::Show);
                    print!("{}", termion::clear::All);
                    println!("Thank you :)");
                    exit(0);
                }
                (NavigationAction::None, _new_lines_value) => {}
            }
        }
    }
}
