use ignore::WalkBuilder;
use std::{io::Error, path::Path};
const CHRONOS: &str = "./.chronos/tree/";

fn data(dir: Option<String>) -> (Vec<String>, Vec<String>) {
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    if dir.is_none() {
        let w = WalkBuilder::new(".")
            .add_custom_ignore_filename(".ignore")
            .standard_filters(true)
            .threads(4)
            .build();
        for d in w {
            if let Ok(x) = d {
                let p = x.path();
                if p.is_dir() {
                    if let Some(d) = p.to_str() {
                        dirs.push(d.to_string());
                    }
                } else if p.is_file() {
                    if let Some(d) = p.to_str() {
                        files.push(d.to_string());
                    }
                }
            }
        }
        return (dirs, files);
    }
    if let Some(p) = dir {
        let w = WalkBuilder::new(p.as_str())
            .add_custom_ignore_filename(".ignore")
            .standard_filters(true)
            .threads(4)
            .build();
        for d in w {
            if let Ok(x) = d {
                let p = x.path();
                if p.is_dir() {
                    if let Some(d) = p.to_str() {
                        dirs.push(d.to_string());
                    }
                } else if p.is_file() {
                    if let Some(d) = p.to_str() {
                        files.push(d.to_string());
                    }
                }
            }
        }
        return (dirs, files);
    }
    (dirs, files)
}
fn diff() -> Result<(), Error> {
    let mut new_files: Vec<String> = Vec::new();
    let mut new_directories: Vec<String> = Vec::new();

    let tree: (Vec<String>, Vec<String>) = data(Some(CHRONOS.to_string()));
    let src: (Vec<String>, Vec<String>) = data(None);
    if tree.0.ne(&src.0) || tree.1.ne(&src.1) {
        for d in &src.0 {
            if d.ne(&".") {
                if Path::new(d.replace("./", CHRONOS).as_str())
                    .is_dir()
                    .eq(&false)
                {
                    new_directories.push(d.to_string().replace("./", ""));
                }
            }
        }
        if new_directories.len() > 1 {
            println!("\nNew directories : \n");
        } else {
            println!("\nNew directory : \n");
        }
        for directory in &new_directories {
            println!("\t{directory}");
        }
        for f in &src.1 {
            if f.ne(&".") {
                if Path::new(f.replace("./", CHRONOS).as_str())
                    .is_file()
                    .eq(&false)
                {
                    new_files.push(f.to_string().replace("./", ""));
                }
            }
        }

        if new_files.len() > 1 {
            println!("\nNew files : \n");
        } else {
            println!("\nNew file : \n");
        }
        for file in &new_files {
            println!("\t{file}");
        }
    } else {
        println!("Nothing to compare");
    }
    println!(
        "\nNew dirs  : {}\nNew files : {}\n",
        new_directories.len(),
        new_files.len(),
    );
    Ok(())
}
fn main() -> Result<(), Error> {
    diff()
}
