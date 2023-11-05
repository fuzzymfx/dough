use clap::{App, Arg, SubCommand};
use dough::Project;
use paris::Logger;
use std::env;
use std::process;

fn main() {
    let mut log = Logger::new();

    let matches = App::new("dough")
        .version("0.0.1")
        .author("Anubhab P, injuly")
        .about("A command-line tool to create presentations from Markdown")
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new project")
                .arg(Arg::with_name("project-name").required(true)),
        )
        .subcommand(
            SubCommand::with_name("present")
                .about("Present a deck")
                .arg(Arg::with_name("project-name").required(true)),
        )
        .get_matches();

    if let Some(args) = matches.subcommand_matches("new") {
        create_project(args, &mut log);
    } else if let Some(args) = matches.subcommand_matches("present") {
        present_project(args, &mut log);
    }
}

fn create_project(args: &clap::ArgMatches, log: &mut Logger) {
    let project_name = args
        .value_of("project-name")
        .expect("project name is required");
    log.info(format!("Creating new project '{}'", project_name));
    let cwd = env::current_dir().expect("Failed to get current working directory");
    let project = Project::new(project_name, &cwd.to_str().unwrap());

    if let Err(err) = project.init_project() {
        log.error(format!("Could not create project, error: {}", err));
        process::exit(1);
    }
}

fn present_project(args: &clap::ArgMatches, log: &mut Logger) {
    let project_name = args
        .value_of("project-name")
        .expect("project name is required");
    log.info(format!("Presenting project '{}'", project_name));
    let cwd = env::current_dir().expect("Failed to get current working directory");
    let project = Project::new(project_name, &cwd.to_str().unwrap());

    if let Err(err) = project.present() {
        log.error(format!("Could not present project, error: {}", err));
        process::exit(2);
    }
}
