use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;
use std::{thread, time::Duration};

fn prompt() -> String {
    let mut write = String::new();
    io::stdin()
        .read_line(&mut write)
        .expect("failed to read line");
    write
}

fn path_name() -> String {
    let mut write_path = String::new();
    io::stdin()
        .read_line(&mut write_path)
        .expect("failed to read line");
    write_path
}

fn main() {
    println!("What would you like to do?");
    println!("1. Write a new file\n2. Open an existing file");
    let choice = prompt();
    if choice.trim() == "1" {
        let file_name = get_file_name();
        file_writing(&file_name);
    } else if choice.trim() == "2" {
        let file_name = choose_file();
        file_open(&file_name);
    } else {
        println!("Invalid input.")
    }
}

fn file_writing(file_name: &str) {
    // file writing
    println!("Write your file name");
    let _path_naming = path_name();
    let path = Path::new(&file_name);
    let display = path.display();
    println!("Contents will be written in {}", display);
    thread::sleep(Duration::from_secs(2));
    println!("Write the contents of {}", display);
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldnt create {}: {}", display, why),
        Ok(file) => file,
    };
    let start = Instant::now();
    let text = prompt();
    thread::sleep(Duration::from_secs(2));
    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("Couldnt write to {}: {}", display, why),
        Ok(_) => println!("Succesfully wrote to {}", display),
    }
    println!("Time spent writing {}: {:?}", display, start.elapsed());
    return;
}

fn file_open(file_name: &str) {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldnt open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldnt read {}: {}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }
}

fn get_file_name() -> String {
    format!("{}.txt", prompt().trim())
}

fn list_files() -> Vec<String> {
    let mut names = Vec::new();
    for entry in fs::read_dir(".").expect("Couldnt read directory") {
        let entry = entry.expect("Couldnt read entry");
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".txt") {
            names.push(name)
        }
    }
    names
}

fn choose_file() -> String {
    let files = list_files();

    for (i, name) in files.iter().enumerate() {
        println!("{}. {}", i + 1, name);
    }

    let choice: usize = prompt().trim().parse().expect("please enter a number");
    files[choice - 1].clone()
}
