use std::{
    fs::{self},
    io::Error,
};
fn display_tree(dir: &str, prefix: &str) -> Result<(), Error> {
    if !dir.contains(".git") && !dir.contains(".chronos") && !dir.contains("target") {
        if let Ok(entries) = fs::read_dir(dir) {
            let entries_vec: Vec<_> = entries.collect();

            for (i, entry) in entries_vec.iter().enumerate() {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let is_last = i == entries_vec.len() - 1;

                    if is_last {
                        println!("{}└── {}", prefix, entry.file_name().to_string_lossy());
                    } else {
                        println!("{}├── {}", prefix, entry.file_name().to_string_lossy());
                    }
                    if path.is_dir() {
                        let new_prefix = if is_last {
                            format!("{}    ", prefix)
                        } else {
                            format!("{}│   ", prefix)
                        };
                        let _ = display_tree(path.to_str().unwrap(), &new_prefix);
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    display_tree(".", "")
}
