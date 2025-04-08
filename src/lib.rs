pub mod Analyzer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// checks if a new analyzer has been created
    fn test_check_initalization() {
        let analyzer = Analyzer::Analyzer::new();
        assert!(analyzer.words.is_empty());
    }

    #[test]
    /// checks if a single word has been added to the analyzer
    fn test_single_word() {
        let mut analyzer = Analyzer::Analyzer::new();
        analyzer.add_text("dudu");
        assert_eq!(analyzer.get_word_count("dudu"), 1);
    }

    #[test]
    /// checks if multiple words have been added to the analyzer
    fn test_multipe_words() {
        let mut analyzer = Analyzer::Analyzer::new();
        analyzer.add_text("dudu faruk dudu faruk faruk faruk");
        assert_eq!(analyzer.get_word_count("dudu"), 2);
        assert_eq!(analyzer.get_word_count("faruk"), 4);
    }

    #[test]
    /// checks nothing has been added to the analyzer
    fn test_empty_text() {
        let mut analyzer = Analyzer::Analyzer::new();
        analyzer.add_text("");
        assert!(analyzer.words.is_empty());
    }
}

