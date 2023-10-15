use super::projects; // Import the projects module from the parent module

pub fn create_new_project(project_name: &str, template_name: &str) {
    let project = projects::Project::new(project_name, template_name);
    let res = project.create();
		match res {
				Ok(_) => {
						println!("Created project '{}'.", project_name);
				}
				Err(err) => {
						println!("Failed to create project: {}", err);
				}
		}
}