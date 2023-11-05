use clap::{App, Arg};
use dough::Project;
use paris::Logger;

fn main() {
    let mut log = Logger::new();

    let matches = App::new("dough")
        .version("0.0.1")
        .author("Anubhab P, injuly")
        .about("A command-line tool to create presentations from Markdown")
        .subcommand(
            App::new("new")
                .about("Create a new project")
                .arg(Arg::with_name("project-name").required(true)),
        )
        .subcommand(
            App::new("present")
                .about("Present a deck")
                .arg(Arg::with_name("project-name").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("new", Some(args)) => match args.value_of("project-name") {
            Some(project_name) => {
                log.info(format!("Creating new project '{}'", project_name));
                if let Ok(cwd) = std::env::current_dir() {
                    let project = Project::new(project_name, cwd.to_str().unwrap());
                    if let Err(err) = project.init_project() {
                        let mut err_msg = String::from("Could not create project, error:");
                        err_msg.push_str(&err.to_string());
                        log.error(err_msg);
                    }
                } else {
                    log.error("Failed to get current working directory");
                    std::process::exit(1);
                }
            }
            _ => {}
        },
        ("present", Some(args)) => match args.value_of("project-name") {
            Some(project_name) => {
                log.info(format!("Presenting project '{}'", project_name));
                if let Ok(cwd) = std::env::current_dir() {
                    let project = Project::new(project_name, cwd.to_str().unwrap());
                    if let Err(err) = project.present() {
                        let mut err_msg = String::from("Could not present project, error:");
                        err_msg.push_str(&err.to_string());
                        log.error(err_msg);
                        std::process::exit(1);
                    }
                } else {
                    log.error("Failed to get current working directory");
                    std::process::exit(1);
                }
            }
            _ => {
                log.error("WTF");
            }
        },
        _ => {} // impossible
    }
}
