use std::env;
use std::process;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        println!("Usage: getstartproj path");
        process::exit(1);
    }
    let arg = &args[0];
    let path = Path::new(arg);
    if !path.exists() {
        println!("Error: the specified path does not exist.");
        process::exit(2);
    }
    process(path, path);
}

fn process(path: &Path, original_path: &Path) {
    if path.is_file() {
        process_file(&path, original_path);
    } else if path.is_dir() {  
        if let Ok(dir) = path.read_dir() {
            for entry in dir {
                if let Ok(entry) = entry {
                    process(&entry.path(), original_path)
                }
            }
        }
    }
}

fn process_file(path: &Path, original_path: &Path) {
    if path.extension() == Some(OsStr::new("sln")) {
        process_sln_file(path, original_path);
    }
}

fn process_sln_file(sln_path: &Path, original_path: &Path) {
    if let Ok(path) = sln_path.strip_prefix(original_path) {
        if let Some(file_name) = path.file_name() {
            if let Some(file_name) = file_name.to_str() {
                if file_name != "" {
                    println!("{}:", path.display());
                }
            }
        }
    }
    get_start_projects(sln_path)
}

fn get_start_projects(_sln_path: &Path) {
    println!("\t(root projects will be presented here)");
}
