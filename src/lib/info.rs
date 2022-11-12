use serde_derive::{Deserialize, Serialize};

use super::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct InfoJson {
    author: String,
}

pub fn get_project_name(name: &String) -> String {
    let json = &read_relative_json("templates\\info.json").0[..];

    let data: InfoJson = serde_json::from_str(json).expect("someone destroyed info.json");

    let mut name = name.to_owned();
    name.insert(0, '.');
    name.insert_str(0, &data.author[..]);
    name
}

pub fn new_author(author: String) -> Result<(), serde_json::Error> {
    let json = &read_relative_json("templates\\info.json").0[..];

    let mut data: InfoJson = serde_json::from_str(json).expect("someone destroyed info.json");

    data.author = author;

    write_relative_json(
        "templates\\info.json",
        &serde_json::to_string_pretty(&data)?,
    );

    Ok(())
}
