use crate::doit_node::get_node_scripts;
use std::io;
use std::path::Path;
use std::process::Command;

pub struct Script {
    pub name: String,
    pub command: String,
    pub program: String,
    pub args: Vec<String>,
}

pub enum ProjectType {
    Node,
}

pub fn run() -> Result<u32, &'static str> {
    let scripts = match get_project_type() {
        Some(project_type) => match project_type {
            ProjectType::Node => match get_scripts_for_project_type(project_type) {
                Ok(scripts) => scripts,
                Err(msg) => return Err(msg),
            },
        },
        None => return Err("Not an eligible project folder."),
    };

    for (idx, script) in scripts.iter().enumerate() {
        println!("{}: {} - {}", idx, script.name, script.command);
    }

    // wait for input
    let mut selection = String::new();
    io::stdin().read_line(&mut selection).expect("Failed");

    let selection: usize = match selection.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid Selection"),
    };

    if selection > scripts.len() - 1 {
        return Err("Invalid selection");
    }

    // if input matches exec command then exit
    let script = &scripts[selection];
    let mut command = Command::new(&script.program);

    for arg in &script.args {
        command.arg(arg);
    }

    command.spawn().expect("Failed to process command");

    Ok(0)
}

fn get_project_type() -> Option<ProjectType> {
    if Path::new("package.json").exists() {
        return Some(ProjectType::Node);
    }
    None
}

fn get_scripts_for_project_type(project_type: ProjectType) -> Result<Vec<Script>, &'static str> {
    match project_type {
        ProjectType::Node => get_node_scripts(),
    }
}
