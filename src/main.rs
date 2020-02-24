mod library;
mod parser;

use std::{fs, io};
use crate::parser::*;
use crate::library::*;


fn main() -> io::Result<()> {
    let test_cases_name = "a_example";
    let input = fs::read_to_string(format!("input/{}.txt", test_cases_name));
    match input {
        Ok(input) => {
            let (metadata, mut libraries) = parser::parse_input(input);
            let output = solve(metadata, &mut libraries);
            fs::write(format!("{}_output.txt", test_cases_name), output)
        }
        Err(e) => Err(e),
    }
}

fn solve(metadata: Metadata, libraries: &mut Vec<Library>) -> String {
    for library in libraries.iter() {
        println!("{:?}", library);
    }

    for _ in 0..metadata.days + 1 {
        for lib in libraries.iter_mut() {
            match lib.state {
                LibraryState::SignUpNotStarted => lib.start_signup(),
                LibraryState::SignUpStarted(days_left) => {
                    if days_left > 1 {
                    lib.state = LibraryState::SignUpStarted(days_left - 1); 
                    } else {
                        lib.state = LibraryState::SignedUp;
                    }
                },
                LibraryState::SignedUp => {
                    println!("{:?}", lib.pull_books()); 
                },
                LibraryState::Empty => {}
            }
        }
    } 

    String::from("result")
}



