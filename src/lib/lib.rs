use std::{fs, path::PathBuf};

use clap::Parser;
use process_path::get_dylib_path;

use super::info::new_author;
use super::new::new_project;

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

impl Cli {
    pub fn commands() {
        match Self::parse().author {
            Some(arg) => match new_author(&arg) {
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
            Some(arg) => new_project(arg, template),
            _ => print!(""),
        }
    }
}

fn get_project_root() -> PathBuf {
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
    };

    new_path
}

pub fn read_relative_json(reltive_path: &str) -> (String, PathBuf) {
    let path = get_project_root();

    println!("path to executable is {:?}", &path);
    let path = path.join(reltive_path);

    (
        fs::read_to_string(&path)
            .expect(&format!("failed to read json file : {:?}", path)[..]),
            path,
    )
}

pub fn write_relative_json(reltive_path: &str, data: &String) {
    let path = get_project_root();

    println!("path to executable is {:?}", &path);
    let path = path.join(reltive_path);

    fs::write(&path, data).unwrap_or_else(|err| {
        println!(
            "WARNING: failed to write to {:?} because of {:?}",
            path, err
        )
    });
}
