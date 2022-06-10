use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        println!("Usage: getstartproj path");
        return;
    }
    let arg = &args[0];
    let path = Path::new(arg);
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
        process_sln(path, original_path);
    }
}

fn process_sln(sln: &Path, original_path: &Path) {
    if let Ok(path) = sln.strip_prefix(original_path) {
        if let Some(file_name) = path.file_name() {
            if let Some(file_name) = file_name.to_str() {
                if file_name != "" {
                    println!("{}:", path.display());
                }
            }
        }
    }
    get_start_projects(sln)
}

fn get_start_projects(_sln: &Path) {
    println!("\t(root projects will be presented here)");
}
