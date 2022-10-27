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
    println!("");

    is_valid_file(path_mod_json, true);
    is_valid_file(path_manifest_json, true);
    is_valid_file(path_read_me, true);
    is_valid_file(path_icon_png, false);

    let mut includes: Vec<PathBuf> = Vec::new();

    dir_walk(&path.to_path_buf(), &mut includes);

    for entry in includes {
        let mut path = path_temp.join(&entry);
        // let path: PathBuf = path
        //     .iter()
        //     .filter(|p| {
        //         if p.to_str() == Some(&name[..]) {
        //             println!("{:?}", Some(&name[..]));
        //             return false;
        //         }
        //         true
        //     })
        //     .collect();

        let mut path_dir = path.clone();
        path_dir.pop();

        match fs::create_dir_all(&path_dir) {
            Err(err) => {
                println!("failed to create dirs for {:?} in {:?}", &entry, path_dir);
                println!("because of {}", err);
            }
            Ok(_) => println!("created all dirs for {:?}", entry),
        };

        if path.file_name().is_some() {
            let filename = &*path.file_name().unwrap().to_string_lossy().to_owned();
            if filename == "manifest.json" || filename == "README.md" || filename == "icon.png" {
                path = path_temp.join(&filename);
            }
        }

        match fs::copy(&entry, &path) {
            Err(err) => {
                println!("copying failed of {:?} to {:?}", &entry, path);
                println!("because of {}", err);
            }
            Ok(_) => println!("copied {:?} to {:?}", &entry, path),
        };
    }

    _ = fs::rename(path_temp.join(&name), path_temp.join("mod"));

    println!("");
    println!("everything was copied successfully to a temp folder");
    println!("zipping the folder");
    println!("");

    let path_packed = path.join("packed.zip");

    if path_packed.is_file() {
        println!("packed.zip already exists");
        println!("removing packed.zip");
        fs::remove_file(&path_packed).expect("lmao this code explode when trying to delete a file");
    }

    let writer = match File::create(path_packed) {
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

    match zip_walk(
        &path_temp,
        &(name.to_owned() + "temp"),
        &mut zip,
        &options,
        &mut buffer,
    ) {
        Ok(_) => match zip.finish() {
            Ok(_) => println!("zip packed successfully"),
            Err(err) => println!("zip packing failed because of {:?}", err),
        },
        Err(err) => println!("zip packing failed because of {:?}", err),
    }

    println!("");
    fs::remove_dir_all(path_temp).expect("lmao this code explode when trying to delete a dir");
    println!("cleaned up temp file");
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
    prefix: &String,
    zip: &mut ZipWriter<File>,
    options: &FileOptions,
    buffer: &mut Vec<u8>,
) -> zip::result::ZipResult<()> {
    let walkdir = WalkDir::new(&folder);

    for entry in walkdir.into_iter() {
        if entry.is_err() {
            println!("{:?} error will be ingored", entry)
        }
        let entry = entry.unwrap();

        let path = entry.path();

        if path.is_file() {
            println!("adding file {:?}", path);
            #[allow(deprecated)]
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
