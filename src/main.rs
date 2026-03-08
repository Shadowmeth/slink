use std::{env, fs, path::Path, process::exit};

fn read_dir(args: &Vec<String>) -> Vec<String> {
    let path = Path::new(&args[1]);
    
    match fs::exists(&path) {
        Ok(true) => {
            if path.is_file() {
                eprintln!("Path is not a directory: {}", &args[1]);
                return vec![];
            }
        },
        Ok(false) => {
            eprintln!("Path does not exist: {}", &args[1]);
            return vec![];
        },
        Err(e) => {
            eprintln!("Unable to query path: {}", e);
            return vec![];
        }
    }

    let mut file_names: Vec<String> = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if let Some(file_name) = file_path.file_name() {
            file_names.push(file_name.to_str().unwrap().to_string());
        }
    }

    file_names
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error. Usage: {} <path>", args[0]);
        exit(1);
    }
    for file_name in read_dir(&args) {
        println!("{}", file_name);
    }
}
