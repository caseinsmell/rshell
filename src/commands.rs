use std::process;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fs;   
use crate::utils::debug;

pub fn builtin_echo(args : &Vec<String>) -> i32 {
    println!("{}", args.join(" "));
    0
}

pub fn builtin_exit() -> i32 {
    println!("Goodbye good friend :)");
    process::exit(0)
}

pub fn builtin_cat(args : &Vec<String>) -> i32 {
    //TODO : not panic if we can't read the file

    // reads every given file
    for arg in args {
        debug(format!("reading the file {}", arg));

        // Create a path to the desired file
        let path = Path::new(arg);
        let display = path.display();

        //TODO: find a way to print out the error instead of panicking
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => println!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{}\n {}", display, s),
        }
    }
    0
}

pub fn builtin_pwd() -> i32 {
    match env::var("PWD") {
        Ok(val) => println!("{}", val),
        Err(_) => println!("Something went wrong when getting your current dir"),
    }
    0
}

pub fn builtin_rm(args : &Vec<String>) -> i32 {
    for arg in args {
        
        let meta = fs::metadata(arg).unwrap();
      
        
        if meta.is_dir() {

            debug(format!("Removing the directory {}", arg));
            let dir_removed = fs::remove_dir_all(arg);

            match dir_removed {
                Ok(_) => println!("Directory successfuly removed"),
                Err(_) => println!("ERR: could not delete directory"),
            }

        } else if meta.is_file() {

            debug(format!("Removing the file {}", arg));
            let file_removed = fs::remove_file(arg);

            match file_removed {
                Ok(_) => println!("File successfuly removed"),
                Err(_) => println!("ERR: could not delete file"),
            }

        } else {
            println!("Could not identify file {}", arg);
        }

    } 
    -1
}
