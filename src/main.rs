use clap::{App, Arg, SubCommand};
use dough::Project;
use paris::Logger;
use std::env;
use std::process;

fn main() {
    let mut log = Logger::new();

    let matches = App::new("dough")
        .version("0.0.2")
        .author("fuzzymfx, injuly")
        .about("
        ~~~~~~
        |     |
        |     | : A command-line tool to create presentations from Markdown files
        |     |
        ~~~~~~
        ")
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new project")
                .arg(Arg::with_name("project-name").required(true))
                .arg(
                    Arg::with_name("template")
                        .long("template")
                        .takes_value(true)
                        .default_value("default")
                        .help("Choose a template for the project"),
                ),
        )
        .subcommand(
            SubCommand::with_name("present")
                .about("Present a deck")
                .arg(Arg::with_name("project-name").required(true))
                .arg(
                    Arg::with_name("mode")
                        .long("mode")
                        .takes_value(true)
                        .possible_values(&["html", "term"])
                        .default_value("term")
                        .help("Choose the mode of presentation: html or term"),
                ),
        )
        .get_matches();

    if let Some(args) = matches.subcommand_matches("new") {
        create_project(args, &mut log);
    } else if let Some(args) = matches.subcommand_matches("present") {
        present_project(args, &mut log);
    }
    else{
        print!("
        ~~~~~~
        |     |
        |     | : A command-line tool to create presentations from Markdown files
        |     |
        ~~~~~~
        ");
        
        App::new("dough")
        .print_help()
        .unwrap();

    }
}

fn create_project(args: &clap::ArgMatches, log: &mut Logger) {
    let project_name = args
        .value_of("project-name")
        .expect("project name is required");

    let template = args.value_of("template").unwrap_or("default");
    log.info(format!("Creating new project '{}'", project_name));
    let cwd = env::current_dir().expect("Failed to get current working directory");
    let project = Project::new(project_name, &cwd.to_str().unwrap(), template);

    if let Err(err) = project.init_project() {
        log.error(format!("Could not create project, error: {}", err));
        process::exit(1);
    }
}

fn present_project(args: &clap::ArgMatches, log: &mut Logger) {
    let project_name = args
        .value_of("project-name")
        .expect("project name is required");
    let mode = args.value_of("mode").unwrap_or("term"); // Default to terminal mode

    log.info(format!(
        "Presenting project '{}' in '{}' mode",
        project_name, mode
    ));
    let cwd = env::current_dir().expect("Failed to get current working directory");
    let project = Project::new(project_name, &cwd.to_str().unwrap(), "default");

    match mode {
        "html" => {
            // if let Err(err) = project.present_html() {
            //     log.error(format!("Could not present project in HTML, error: {}", err));
            //     process::exit(3);
            // }
        }
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
