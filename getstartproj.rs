use std::env;
use std::process;
use std::path::Path;
use std::ffi::OsStr;
use std::fs;

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

fn get_start_projects(sln_path: &Path) {
    let project_files = get_project_paths(sln_path);
    println!("\t(temporary count of projects: {})", project_files.len()); // will be replaced later
}

fn get_project_paths(sln_path: &Path) -> Vec<String> {
    let mut project_paths = Vec::<String>::new();
    if let Ok(contents) = fs::read_to_string(sln_path) {
        for line in contents.lines() {
            let project_indicator = "Project";
            let project_indicator_len = project_indicator.len();
            let csproj_indicator = ".csproj";
            let vbproj_indicator = ".vbproj";
            let start_separator = ", \"";
            let end_separator = "\", ";
            if line.starts_with(project_indicator) && (line.contains(csproj_indicator) || line.contains(vbproj_indicator)) {
                if let Some(start_index) = line[project_indicator_len..].find(start_separator) {
                    let start_index = project_indicator_len + start_index + start_separator.len();
                    if let Some(end_index) = line[start_index..].find(end_separator) {
                        let end_index = start_index + end_index;
                        let project_path = &line[start_index..end_index];
                        project_paths.push(project_path.to_string());
                    }
                }
            }
        } 
    }
    return project_paths;
}
