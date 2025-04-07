use std::fmt;

///struct which represents a book and includes the title, author, and availability status of the book.
pub struct Book {
    pub title: String,
    pub author: String,
    pub is_avalible: bool,

}

impl Book {

    ///this is a constructor of the book struct
    /// parameters:
    /// - title: a string that contains the title of the book
    /// - author: a string that contains the author of the book
    /// - is_avalible: a boolean that indicates if the book is available or not
    /// returns:
    /// - Book: a new instance of the book struct
    pub fn new(title: String, author: String, is_avalible: bool) -> Book {
        Book {
            title,
            author,
            is_avalible,
        }
    }

}

///this function prints the information of the book
/// parameters:
/// - none
/// returns:
/// - none
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {}, Author: {}, Availability: {}", self.title, self.author, self.is_avalible)
    }
}