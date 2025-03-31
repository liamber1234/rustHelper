pub struct Book {
    pub title: String,
    pub author: String,
    pub is_avalible: bool,

}

impl Book {

    pub fn new(title: String, author: String, is_avalible: bool) -> Book {
        Book {
            title,
            author,
            is_avalible,
        }
    }
    pub fn print_book_info(&mut self) {
        println!("Title: {}, Author: {}, Avalablity: {}", self.title, self.author, self.is_avalible);
    }
}

