use serde::Serialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, fs, path::PathBuf};

use super::prelude::*;

#[derive(Serialize, Deserialize)]
struct Templates {
    templates: HashMap<String, Vec<TemplateFileData>>,
    combo_templates: HashMap<String, Vec<String>>,
}

struct Template {
    path_to_json: PathBuf,
    includes: Vec<TemplateFileData>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TemplateFileData {
    path: PathBuf,
    name: String,
    run_on: Option<String>,
    inits: Option<Vec<InitData>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct InitData {
    target: String,
    when: String,
    function: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct ModJson {
    Name: String,
    Description: String,
    Version: String,
    LoadPriority: i32,
    ConVars: Vec<Value>,
    Scripts: Vec<Value>,
}

pub fn new_project(name: String, template: String) {
    let name = get_project_name(&name);

    let template = get_template(&template);

    println!("creating mod.json");

    fs::create_dir(&name).unwrap_or_else(|_| println!("we failed to create folder :("));

    let mut scripts = Vec::new();

    for file in template.includes {
        let mut path = PathBuf::new();
        path = path.join(&name);
        path = path.join(&file.path); // yes
        match fs::create_dir_all(&path) {
            Err(err) => println!(
                "folder builder build failed; reason : {:?}; error will be ignored",
                err
            ),
            _ => println!("built folders: {:?}", path),
        };

        let mut write_path = PathBuf::new();
        write_path = write_path.join(&path);
        write_path = write_path.join(&file.name);

        let mut read_path = PathBuf::new();
        read_path = read_path.join(&template.path_to_json);
        read_path = read_path.join(&file.path);
        read_path = read_path.join(&file.name);

        if read_path.is_dir() {
            println!("This is a dir; reading nothing; continuing normally");
            continue;
        }

        let content = match fs::read(&read_path) {
            Ok(content) => content,
            Err(err) => {
                println!( "failed to read the contents of {} at {:?} because of {}; error will ignored aka template not fully generate", &file.name, &read_path, err );
                continue;
            }
        };

        match fs::write(&write_path, &content) {
            Ok(_) => println!( "successfully added {} to the project", &file.name ),
            Err(err) => println!( "failed to write the contents of {} to {:?} because of {}; error will ignored aka template not fully generate", &file.name, &write_path, err ),
        }

        if file.run_on.is_some() {
            scripts.push(generate_script_data(file))
        }
    }

    let mut path_to_mod_json = PathBuf::new();
    path_to_mod_json = path_to_mod_json.join(&name);
    path_to_mod_json = path_to_mod_json.join("mod.json");

    let json_data = generate_mod_json(&name, &scripts); // TODO: redo this with clap

    match fs::write(&path_to_mod_json, &json_data) {
        Ok(_) => println!("successfully added mod.json to the project"),
        Err(err) => println!(
            "failed to generate mod json, {:?} ----- dumped data {}",
            err, json_data
        ),
    }
}

fn get_template(template_name: &String) -> Template {
    let (json, mut path) = read_relative_json("templates\\templates.json");

    let templates: Templates =
        serde_json::from_str(&json[..]).expect("someone destroyed templates.json");

    let mut includes = Vec::new();

    let template_names = match templates.combo_templates.get(template_name) {
        Some(content) => content.clone(),
        None => vec![template_name.to_owned()],
    };

    for template_name in template_names {
        match templates.templates.get(&template_name) {
            Some(value) => includes.append(&mut value.clone()),
            None => {
                println!("{} template wasn't found", &template_name);
                std::process::exit(0);
            }
        };
    }

    path.pop(); // to remove the templates.json field

    Template {
        path_to_json: path,
        includes: includes,
    }
}

fn generate_script_data(file: TemplateFileData) -> Value {
    let mut scripts = json!({
        "Path": file.name,
        "RunOn": file.run_on.unwrap(),
    });

    if file.inits.is_some() {
        for content in file.inits.unwrap() {
            merge(
                &mut scripts,
                &json!({
                    &content.target: {
                        &content.when: content.function
                    }
                }),
            )
        }
    };

    scripts
}

fn generate_mod_json(name: &String, scripts: &Vec<Value>) -> String {
    let mod_json = json!({
        "Name" : name,
        "Description" : "",
        "Version": "0.1.0",
        "LoadPriority": 1,

        "ConVars": [
        ],

        "Scripts": scripts
    });

    let mod_json: ModJson = serde_json::from_value(mod_json).expect("smth failed while trying to generate mod.json");

    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    mod_json.serialize(&mut ser).unwrap();
    String::from_utf8(ser.into_inner()).unwrap()
}

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

/*
reference :D
{
    "Name" : "cat_or_not.AimLab",
    "Description" : "A mod to help you improve your aim :D",
    "Version": "0.1.0",
    "LoadPriority": 1,

    "ConVars": [
    ],

    "Scripts": [
        {
            "Path": "server.nut",
            "RunOn": "SERVER && MP",
            "ServerCallback": {
                "After": "Init_server"
            }
        }
    ]
}
*/
