use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub struct Project {
    name: String,
}

impl Project {
    pub fn new(name: &str, _template_name: &str) -> Project {
        Project {
            name: name.to_string(),
        }
    }

    pub fn create(&self) -> Result<(), Error> {
        // Construct the project directory path
        let project_dir_path = Path::new("projects").join(&self.name);

        // Check if the project directory already exists
        if project_dir_path.exists() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("Project directory '{}' already exists.", self.name),
            ));
        }

        // Create the project directory
        if let Err(err) = std::fs::create_dir(&project_dir_path) {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Failed to create project directory: {}", err),
            ));
        }

        // Create the 'first.md' file inside the project directory
        let first_md_path = project_dir_path.join("first.md");
        if let Err(err) = File::create(&first_md_path) {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Failed to create 'first.md' file: {}", err),
            ));
        }

        Ok(())
    }
}
