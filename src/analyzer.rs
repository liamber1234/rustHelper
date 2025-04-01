use std::collections::HashMap;

struct Analyzer {
    words : HashMap<String, usize>,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {
            words: HashMap::new(),
        }
    }

    ///this function takes a string, counts the apearances of each word and stores them in a hashmap
    pub fn analyze_text(&mut self, text: &str) {
        for word in text.split_whitespace() {
             // this line inserts a word to the hashmap, if it is exists it increases the count, and if not adds a new one
            words.insert(word.to_string(), words.get(word).unwrap_or(&0) + 1);
        }
    }

    ///this function gets a word and returns the amount of times this word has been appeared
    pub fn get_word_count(&self, word: &str) -> usize {
        return *self.words.get(word).unwrap_or(&0);
    }
}