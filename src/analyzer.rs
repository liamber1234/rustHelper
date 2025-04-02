use std::collections::HashMap;

/// this struct is a text analyzer, it holds dictionary of words and their counts
pub struct Analyzer {
    pub words : HashMap<String, usize>,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {
            words: HashMap::new(),
        }
    }

    /// this function takes a string, counts the apearances of each word and stores them in a hashmap
    /// parameters:
    /// - text: a string slice that contains the text to analyze
    /// returns:
    /// - none
    pub fn add_text(&mut self, text: &str) {
        for word in text.split_whitespace() {
             // this line inserts a word to the hashmap, if it is exists it increases the count, and if not adds a new one
            self.words.insert(word.to_string(), self.words.get(word).unwrap_or(&0) + 1);
        }
    }

    /// this function gets a word and returns the amount of times this word has been appeared
    /// parameters:
    /// - word: a string slice that contains the word to count
    /// returns:
    /// - usize: the amount of times the word has been appeared
    pub fn get_word_count(&self, word: &str) -> usize {
        return *self.words.get(word).unwrap_or(&0);
    }
}