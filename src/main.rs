use std::{env, fs, io};

// get the list of directories and files in pwd
// loop through every item and do the following
// if the item is a file get the size and add print it in MB
// if the item is a directory, get the size of the content and print it
//
fn main() {
    let cwd = get_directory_from_args();
    println!("current working directory: {}\n", cwd);
    match get_files_in_directory(cwd.as_str()) {
        Ok(file_names) => {
            for file_name in file_names {
                // check if this is a directory or a file
                // if its a file, then print the size
                let filepath = format!("{cwd}/{file_name}");
                let metadata = fs::metadata(filepath);
                if metadata.is_ok() {
                    let mt = metadata.as_ref();
                    let size: f32;
                    if mt.unwrap().is_file() {
                        size = mt.unwrap().len() as f32;
                    } else {
                        // get the size of each file in the directory
                        size = get_file_sizes_of_directory(
                            String::from(format!("{cwd}/{file_name}")).as_str(),
                        );
                    };
                    println!("{:.3}KB - {file_name}", size / 1000.0);
                } else {
                    println!("cant find the metadata")
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn get_directory_from_args() -> String {
    let pwd = env::current_dir().unwrap();
    let cwd = pwd.as_os_str().to_str().unwrap().to_owned();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("current working directory is being used");
        // TODO: make sure not to use the current working directory if the path
        // begins with /
        return cwd;
    }
    let path = &args[1];
    println!("path: {}", path);
    return format!("{}/{}", cwd, path);
}

// get the sum of the size of the files inside a directory recursively
fn get_file_sizes_of_directory(path: &str) -> f32 {
    let mut size: f32 = 0.0;
    match get_files_in_directory(path) {
        Ok(file_names) => {
            for file_name in file_names {
                let filepath = format!("{path}/{file_name}");
                let metadata = fs::metadata(filepath.clone());
                let mt = metadata.as_ref();
                let file_size = mt.unwrap().len(); //metadata.unwrap().len();
                let is_file = mt.unwrap().is_file();
                if is_file {
                    // println!("{:.3}KB - {file_name}", size as f32/1000.0);
                    size = size + file_size as f32;
                } else {
                    size = size + get_file_sizes_of_directory(&filepath);
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    return size;
}

// get the list of files and directory names in a folder specified by the path arguments
fn get_files_in_directory(path: &str) -> io::Result<Vec<String>> {
    // Get a list of all entries in the folder
    let entries = fs::read_dir(path)?;

    // Extract the filenames from the directory entries and store them in a vector
    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            // if path.is_file() {
            // let f = path.file();//.unwrap().len()
            // let f = entry.ok()?.file();
            path.file_name()?.to_str().map(|s| s.to_owned())
            // } else {
            // None
            // }
        })
        .collect();

    Ok(file_names)
}
