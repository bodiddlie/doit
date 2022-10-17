use crate::doit::Script;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

pub fn get_node_scripts() -> Result<Vec<Script>, &'static str> {
    let mut file = match File::open("package.json") {
        Ok(f) => f,
        Err(_) => return Err("Error opening package.json file"),
    };

    // get contents of file
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return Err("Error reading package.json contents");
    }

    // get just scripts of json
    let json = match serde_json::from_str::<Value>(&contents) {
        Ok(value) => value,
        Err(_) => return Err("Error deserializing the package.json file."),
    };

    let script_json = &json["scripts"];

    // for each script
    //   print idx, name, command
    let mut scripts: Vec<Script> = Vec::new();
    match script_json {
        Value::Object(obj) => {
            for (k, v) in obj.iter() {
                if let Some(command) = v.as_str() {
                    let script = Script {
                        name: k.clone(),
                        command: command.to_string(),
                        program: "npm".to_string(),
                        args: vec!["run".to_string(), k.clone()],
                    };
                    scripts.push(script);
                }
            }
        }
        _ => {}
    }

    Ok(scripts)
}
