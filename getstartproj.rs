use std::env;
use std::process;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Usage: getstartproj path");
        process::exit(1);
    }
    let arg = &args[0];
    let path = Path::new(arg);
    if !path.exists() {
        eprintln!("Error: the specified path does not exist.");
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
    let project_paths = get_project_paths(&sln_path);
    let start_project_paths = get_start_project_paths(&project_paths, &sln_path);
    print_paths(&start_project_paths, &sln_path);
    let path_count = start_project_paths.len();
    println!("\t({} start project{})", path_count, if path_count != 1 { "s" } else { "" });
}

fn get_project_paths(sln_path: &Path) -> Vec<PathBuf> {
    let mut project_paths = Vec::<PathBuf>::new();
    if let Ok(contents) = fs::read_to_string(&sln_path) {
        let sln_dir = get_canonical_dir(&sln_path);
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
                        let project_path_str = &line[start_index..end_index];
                        let project_path = PathBuf::from(project_path_str);
                        let project_path = get_canonical_path(&project_path, &sln_path);
                        if project_path.starts_with(&sln_dir) {
                            project_paths.push(project_path);
                        }
                    }
                }
            }
        }
    }
    return project_paths;
}

fn get_start_project_paths(project_paths: &Vec<PathBuf>, sln_path: &Path) -> Vec<PathBuf> {
    let mut start_paths = project_paths.to_vec();
    for project_path in project_paths {
        let dependency_paths = get_project_dependency_paths(&project_path, sln_path);
        start_paths.retain(|path| !dependency_paths.contains(&path));
    }
    return start_paths;
}

fn get_project_dependency_paths(project_path: &Path, sln_path: &Path) -> Vec<PathBuf> {
    let mut project_paths = Vec::<PathBuf>::new();
    if let Ok(contents) = fs::read_to_string(project_path) {
        let sln_dir = get_canonical_dir(&sln_path);
        for line in contents.lines() {
            let project_indicator = "<ProjectReference ";
            let project_indicator_len = project_indicator.len();
            let start_separator = "Include=\"";
            let end_separator = "\"";
            if let Some(project_indicator_index) = line.find(project_indicator) {
                if let Some(start_index) = line[project_indicator_index + project_indicator_len..].find(start_separator) {
                    let start_index = project_indicator_index + project_indicator_len + start_index + start_separator.len();
                    if let Some(end_index) = line[start_index..].find(end_separator) {
                        let end_index = start_index + end_index;
                        let reference_project_path_str = &line[start_index..end_index];
                        let reference_project_path = PathBuf::from(reference_project_path_str);
                        let reference_project_path = get_canonical_path(&reference_project_path, &project_path);
                        if reference_project_path.starts_with(&sln_dir) {
                            project_paths.push(reference_project_path);
                        }
                    }
                }
            }
        } 
    }
    return project_paths;
}

fn get_canonical_dir(path: &Path) -> PathBuf {
    if let Some(parent) = path.parent() {
        if let Ok(dir) = parent.canonicalize() {
            return PathBuf::from(dir);
        }
    }
    return PathBuf::from(path);
}

fn get_canonical_path(path: &Path, root_path: &Path) -> PathBuf {
    if path.is_relative() {
        if let Some(parent) = root_path.parent() {
            let path = parent.join(path);
            if let Ok(path) = path.canonicalize() {
                return PathBuf::from(path);
            }
        }
    }
    return PathBuf::from(path);
}

fn print_paths(paths: &Vec<PathBuf>, root_path: &Path) {
    let root_dir = get_canonical_dir(root_path);
    for path in paths {
        if let Ok(local_path) = path.strip_prefix(&root_dir) {
            println!("\t{}", local_path.display());
        } else {
            println!("\t{}", path.display());
        }
    }
}
