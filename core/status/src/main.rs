use colored_truecolor::Colorize;
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
        for d in w.flatten() {
            let p = d.path();
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
        return (dirs, files);
    }
    if let Some(p) = dir {
        let w = WalkBuilder::new(p.as_str())
            .add_custom_ignore_filename(".ignore")
            .standard_filters(true)
            .threads(4)
            .build();
        for d in w.flatten() {
            let p = d.path();
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
        return (dirs, files);
    }
    (dirs, files)
}

fn cargo_project(p: &Path) -> bool {
    Path::new(format!("{}/Cargo.toml", p.display()).as_str()).exists()
}
fn diff() -> Result<(), Error> {
    let mut new_files: Vec<String> = Vec::new();
    let mut new_directories: Vec<String> = Vec::new();
    let mut modified_files: Vec<String> = Vec::new();
    let mut new_project: Vec<String> = Vec::new();
    let mut projects: Vec<String> = Vec::new();

    let tree: (Vec<String>, Vec<String>) = data(Some(CHRONOS.to_string()));
    let src: (Vec<String>, Vec<String>) = data(None);
    for f in &src.1 {
        if f.ne(&".") {
            let old: String = f.replace("./", CHRONOS);
            if Path::new(old.as_str()).is_file().eq(&false) {
                new_files.push(f.to_string().replace("./", ""));
            } else {
                modified_files.push(f.to_string());
            }
        }
    }
    if tree.0.ne(&src.0) || tree.1.ne(&src.1) {
        for d in &src.0 {
            if d.ne(&".") {
                if cargo_project(Path::new(d)) {
                    projects.push(d.to_string());
                    if Path::new(d.replace("./", CHRONOS).as_str())
                        .exists()
                        .eq(&false)
                    {
                        new_project.push(d.to_string().replace("./", ""));
                    }
                }
                if Path::new(d.replace("./", CHRONOS).as_str())
                    .is_dir()
                    .eq(&false)
                {
                    new_directories.push(d.to_string().replace("./", ""));
                }
            }
        }
        if new_project.len() > 1 {
            println!("\n     {}\n", "@projects".green());
        } else {
            println!("\n     {}\n", "@project".green());
        }
        for directory in &new_project {
            println!("\t\t{} {}", "+".green(), directory.green());
        }
        if new_directories.len() > 1 {
            println!("\n     {}\n", "@dirs".blue());
        } else {
            println!("\n     {}\n", "@dir".blue());
        }
        for directory in &new_directories {
            println!("\t\t{} {}", "+".blue(), directory.blue());
        }

        if new_files.len() > 1 {
            println!("\n    {}\n", "@files".white());
        } else {
            println!("\n    {}\n", "@file".white());
        }
        for file in &new_files {
            println!("\t\t{} {}", "+".white(), file.white());
        }
    } else {
        println!("Nothing to compare");
    }
    println!(
        "\n\t\tNew dirs  : {}\n\t\tNew files : {}\n\t\tModified  : {}",
        new_directories.len(),
        new_files.len(),
        modified_files.len(),
    );
    println!();
    Ok(())
}
fn main() -> Result<(), Error> {
    diff()
}
