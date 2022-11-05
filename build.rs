#[cfg(feature = "cargo_templates")]
use {
    home::cargo_home,
    std::{fs, path::PathBuf},
    walkdir::WalkDir,
};

fn main() {
    #[cfg(feature = "cargo_templates")]
    {
        println!(
            "cargo:warning=copying templates dir to {:?}",
            cargo_home().unwrap()
        );

        let mut path = PathBuf::from(cargo_home().unwrap());
        path.push("bin");

        templates_walk(&path, &PathBuf::from("templates"));
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=templates/templates.json");
}

#[cfg(feature = "cargo_templates")]
fn templates_walk(target_path: &PathBuf, templates_path: &PathBuf) {
    let walkdir = WalkDir::new(templates_path);

    for entry in walkdir.into_iter() {
        match entry {
            Ok(file) => {
                let path = file.path();
                let mut tpath = target_path.clone();
                tpath.push(&path);

                println!("{:?} to {:?}", &path, &tpath);

                if path.is_file() {
                    match fs::copy(&path, &tpath) {
                        Err(err) => {
                            println!("failed to create file {:?}", &tpath);
                            println!("{:?}", err);
                            panic!()
                        }
                        Ok(_) => println!("created file {:?}", &tpath),
                    }
                } else if path.is_dir() {
                    match fs::create_dir_all(&tpath) {
                        Err(err) => {
                            println!("failed to create folder {:?}", &tpath);
                            println!("{:?}", err);
                            panic!()
                        }
                        Ok(_) => println!("created folder {:?}", &tpath),
                    }
                }
            }
            Err(_) => panic!("failied to move templates to bin, cannot proccesed further"),
        }
    }
}
