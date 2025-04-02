pub mod analyzer;
use analyzer::Analyzer;

const TEXT_INSERT: &str = "1";
const WORD_COUNT: &str = "2";
const EXIT: &str = "3";

fn main() {
    let mut analyzer = Analyzer::new();
    loop {
        print_menu();
        let mut choice = String::new();
        if  std::io::stdin().read_line(&mut choice) == Err(e) {
            eprintln!("Failed to read input: {}", e);
            break;;
        }
        match choice.trim() {
            TEXT_INSERT => {
                println!("Inserting text...");
                insert_text(&mut analyzer);
            }
            WORD_COUNT => {
                println!("Getting word count...");
                get_word_count(&mut analyzer);
            }
            EXIT => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

/// this function inserts a text to the analyzer
/// parameters:
/// - analyzer: a mutable reference to the analyzer
/// returns:
/// - none
fn insert_text(analyzer: &mut Analyzer) {
    println!("Please enter a text to analyze:");
    let mut text = String::new();
    if std::io::stdin().read_line(&mut text) == Err(e) {
        eprintln!("Failed to read input: {}", e);
        return;
    }
    analyzer.add_text(&text);
}

/// this function gets a word and returns the amount of times this word has been appeared
/// parameters:
/// - analyzer: a mutable reference to the analyzer
/// returns:
/// - none
fn get_word_count(analyzer: &mut Analyzer) {
    println!("Please enter a word to count:");
    let mut word = String::new();
    if std::io::stdin().read_line(&mut word) == Err(e) {
        eprintln!("Failed to read input: {}", e);
        return;
    }
    let word = word.trim();
    let count = analyzer.get_word_count(&word);
    println!("{} times", count);
}

/// this function prints the menu
/// parameters:
/// - none
/// returns:
/// - none
fn print_menu() {
    println!("Please choose an option:");
    println!("1. Insert text");
    println!("2. Get word count");
    println!("3. Exit");
}