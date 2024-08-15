use colored_truecolor::Colorize;
use ignore::WalkBuilder;
use similar::{ChangeTag, TextDiff};

use std::{fs::read_to_string, io::Error, path::Path};
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
        for file in &modified_files {
            if let Ok(old) = read_to_string(file.replace("./", CHRONOS).as_str()) {
                if let Ok(new) = read_to_string(file.as_str()) {
                    let diff = TextDiff::from_lines(old.as_str(), new.as_str());
                    for change in diff.iter_all_changes() {
                        match change.tag() {
                            ChangeTag::Delete => {
                                print!("{} {}", "-".red(), change.to_string().red());
                            }
                            ChangeTag::Insert => {
                                print!("{} {}", "+".green(), change.to_string().green());
                            }
                            ChangeTag::Equal => {}
                        }
                    }
                }
            }
        }
    } else {
        println!("Nothing to compare");
    }
    println!("\nrun kairo_commit to commit changes\n");
    Ok(())
}
fn main() -> Result<(), Error> {
    diff()
}
