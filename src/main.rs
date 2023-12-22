use std::{collections::HashMap, env, fs, io, path::PathBuf};

// get the list of directories and files in pwd
// loop through every item and do the following
// if the item is a file get the size and add print it in MB
// if the item is a directory, get the size of the content and print it
//
fn main() {
    let cwd = get_directory_from_args();
    // println!("current working directory: {}\n", cwd);
    match get_files_in_directory(cwd.as_str()) {
        Ok(file_paths) => {
            let mut map: HashMap<String, f32> = HashMap::new();
            for file_path in file_paths {
                // check if this is a directory or a file
                // if its a file, then print the size
                let file_name = file_path.to_str().unwrap();
                let metadata = file_path.metadata();
                if metadata.is_ok() {
                    let mt = metadata.as_ref();
                    let size: f32;
                    if mt.unwrap().is_file() {
                        size = mt.unwrap().len() as f32;
                    } else {
                        // get the size of each file in the directory
                        size = get_file_sizes_of_directory(file_name);
                    };
                    match file_path.file_name() {
                        Some(file_name) => {
                            map.insert(file_name.to_str().unwrap().to_owned(), size);
                        }
                        None => {
                            println!("cant find the file name");
                        }
                    }
                } else {
                    println!("cant find the metadata")
                }
            }
            let mut vec: Vec<(&String, &f32)> = map.iter().collect();
            vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

            for (key, value) in vec {
                print_size(*value, key);
            }
          }

        
        Err(e) => println!("Error: {}", e),
    }
}

fn print_size(mut size: f32, file_name: &str) {
    size = size / 1000.0;
    if size < 1000.0 {
        println!("{:>10.3}KB - {file_name}", size);
        return;
    }
    size = size / 1000.0;
    if size < 1000.0 {
        println!("{:>10.3}MB - {file_name}", size);
        return;
    }
    size = size / 1000.0;
    println!("{:>10.3}GB - {file_name}", size);
}

fn get_directory_from_args() -> String {
    let pwd = env::current_dir().unwrap();
    let cwd = pwd.as_os_str().to_str().unwrap().to_owned();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("current working directory is being used");
        return cwd;
    }
    let path = &args[1];
    if path.starts_with("/") {
        println!("absolute path is being used");
        return path.to_owned();
    }
    if path.starts_with("./") {
        println!("relative path is being used");
        return format!("{}/{}", cwd, path.replace("./", ""));
    }
    if path == "." {
        println!("current working directory is being used");
        return cwd;
    }
    return format!("{}/{}", cwd, path);
}

// get the sum of the size of the files inside a directory recursively
fn get_file_sizes_of_directory(path: &str) -> f32 {
    let mut size: f32 = 0.0;
    match get_files_in_directory(path) {
        Ok(file_paths) => {
            for file_path in file_paths {
                if file_path.is_symlink() {
                    continue;
                }
                // check if the program have permission to open the file
                match file_path.metadata() {
                    Ok(file_metadata) => {
                        let file_size = file_metadata.len(); //metadata.unwrap().len();
                        let is_file = file_metadata.is_file();
                        if is_file {
                            size = size + file_size as f32;
                        } else {
                            size = size + get_file_sizes_of_directory(file_path.to_str().unwrap());
                        }
                    }
                    Err(e) => {
                        match e.kind() {
                            io::ErrorKind::PermissionDenied => {
                                // println!("Permission denied for file: {}", file_path.to_str().unwrap());
                                continue;
                            }
                            _ => {
                                println!("Error getting metadata: {}", e);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => println!("Error getting the size of a directory {}: {}", path, e),
    }

    return size;
}

// get the list of files and directory names in a folder specified by the path arguments
fn get_files_in_directory(path: &str) -> io::Result<Vec<PathBuf>> {
    // Get a list of all entries in the folder
    let entries = fs::read_dir(path)?;

    // Extract the filenames filePaths the directory entries and store them in a vector
    let file_paths: Vec<PathBuf> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            Some(path)
        })
        .collect();

    Ok(file_paths)
}
