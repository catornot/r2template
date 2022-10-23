use std::{collections::HashMap, fs, path::PathBuf};

use clap::Parser;
use process_path::get_dylib_path;
use serde_derive::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version,about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    new: Option<String>,

    #[arg(short, long)]
    template: Option<String>,

    #[arg(short, long)]
    author: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Templates {
    templates: HashMap<String, Vec<TemplateFileData>>,
    combo_templates: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct InfoJson {
    author: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct InitData {
    target: String,
    when: String,
    function: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct TemplateFileData {
    path: PathBuf,
    name: String,
    run_on: Option<String>,
    inits: Option<Vec<InitData>>,
}

struct Template {
    path_to_json: PathBuf,
    includes: Vec<TemplateFileData>,
}

impl Cli {
    pub fn commands() {
        match Self::parse().author {
            Some(arg) => match Cli::new_author(&arg) {
                Err(err) => println!("failed to serialize : {:?}", err),
                Ok(_) => println!("author is set to {}", arg),
            },
            _ => println!("author remains unchanged"),
        }

        let template = match Self::parse().template {
            Some(arg) => arg,
            _ => "server-side".to_string(),
        };

        match Self::parse().new {
            Some(arg) => Cli::new_project(arg, template),
            _ => print!(""),
        }
    }

    pub fn new_author(author: &String) -> Result<(), serde_json::Error> {
        let json = &read_relative_json("templates\\info.json").0[..];

        let mut data: InfoJson = serde_json::from_str(json).expect("someone destroyed info.json");

        data.author = author.clone();

        write_relative_json(
            "templates\\info.json",
            &serde_json::to_string_pretty(&data)?,
        );

        Ok(())
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

        let json_data = generate_mod_json(&name, scripts.concat()); // TODO: redo this with clap 

        match fs::write(&path_to_mod_json, &json_data) {
            Ok(_) => println!("successfully added mod.json to the project"),
            Err(err) => println!(
                "failed to generate mod json, {:?} ----- dumped data {}",
                err, json_data
            ),
        }
    }
}

fn get_project_name(name: &String) -> String {
    let json = &read_relative_json("templates\\info.json").0[..];

    let data: InfoJson = serde_json::from_str(json).expect("someone destroyed info.json");

    let mut name = name.to_owned();
    name.insert(0, '.');
    name.insert_str(0, &data.author[..]);
    name
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

fn read_relative_json(reltive_path: &str) -> (String, PathBuf) {
    let path = get_dylib_path();
    let mut path = match path {
        None => panic!("The process path could not be determined"),
        Some(path) => path,
    };

    if cfg!(debug_assertions) {
        path.pop();
        path.pop();
    }
    path.pop();

    let mut new_path = PathBuf::new();
    for p in &path {
        // so like for reason the function above gives me \\\\?\\C: on windows so like hack here for it :(
        if p.to_string_lossy().find("C:").is_some() {
            new_path = new_path.join("C:\\");
        } else {
            new_path = new_path.join(p);
        }
    }

    println!("path to executable is {:?}", &new_path);
    let new_path = new_path.join(reltive_path);

    (
        fs::read_to_string(&new_path)
            .expect(&format!("failed to read json file : {:?}", new_path)[..]),
        new_path,
    )
}

fn write_relative_json(reltive_path: &str, data: &String) {
    let path = get_dylib_path();
    let mut path = match path {
        None => panic!("The process path could not be determined"),
        Some(path) => path,
    };

    if cfg!(debug_assertions) {
        path.pop();
        path.pop();
    }
    path.pop();

    let mut new_path = PathBuf::new();
    for p in &path {
        // so like for reason the function above gives me \\\\?\\C: on windows so like hack here for it :(
        if p.to_string_lossy().find("C:").is_some() {
            new_path = new_path.join("C:\\");
        } else {
            new_path = new_path.join(p);
        }
    }

    println!("path to executable is {:?}", &new_path);
    let new_path = new_path.join(reltive_path);

    fs::write(&new_path, data).unwrap_or_else(|err| {
        println!(
            "WARNING: failed to write to {:?} because of {:?}",
            new_path, err
        )
    });
}

fn generate_script_data(file: TemplateFileData) -> String {
    let head = format!(
        r#"
        {{
            "Path": "{0}",
            "RunOn": "{1}","#,
        file.name,
        file.run_on.unwrap()
    );

    let body: String = match file.inits {
        Some(inits) => inits
            .iter()
            .map(|content| {
                format!(
                    r#"
            "{}": {{
                "{}": "{}"
            }},
        "#,
                    content.target, content.when, content.function
                )
            })
            .collect(),
        _ => String::from(""),
    };

    let tail = String::from("},");

    head + &body[..] + &tail[..]
}

fn generate_mod_json(name: &String, scripts: String) -> String {
    let head = format!(
        r#"{{
    "Name" : "{}",
    "Description" : "",
    "Version": "0.1.0",
    "LoadPriority": 1,
    
    "ConVars": [
    ],

    "Scripts": [
        {}
    ]
"#,
        name, scripts
    );

    let tail = String::from("}");

    head + &tail[..]
}
