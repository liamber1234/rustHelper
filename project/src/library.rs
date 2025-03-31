mod book;
use crate::book::Book;

pub struct Library {
    librarian: String,
    books: Vec<Book>,
    location: String,
}

impl Library {

    pub fn new() -> Library {
        Library {
            librarian: String::new(),
            books: Vec::new(),
            location: String::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    
    pub fn remove_book(&mut self, book: Book) {
        self.books.remove(self.find_index(&book).try_into().unwrap());
    }
    
    pub fn list_books(&mut self) {
        for book in &mut self.books {
            book.print_book_info();
        }
    }
    
    pub fn rent_book(&mut self, title : &str) {
        for book in &mut self.books {
            if book.title == title {
                if book.is_avalible {
                    book.is_avalible = false;
                    println!("You have rented the book: {}", book.title);
                    return;
                } else {
                    println!("The book is not available for rent.");
                    return;
                }
                return;
            }
        }
        println!("Book not found");
    }
    
    pub fn return_book(&mut self, title : &str) {
        for book in &mut self.books {
            if book.title == title {
                book.is_avalible = true;
                println!("You have returned the book: {}", book.title);
                return;
            }
        }
        println!("Book not found");
    }

    pub fn find_index(&self, book: &Book) -> i32 {
        for (i, b) in self.books.iter().enumerate() {
            if b.title == book.title && b.author == book.author {
                return i as i32;
            }
        }
        return -1;
    }
}
