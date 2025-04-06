use analyzer_project::Analyzer::Analyzer;

const TEXT_INSERT: &str = "1";
const WORD_COUNT: &str = "2";
const EXIT: &str = "3";

fn main() {
    let mut analyzer = Analyzer::new();
    loop {
        print_menu();
        let mut choice = String::new();
        if std::io::stdin().read_line(&mut choice).is_err() {
            eprintln!("Failed to read input");
            break;
        }
        if !do_analyzer_operaion(choice.as_str(), &mut analyzer) {
            break;
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
    std::io::stdin().read_line(&mut text).map_err(|e| {
        println!("Failed to read input");
        return;
    });

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

    std::io::stdin().read_line(&mut word).map_err(|e| {
        println!("Failed to read input");
    });

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

/// this function looks at the user input and calls to the right function
/// parameters:
/// - choice: the user input
/// - analyzer: a mutable reference to the analyzer
/// returns:
/// - none
fn do_analyzer_operaion(choice: &str, analyzer: &mut Analyzer) -> bool{ 
    match choice.trim() {
        TEXT_INSERT => {
            println!("Inserting text...");
            insert_text(analyzer);
            return true;
        }
        WORD_COUNT => {
            println!("Getting word count...");
            get_word_count(analyzer);
            return true;
        }
        EXIT => {
            println!("Exiting...");
            return false;
        }
        _ => {
            println!("Invalid choice, please try again.");
            return true;
        }
    }
}