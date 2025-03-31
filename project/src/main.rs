mod library;
mod book;
use crate::book::Book;
use crate::library::Library;

const OPERATIONS: [&str; 6] = ["Add Book", "Remove Book", "List Books", "Rent Book", "Return Book", "Exit"];

const ADD_BOOK: i32 = 1;
const REMOVE_BOOK: i32 = 2;
const LIST_BOOKS: i32 = 3;
const RENT_BOOK: i32 = 4;
const RETURN_BOOK: i32 = 5;
const EXIT: i32 = 6;

fn main() {
    let mut input = String::new();
    let mut library = Library::new();

    loop {
        print_menu();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
                
        let user_choice: i32 = input.trim().parse().expect("Failed to parse string to integer");

        if user_choice < ADD_BOOK || user_choice > EXIT {
            println!("Invalid choice, please try again.");
            continue;
        }

        if user_choice == ADD_BOOK as i32 {
            println!("Adding a book...");
            let book = input_new_book();
            library.add_book(book);

        } else if user_choice == REMOVE_BOOK as i32 {
            println!("Removing a book...");
            let book = input_new_book();
            library.remove_book(book);

        } else if user_choice == LIST_BOOKS as i32 {
            println!("Listing all books...");
            library.list_books();

        } else if user_choice == RENT_BOOK as i32 {
            println!("Renting a book...");
            println!("Enter the book title:");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let title = input.trim().to_string();

            let book = library.rent_book(&title);
        
        } else if user_choice == RETURN_BOOK as i32 {
            println!("Returning a book...");
            println!("Enter the book title:");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let title = input.trim().to_string();

            let book = library.return_book(&title);

        } else {
            println!("Exiting the program.");
            break;
        }
    }
}

fn print_menu() {
    println!("Enter your choice, this is the menu:");
    for (i, operation) in OPERATIONS.iter().enumerate() {
        println!("{}: {}", i + 1, operation);
    }
}

fn input_new_book() -> Book {
    let mut input = String::new();
    println!("Enter the book title:");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let title = input.trim().to_string();

    println!("Enter the book author:");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
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