mod library;
mod book;
use crate::book::Book;
use crate::library::Library;

const OPERATIONS: [&str; 6] = ["Add Book", "Remove Book", "List Books", "Rent Book", "Return Book", "Exit"];

enum operation{
    ADD_BOOK = 1,
    REMOVE_BOOK,
    LIST_BOOKS,
    RENT_BOOK,
    RETURN_BOOK,
    EXIT
}

fn main() {
    let mut input = String::new();
    let mut library = Library::new();

    loop {
        print_menu();
        input.clear();
        match std::io::stdin().read_line(&mut input) {
            Err(e) => {
                println!("Error reading input: {}", e);
                return; // Exit the program on error
            }
            Ok(_) => {}
        }
                
        let user_choice: operation = convert_to_operation(input.trim().parse().unwrap_or(0)); // Default to 0 if parsing fails


        match user_choice {
            operation::ADD_BOOK => add_book(&mut library),
            operation::REMOVE_BOOK => remove_book(&mut library),
            operation::LIST_BOOKS => list_books(&mut library),
            operation::RENT_BOOK => rent_book(&mut library),
            operation::RETURN_BOOK => return_book(&mut library),
            operation::EXIT => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }

    }
}

/// this function prints the menu of the program
/// parameters:
/// - none
/// returns:
/// - none
fn print_menu() {
    println!("Enter your choice, this is the menu:");
    for (i, operation) in OPERATIONS.iter().enumerate() {
        println!("{}: {}", i + 1, operation);
    }
}

/// this function takes the input from the user and creates a new book
/// parameters:
/// - none
/// returns:
/// - Book: a new instance of the book struct
fn input_new_book() -> Book {
    let mut input = String::new();
    println!("Enter the book title:");
    input.clear();
    std::io::stdin().read_line(&mut input);
    let title = input.trim().to_string();

    println!("Enter the book author:");
    input.clear();
    std::io::stdin().read_line(&mut input);
    let author = input.trim().to_string();

    if title.is_empty() {
        println!("uncorrect, pls try again");
        return input_new_book(); // Recursively prompt for a valid title
    }
    
    if author.is_empty() {
        println!("uncorrect, pls try again");
        return input_new_book(); // Recursively prompt for a valid author
    }

    let book = Book::new(title, author, true);
    return book;
}

/// this function adds a book to the library
/// parameters:
/// - library: the library to add the book to
/// returns:
/// - none
fn add_book(library: &mut Library) {
    println!("Adding a book...");
    let book = input_new_book();
    library.add_book(book);
}

/// this function removes a book from the library
/// parameters:
/// - library: the library to remove the book from
/// returns:
/// - none
fn remove_book(library: &mut Library) {
    println!("Removing a book...");
    let book = input_new_book();
    library.remove_book(book);
}

/// this function lists all the books in the library
/// parameters:
/// - library: the library to list the books from
/// returns:
/// - none
fn list_books(library: &mut Library) {
    println!("Listing all books...");
    library.list_books();
}

/// this function rents a book from the library
/// parameters:
/// - library: the library to rent the book from
/// returns:
/// - none
fn rent_book(library: &mut Library) {
    println!("Renting a book...");
    let book = input_new_book();
    library.rent_book(book.title.as_str());
}

/// this function returns a book to the library
/// parameters:
/// - library: the library to return the book to
/// returns:
/// - none
fn return_book(library: &mut Library) {
    println!("Returning a book...");
    let book = input_new_book();
    library.return_book(book.title.as_str());
}


/// this function converts the input from the user to an operation enum
/// parameters:
/// - input: the input from the user
/// returns:
/// - operation: the operation enum that corresponds to the input
fn convert_to_operation(input: i32) -> operation {
    match input {
        1 => operation::ADD_BOOK,
        2 => operation::REMOVE_BOOK,
        3 => operation::LIST_BOOKS,
        4 => operation::RENT_BOOK,
        5 => operation::RETURN_BOOK,
        _ => operation::EXIT,
    }
}