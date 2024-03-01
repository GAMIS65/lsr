use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let mut directories = Vec::new();
    let mut files = Vec::new();

    for entry in fs::read_dir(Path::new("."))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            directories.push(path);
        } else {
            files.push(path);
        }
    }

    // Print directories first
    for directory in directories {
        println!("{}", directory.display());
    }

    for file in files {
        println!("{}", file.file_name().unwrap().to_string_lossy());
    }

    Ok(())
}
