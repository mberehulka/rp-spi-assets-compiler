use std::{path::Path, fs::File, io::{Write, BufReader}};

mod font;
mod image;
mod gif;

fn main() {
    std::fs::create_dir("raw/").ok();
    std::fs::create_dir("compiled/").ok();
    std::fs::OpenOptions::new().create(true).truncate(true).write(true).open("compiled/mod.rs").unwrap();

    let settings: serde_json::Value =
        serde_json::from_reader(BufReader::new(File::open("raw/settings.json").unwrap())).unwrap();
    
    read_dir("raw/", &settings)
}

fn read_dir(path: impl AsRef<Path>, settings: &serde_json::Value) {
    for dir in std::fs::read_dir(path).unwrap() {
        let path = dir.unwrap().path();
        if path.is_dir() {
            read_dir(path, settings)
        } else {
            read_file(path, settings)
        }
    }
}

fn create_file(path: impl AsRef<Path>) -> File {
    let mut path = Path::new("compiled/").join(path.as_ref().strip_prefix("raw/").unwrap().with_extension("rs"));
    let name = path.file_name().unwrap().to_string_lossy().replace(' ', "_").to_lowercase();

    write!(
        std::fs::OpenOptions::new().create(true).append(true).write(true).open("compiled/mod.rs").unwrap(),
        "#[rustfmt::skip]\npub mod {};\n", name.split('.').next().unwrap()
    ).unwrap();

    path = path.with_file_name(name);
    std::fs::OpenOptions::new().create(true).truncate(true).write(true).open(path).unwrap()
}

fn read_file(path: impl AsRef<Path>, settings: &serde_json::Value) {
    let path = path.as_ref();
    println!("File found: {}", path.display());
    match path.extension().unwrap().to_str().unwrap() {
        "jpg" | "png" | "jpeg" => image::read(path),
        "gif" => gif::read(path, settings),
        "ttf" => font::read(path, settings),
        "json" => {},
        ext => println!("Extension: {} not supported.", ext)
    }
}