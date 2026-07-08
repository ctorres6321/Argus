use walkdir::WalkDir;
use std::path::PathBuf;

pub fn collect_files(root: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()){

        let path = entry.path();

        // If any of the paths are .git folders or the target folder we ignore them

        if path.components().any(|component|{
            let name = component.as_os_str();
            name == ".git" || name == "target"
        }){
            continue;
        }

        // Else we just end up adding to our vector of PathBuf
        if path.is_file(){
            files.push(path.to_path_buf());
        }

    }
    files
}



