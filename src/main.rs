use std::{fs, io, env};

// get the list of directories and files in pwd
// loop through every item and do the following
// if the item is a file get the size and add print it in MB
// if the item is a directory, get the size of the content and print it
//
fn main() {
  // let y = 1.2345f64;
  // let x = 1.2346f64;
  // if x != y {
    // println!("x is not equal to y");
  // }
  // let my_name: &str = "Rust is the language i am learning now ain't vim just so awesome?";
  // unwrap kinda handle the error (I think)
  let pwd = env::current_dir().unwrap();
  // println!("first pwd: {}", pwd.display());
  let cwd = pwd.as_os_str().to_str().unwrap();
  // println!("first pwd: {}", pwd.unwrap().as_os_str().to_str().unwrap());
  // match env::current_dir() {
    // Ok(cwd) => {
      // pwd = cwd.as_os_str().to_str().unwrap();
//
      // println!("pwd: {}", pwd);
    // },
    // Err(e) => println!("Error: {}", e),
  // }
  println!("first pwd: {}", cwd);
  match get_files_in_directory(cwd) {
        Ok(file_names) => {
            for file_name in file_names {
              // check if this is a directory or a file
              // if its a file, then print the size
                println!("{}/{}", cwd, file_name);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    //.into_string().unwrap();
  // let files = get_files_in_directory(pwd);
  // println!("Hello, {}!", my_name);
}


fn get_files_in_directory(path: &str) -> io::Result<Vec<String>> {
    // Get a list of all entries in the folder
    let entries = fs::read_dir(path)?;

    // Extract the filenames from the directory entries and store them in a vector
    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            // if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            // } else {
                // None
            // }
        })
        .collect();

    Ok(file_names)
}
