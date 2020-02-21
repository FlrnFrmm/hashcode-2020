use crate::library::Library;

#[derive(Debug)]
pub struct Metadata {
    books_count: usize,
    library_count: usize,
    pub days: usize,
}

impl Metadata {
    fn new(values: Vec<usize>) -> Self {
        Self {
            books_count: values[0],
            library_count: values[1],
            days: values[2],
        }
    }
}

pub fn parse_input(input: String) -> (Metadata, Vec<Library>) {
    let lines: Vec<&str> = input.lines().collect();
    
    let metadata_values = parse_line(lines[0]);
    let metadata = Metadata::new(metadata_values);

    let libraries = parse_libraries(&metadata, &lines[1..]);

    (metadata, libraries)
}

fn parse_libraries(metadata: &Metadata, lines: &[&str]) -> Vec<Library> {
    let book_lut = parse_line(lines[0]);
    let mut libraries = Vec::with_capacity(metadata.library_count);
    for library_id in 0..metadata.library_count {
        let start_index = 1 + library_id * 2 as usize;
        let library_metadata = parse_line(lines[start_index]);
        let book_ids = parse_line(lines[start_index + 1]);
        let books: Vec<(usize, usize)> = book_ids.iter()
            .map(|id| (*id, book_lut[*id as usize])).collect();
        libraries.push(Library::new(
            library_metadata,
            books
        ));
    }
    libraries
}

pub fn generate_output(book_scores: Vec<(i32, Vec<i32>)>) -> String {
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

fn parse_line(s: &str) -> Vec<usize> {
    s.split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}
