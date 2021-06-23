use std::process;
use std::io::prelude::*;
use std::fs::File;
use crate::utils::debug;

// Implementation of echo
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
    //TODO : read all the files in the args

    for arg in args {
        debug(format!("reading the file {}", arg));

        let mut file = File::open(&arg)
            .expect("Could not read the file");

        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Could not read the contents of the file");

        println!("{}", contents);
    }
    0
}
