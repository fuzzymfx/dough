extern crate clap;

use clap::{App, Arg};

mod commands;
mod projects;

use crate::commands::new::create_new_project;
use crate::commands::build::build_project;

fn main() {
    let matches = App::new("dough")
        .version("1.0")
        .author("Anubhab P")
        .about("A command-line tool to create presentations from Markdown")
        .subcommand(
            App::new("new")
                .about("Create a new project")
                .arg(Arg::with_name("project_name").required(true))
                .arg(Arg::with_name("template_name").required(false))
        )
        .subcommand(
            App::new("build")
                .about("Build a presentation")
                .arg(Arg::with_name("project_name").required(true))
                .arg(Arg::with_name("output_format").required(true))
        )
        .get_matches();

    match matches.subcommand() {
        ("new", Some(new_matches)) => {
            let project_name = new_matches.value_of("project_name").unwrap();
            let template_name = new_matches.value_of("template_name").unwrap();
            create_new_project(project_name, template_name);
        }
        ("build", Some(build_matches)) => {
            let project_name = build_matches.value_of("project_name").unwrap();
            let output_format = build_matches.value_of("output_format").unwrap();
            build_project(project_name, output_format);
        }
        _ => {
            println!("Invalid command. Use 'dough --help' for usage information.");
        }
    }
}
