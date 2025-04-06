use crate ::book::Book;

///this struct represents a library and includes the librarian, books, and location of the library
pub struct Library {
    librarian: String,
    books: Vec<Book>,
    location: String,
}

impl Library {

    ///this is a constructor of the library struct
    /// parameters:
    /// - none
    /// returns:
    /// - Library: a new instance of the library struct
    pub fn new() -> Library {
        Library {
            librarian: String::new(),
            books: Vec::new(),
            location: String::new(),
        }
    }

    ///this function adds a book to the library
    /// parameters:
    /// - book: a book struct that contains the information of the book to add
    /// returns:
    /// - none
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    
    ///this function removes a book from the library
    /// parameters:
    /// - book: a book struct that contains the information of the book to remove
    /// returns:
    /// - none
    pub fn remove_book(&mut self, book: Book) {
        self.books.remove(self.find_index(&book).try_into().unwrap());
    }
    
    ///this function lists all the books in the library
    /// parameters:
    /// - none
    /// returns:
    /// - none
    pub fn list_books(&mut self) {
        for book in &mut self.books {
            book.print_book_info();
        }
    }
    
    ///this function rents a book from the library
    /// parameters:
    /// - title: a string which represents the title of the book to rent
    /// returns:
    /// - none
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
            }
        }
        println!("Book not found");
    }
    
    ///this function returns a book to the library
    /// parameters:
    /// - title: a string which represents the title of the book to return
    /// returns:
    /// - none
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

    ///this function finds index of a book in the library
    /// parameters:
    /// - book: a book struct that contains the information of the book to find
    /// returns:
    /// - i32: the index of the book in the library
    pub fn find_index(&self, book: &Book) -> i32 {
        for (i, b) in self.books.iter().enumerate() {
            if b.title == book.title && b.author == book.author {
                return i as i32;
            }
        }
        return -1;
    }
}
