use std::{
    fs::{self, create_dir},
    path::Path,
    path::PathBuf,
    process::exit,
};

pub fn pack_project(name: String) {
    let path = Path::new(&name);

    let path_mod_json = path.join("mod.json");
    let path_manifest_json = path.join("manifest.json");
    let path_read_me = path.join("README.md");
    let path_temp = path.join("temp");

    create_dir(&path_temp).expect("failed to generate a temp folder");
    println!("generated /temp folder");

    is_valid_file(path_mod_json, true);
    is_valid_file(path_manifest_json, true);
    is_valid_file(path_read_me, true);

    let mut includes = Vec::new();

    for entry in fs::read_dir(path).expect("failed to read target dir :(") {
        match entry {
            Ok(file) => includes.push(file.path()),
            Err(err) => println!("ingoring error caused by faulty entry : {:?}", err),
        }
    }

    for entry in includes {
        let path = path_temp.join(match entry.as_path().file_name() {
            None => {
                println!("failed to get a new path; error will be ignored");
                continue;
            }
            Some(name) => name,
        });

        match fs::copy(&entry, &path) {
            Err(err) => println!(
                "copying failed of {:?} to {:?} because of {}",
                entry, path, err
            ),
            Ok(_) => println!("copied {:?} to {:?} successfully", entry, path),
        };
    }

    println!("everything was copied successfully to a temp folder");
    // println!("zipping the folder");
    
    println!("zip the temp folder your self :)");

    // if cfg!(target_os = "windows") {
    //     zip_windows(&path_temp, &name)
    // } else {
    //     println!("unsupported os; so zip it manually");
    //     exit(0);
    // }

    // fs::remove_dir_all(path_temp).expect("lmao this code explode when trying to delete a dir");
    // println!("cleaned up temp file");
}

fn is_valid_file(path: PathBuf, should_exit: bool) {
    if !path.is_file() {
        println!("couldn't find file at {:?}", path);
        if should_exit {
            exit(0)
        } else {
            println!("error ignored")
        }
    }
}

// fn zip_windows(path: &PathBuf, name: &String) {
//     let path = path.join("/");

//     match Command::new("powershell")
//         .args([
//             &format!("Compress-Archive {} {}", path.to_str().unwrap(), name.to_owned() + ".zip"),
//         ])
//         .output()
//     {
//         Ok(out) => println!("zipped successfully : {:?}", out),
//         Err(err) => println!("zip failed : {}", err),
//     }
// }
