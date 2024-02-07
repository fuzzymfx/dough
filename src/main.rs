use clap::{App, Arg, SubCommand};
use dough::Project;
use paris::Logger;
use std::env;
use std::process;

/// The main function of the program.
///
/// This function initializes the paris logger, parses command-line arguments using `clap`, and
/// dispatches commands based on the provided subcommands. It handles the 'new' and 'present'
/// subcommands, creating or presenting projects accordingly, and prints help information
/// if no valid subcommand is provided.

fn main() {
    // Initialize the logger instance.
    let mut log = Logger::new();

    // Parse command-line arguments using `clap`.
    let matches = App::new("dough")
        .version("0.0.2")
        .author("fuzzymfx, injuly")
        .about(
            "
        ~~~~~~
        |     |
        |     | : A command-line tool to create and present using Markdown files
        |     |
        ~~~~~~
        ",
        )
        .subcommand(
            // Creates a new project under the `projects` directory.
            SubCommand::with_name("new")
                .about("Create a new project.")
                .arg(Arg::with_name("project-name").required(true).help("The name of the project"))
                .arg(
                    Arg::with_name("template")
                        .long("template")
                        .takes_value(true)
                        .default_value("default")
                        .help("Choose a template for the project. If you don't specify a template, the default template will be used. You can also create a project just by creating a new directory and adding a 1.md file to it."),
                ),
        )
        .subcommand(
            // Presents a project in the specified mode. Defaults to terminal mode.
            SubCommand::with_name("present")
                .about("Present a deck")
                .arg(Arg::with_name("project-name").required(true))
                .arg(
                    Arg::with_name("mode")
                        .long("mode")
                        .takes_value(true)
                        .possible_values(&["html", "term"])
                        .default_value("term")
                        .help("Choose the mode of presentation: html or term. Currently we only support term"),
                ),
        )
        .get_matches();

    // println!("{:?}", matches);

    // Dispatch commands based on the provided subcommands.
    if let Some(args) = matches.subcommand_matches("new") {
        // Create a new project
        create_project(args, &mut log);
    } else if let Some(args) = matches.subcommand_matches("present") {
        // Present a project
        present_project(args, &mut log);
    } else {
        // Print help information if no valid subcommand is provided.
        print!(
            "
        ~~~~~~
        |     |
        |     | : A command-line tool to create and present using Markdown files
        |     |
        ~~~~~~
        "
        );

        println!("\n{}", matches.usage());
    }
}

/// Create a new project.
/// This function creates a new project using the provided arguments.
/// It creates a new project directory, copies the template files into it, and initializes
/// the project.
/// If any of these steps fail, the function prints an error message and exits with a
/// non-zero exit code.
/// # Arguments
/// * `args` - The command-line arguments provided by the user.
/// * `log` - The paris logger instance.

fn create_project(args: &clap::ArgMatches, log: &mut Logger) {
    // Get the project name from the command-line arguments.
    let project_name = args
        .value_of("project-name")
        .expect("project name is required");

    // Get the template name from the command-line arguments. If no template is provided, use
    // the default template.
    let template = args.value_of("template").unwrap_or("default");
    log.info(format!("Creating new project '{}'", project_name));
    let cwd = env::current_dir().expect("Failed to get current working directory");
    // Create a new project instance.
    let project = Project::new(project_name, &cwd.to_str().unwrap(), template);

    if let Err(err) = project.init_project() {
        log.error(format!("Could not create project, error: {}", err));
        process::exit(1);
    }
}

/// Present a project.
/// This function presents a project using the provided arguments.
/// It initializes the project, and then presents it in the specified mode.
/// If any of these steps fail, the function prints an error message and exits with a
/// non-zero exit code.
/// # Arguments
/// * `args` - The command-line arguments provided by the user.
/// * `log` - The paris logger instance.

fn present_project(args: &clap::ArgMatches, log: &mut Logger) {
    // Get the project name from the command-line arguments.
    let project_name = args
        .value_of("project-name")
        .expect("project name is required");

    // Get the presentation mode from the command-line arguments. If no mode is provided, use terminal mode.
    let mode = args.value_of("mode").unwrap_or("term"); // Default to terminal mode

    log.info(format!(
        "Presenting project '{}' in '{}' mode",
        project_name, mode
    ));
    let cwd = env::current_dir().expect("Failed to get current working directory");
    // Create a new project instance.
    let project = Project::new(project_name, &cwd.to_str().unwrap(), "default");

    match mode {
        // The HTML mode is not implemented yet, so we only support terminal mode for now. This
        // will be updated in the future.
        "term" | _ => {
            if let Err(err) = project.present_term() {
                log.error(format!(
                    "Could not present project in terminal, error: {}",
                    err
                ));
                process::exit(4);
            }
        }
    }
}
