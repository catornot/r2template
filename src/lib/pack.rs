use std::{
    fs::{self, create_dir, File},
    io::{Read, Write},
    path::Path,
    path::PathBuf,
    process::exit,
};
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::ZipWriter;

pub fn pack_project(name: String) {
    let path = Path::new(&name);

    let path_mod_json = path.join("mod.json");
    let path_manifest_json = path.join("manifest.json");
    let path_read_me = path.join("README.md");
    let path_icon_png = path.join("icon.png");
    let path_temp = path.join("temp");

    is_valid_file(path_mod_json, true);
    is_valid_file(path_manifest_json, true);
    is_valid_file(path_read_me, true);
    is_valid_file(path_icon_png, false);

    if path_temp.is_dir() {
        println!("/temp already exists");
        println!("removing /temp");
        fs::remove_dir_all(&path_temp).expect("lmao this code explode when trying to delete a dir");
    }

    match create_dir(&path_temp) {
        Ok(_) => println!("generated /temp folder"),
        Err(err) => {
            println!("couldn't create {:?}", path_temp);
            println!("{:?}", err);
            exit(0)
        }
    };
    match create_dir(path_temp.join("mods")) {
        Ok(_) => println!("generated /temp/mods folder"),
        Err(err) => {
            println!("couldn't create mods folder");
            println!("{:?}", err);
            exit(0)
        }
    };
    println!();

    let mut includes: Vec<PathBuf> = Vec::new();

    dir_walk(&path.to_path_buf(), &mut includes);

    let mod_path = path_temp.join("mods");
    for entry in includes {
        let path = path_temp.join(&entry);

        let path = if path.file_name().is_some() {
            let filename = &*path.file_name().unwrap().to_string_lossy().to_owned();
            let org_path = &entry;
            if filename != "manifest.json" && filename != "README.md" && filename != "icon.png" {
                mod_path.join(org_path)
            } else {
                path_temp.join(filename)
            }
        } else {
            path
        };

        let mut path_dir = path.clone();
        path_dir.pop();

        match fs::create_dir_all(&path_dir) {
            Err(err) => {
                println!("failed to create dirs for {:?} in {:?}", &entry, path_dir);
                println!("because of {}", err);
            }
            Ok(_) => println!("created all dirs for {:?}", entry),
        };

        match fs::copy(&entry, &path) {
            Err(err) => {
                println!("copying failed of {:?} to {:?}", &entry, path);
                println!("because of {}", err);
            }
            Ok(_) => println!("copied {:?} to {:?}", &entry, path),
        };
    }

    _ = fs::rename(path_temp.join(&name), path_temp.join("mods"));

    println!();
    println!("everything was copied successfully to a temp folder");
    println!("zipping {:?}", path_temp);
    println!();

    let path_packed = path.join("packed.zip");

    if path_packed.is_file() {
        println!("packed.zip already exists");
        println!("removing packed.zip");
        fs::remove_file(&path_packed).expect("lmao this code explode when trying to delete a file");
    }

    let writer = match File::create(&path_packed) {
        Ok(file) => file,
        Err(err) => {
            println!("couldn't create packed.zip");
            println!("{:?}", err);
            exit(0)
        }
    };

    let mut zip = Box::new(zip::ZipWriter::new(writer));
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();

    match zip_walk(&path_temp, name.to_owned(), &mut zip, &options, &mut buffer) {
        Ok(_) => match zip.finish() {
            Ok(_) => println!("zip packed successfully"),
            Err(err) => println!("zip packing failed because of {:?}", err),
        },
        Err(err) => println!("zip packing failed because of {:?}", err),
    }

    println!();
    fs::remove_dir_all(path_temp).expect("lmao this code explode when trying to delete a dir");
    println!("cleaned up temp file");

    // fs::remove_file(&path_packed).expect("lmao this code explode when trying to delete a file");

    // println!("currently zipping is broken so pls consider opening a pr to fix or zip it yourself, then delete the temp folder")
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

fn dir_walk(path: &PathBuf, includes: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(path).expect("failed to read target dir :(") {
        match entry {
            Ok(file) => {
                let path = file.path();
                if path.is_file() {
                    includes.push(file.path())
                } else {
                    dir_walk(&path, includes)
                }
            }
            Err(err) => println!("ingoring error caused by faulty entry : {:?}", err),
        }
    }
}

fn zip_walk(
    folder: &PathBuf,
    prefix: String,
    zip: &mut ZipWriter<File>,
    options: &FileOptions,
    buffer: &mut Vec<u8>,
) -> zip::result::ZipResult<()> {
    let walkdir = WalkDir::new(folder);

    for entry in walkdir.into_iter() {
        if entry.is_err() {
            println!("{:?} error will be ingored", entry)
        }
        let entry = entry.unwrap();

        let path = entry.path();
        let path = path.strip_prefix(&prefix).unwrap();

        if path.is_file() {
            println!("adding file {:?}", path);
            zip.start_file(path.to_str().unwrap(), *options)?;
            let mut f = File::open(path)?;

            f.read_to_end(buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if path.is_dir() {
            println!("adding dir {:?}", path);
            zip.add_directory(path.to_str().unwrap(), *options)?;
        }
    }

    Ok(())
}
