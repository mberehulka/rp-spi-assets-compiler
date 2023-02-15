use std::path::Path;

fn main() {
    std::fs::create_dir("raw/").ok();
    std::fs::create_dir("compiled/").ok();
    read_dir("raw/")
}

fn read_dir(path: impl AsRef<Path>) {
    for dir in std::fs::read_dir(path).unwrap() {
        let path = dir.unwrap().path();
        if path.is_dir() {
            read_dir(path)
        } else {
            read_file(path)
        }
    }
}

fn read_file(path: impl AsRef<Path>) {
    let path = path.as_ref();
    println!("File found: {}", path.display())
}