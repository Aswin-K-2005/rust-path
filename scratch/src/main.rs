use std::{fs, path::Path};

fn list(dir: &Path) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.ends_with("target") {
            continue;
        }
        if path.is_dir() {
            list(&path);
        } else {
            println!("{}", path.display())
        }
    }
}

fn main() {
    list(Path::new("."));
}
