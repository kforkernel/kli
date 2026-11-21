/*
Copyright (c) 2026 @kforkernel
All rights reserved.

Licensed under the kforkernel Use License (KUL) v1.0.

See LICENSE.md for terms and conditions.
*/

use std::io;
use std::fs;
use std::env;



fn main() {

    // collecting user arg
    let args: Vec<String> = env::args().collect();
    
    // checking args
    if args.len() < 2{
        eprintln!("Usage: kli <OUTPUT FILE>");
        std::process::exit(1);
    }

    // creating empty vec
    let mut list: Vec<String> = Vec::new();

    // getting output file
    let output_file: &str = &args[1];

    // getting user input 
    println!("Type, press enter on empty line to quit");

    loop{
        // creating new string 
        let mut input = String::new();

        // reading user input
        io::stdin().read_line(&mut input).expect("Failed to read line!");
    
        // trimming user input
        let trimmed = input.trim();

        // checking if trimmed empty
        if trimmed.is_empty(){
            break;
        }

        // appending to list vector
        list.push(trimmed.to_string());
    
    }

    // join vector items
    let mut file_contents = list.join("\n");

    // append newline
    file_contents.push('\n');    

    // write to file
    if let Err(e) = fs::write(output_file, &file_contents) {
        eprintln!("Failed to write file: {e}");
        std::process::exit(1);
    }

    println!("Saved to {output_file}");
}



