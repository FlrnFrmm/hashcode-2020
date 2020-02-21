use std::collections::vec_deque::*;

type Book = (usize, usize);

#[derive(Debug,PartialEq)]
pub enum LibraryState {
    SignUpNotStarted,
    SignUpStarted(usize),
    SignedUp,
    Empty
}

#[derive(Debug)]
pub struct Library {
    pub state: LibraryState,
    pub books: VecDeque<Book>,
    signup_days: usize,
    books_per_day: usize,
}

impl Library {
    pub fn new(library_metadata: Vec<usize>, mut books: Vec<Book>) -> Self {
        books.sort_by(|(_, score_one), (_, score_two)| score_two.cmp(score_one));
        Self {
            state: LibraryState::SignUpNotStarted,
            signup_days: library_metadata[1],
            books_per_day: library_metadata[2],
            books: VecDeque::from(books)
        }
    }

    pub fn start_signup(&mut self) {
        if self.state == LibraryState::SignUpNotStarted {
            self.state = LibraryState::SignUpStarted(self.signup_days);
        }
    }

    pub fn pull_books(&mut self) -> Vec<Book> {
        let mut pulled_books = Vec::with_capacity(self.signup_days);

        for _ in 0..self.signup_days {
            match self.books.pop_front() {
                Some(book) => pulled_books.push(book),
                None => {
                    self.state = LibraryState::Empty;
                    break;
                }
            }
        }

        pulled_books
    }
}
