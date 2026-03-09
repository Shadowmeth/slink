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

fn common_prefix(file_names: &Vec<String>) -> String {
    let s1 = &file_names[0].as_bytes();
    let s2 = &file_names[file_names.len() - 1].as_bytes();
    
    let mut prefix: String = String::from("");
    let short_len = if s1.len() < s2.len() {
        s1.len()
    } else {
        s2.len()
    };
    
    for i in 0..short_len {
        if s1[i] == s2[i] {
            prefix.push(s1[i] as char);
        } else {
            break;
        }
    }

    prefix
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error. Usage: {} <path>", args[0]);
        exit(1);
    }

    let mut file_names: Vec<String> = read_dir(&args);
    if file_names.is_empty() {
        eprintln!("Directory {} is empty. Exiting early...", args[1]);
        exit(0);
    }

    file_names.sort();

    println!("Common prefix is: {}", common_prefix(&file_names));
}
