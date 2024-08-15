use std::{
    fs::{read_dir, DirEntry, File},
    io::Error,
    path::PathBuf,
};

use tar::Builder;

fn main() -> Result<(), Error> {
    let fichier_tar = File::create("mon_archive.tar").unwrap();
    let mut archive = Builder::new(fichier_tar);

    if let Ok(x) = read_dir("./diff") {
        for y in x {
            let p: DirEntry = y?;
            let ph: PathBuf = p.path();
            if ph.as_path().is_dir() {
                assert!(archive.append_dir(ph.as_path(), ph.as_path()).is_ok());
            } else if ph.as_path().is_file() {
                if let Ok(mut content) = File::open(ph.as_path()) {
                    assert!(archive.append_file(ph.as_path(), &mut content).is_ok());
                }
            }
        }
    }
    archive.finish().unwrap();
    Ok(())
}
