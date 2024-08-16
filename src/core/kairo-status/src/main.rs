use std::{
    fs::{read_dir, read_to_string},
    io::Error,
    path::Path,
};
const CHRONOS: &str = "./.chronos/tree/";

fn parse_directory(dir: &str) -> (Vec<String>, Vec<String>) {
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    if let Ok(entries) = read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let file: String = path.display().to_string();
                    if check(file.as_str()).eq(&false) {
                        files.push(file);
                    }
                } else if path.is_dir() {
                    let dir: String = path.display().to_string();
                    if check(dir.as_str()).eq(&false) {
                        dirs.push(dir.to_string());
                    }
                }
            }
        }
    }
    (dirs, files)
}
fn check(w: &str) -> bool {
    if let Ok(content) = read_to_string("./.ignore") {
        return content.contains(w.to_string().replace("./", "").as_str());
    }
    return false;
}
fn data(dir: Option<String>) -> (Vec<String>, Vec<String>) {
    if let Some(d) = dir {
        return parse_directory(d.as_str());
    }
    parse_directory(".")
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
            println!("\n     @projects\n");
        } else {
            println!("\n     @project\n");
        }
        for directory in &new_project {
            println!("\t\t+ {directory}");
        }
        if new_directories.len() > 1 {
            println!("\n     @dirs\n");
        } else {
            println!("\n     @dir\n");
        }
        for directory in &new_directories {
            println!("\t\t+ {directory}");
        }

        if new_files.len() > 1 {
            println!("\n     @files\n");
        } else {
            println!("\n     @file\n");
        }
        for file in &new_files {
            println!("\t\t+ {file}");
        }
    } else {
        println!("Nothing to compare");
        return Ok(());
    }
    println!("\n     @stats\n");
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
