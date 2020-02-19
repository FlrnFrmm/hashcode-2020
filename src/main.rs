use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt");
    match input {
        Ok(input) => {
            let output = solve(input);
            fs::write("output.txt", output)
        }
        Err(e) => Err(e)
    }
}

fn solve(input: String) -> String {
    input.chars().rev().collect::<String>()
}
