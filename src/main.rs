use std::collections::HashMap;
use std::{fs, io};

#[derive(Debug)]
struct Metadata {
    books_count: usize,
    library_count: usize,
    days: usize,
}

impl Metadata {
    pub fn new(values: Vec<i32>) -> Self {
        Self {
            books_count: values[0] as usize,
            library_count: values[1] as usize,
            days: values[2] as usize,
        }
    }
}

#[derive(Debug)]
struct Library {
    signup_days: i32,
    books_per_day: i32,
    books: Vec<i32>,
}

impl Library {
    pub fn new(library_metadata: Vec<i32>, books: Vec<i32>) -> Self {
        Self {
            signup_days: library_metadata[1],
            books_per_day: library_metadata[2],
            books: books,
        }
    }
}

fn main() -> io::Result<()> {
    let test_cases_name = "a_example";
    let input = fs::read_to_string(format!("{}.txt", test_cases_name));
    match input {
        Ok(input) => {
            let output = solve(input);
            fs::write(format!("{}_output.txt", test_cases_name), output)
        }
        Err(e) => Err(e),
    }
}

fn solve(input: String) -> String {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let raw_metadata = parse_line(lines[0]);
    let metadata = Metadata::new(raw_metadata);
    let book_lookup_table = parse_line(lines[1]);
    let libraries = parse_libraries(&metadata, lines);

    let book_scores = calc_book_scores(&metadata, book_lookup_table, &libraries);

    generate_output(book_scores)
}

fn calc_book_scores(
    meta: &Metadata,
    book_lut: Vec<i32>,
    libs: &Vec<Library>,
) -> Vec<(i32, Vec<i32>)> {
    let mut days_left = meta.days;

    let mut already_started: Vec<Library> = Vec::with_capacity(libs.len());

    while libs.len() > 0 {
        let (mut max_score, mut index) = (0, 0);
        for (i, lib) in libs.iter().enumerate() {
            let days_left_to_pull_books = days_left - lib.signup_days as usize;
            let mut book = get_book_scores(&lib.books, &book_lut);
            book.sort();
            book.reverse();
            let current_max_score: i32 = book
                .iter()
                .take(days_left_to_pull_books * lib.books_per_day as usize)
                .sum();
            if current_max_score > max_score {
                index = i;
                max_score = current_max_score;
            }
            println!("{:?}", current_max_score);
        }
        println!("{} {}", index, max_score);
        let next_lib = libs.remove(index);
        already_started.push(libs.remove(index))
    }

    Vec::new()
}

// Helpers
fn parse_line(s: &str) -> Vec<i32> {
    s.split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn get_book_scores(books: &Vec<i32>, book_lut: &Vec<i32>) -> Vec<i32> {
    books
        .iter()
        .map(|i| book_lut[*i as usize])
        .collect::<Vec<i32>>()
}

fn parse_libraries(metadata: &Metadata, lines: Vec<&str>) -> Vec<Library> {
    let mut libraries: Vec<Library> = Vec::with_capacity(metadata.library_count);
    for library_id in 0..metadata.library_count {
        let start_index = 2 + library_id * 2 as usize;
        let library_metadata = parse_line(lines[start_index]);
        libraries.push(Library::new(
            library_metadata,
            parse_line(lines[start_index + 1]),
        ));
        println!("{:?}", libraries[library_id]);
    }
    libraries
}

fn generate_output(book_scores: Vec<(i32, Vec<i32>)>) -> String {
    let mut output = format!("{}\n", book_scores.len().to_string());

    for (k, v) in book_scores.iter() {
        output = format!("{}{}", output, format!("{} {}\n", k, v.len()));
        output = format!(
            "{}{}\n",
            output,
            v.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
    output
}
