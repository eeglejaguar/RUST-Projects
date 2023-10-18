#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::collections::HashMap;
    use std::io::{BufRead, BufReader};


    #[test]
    fn test_count_words() {

        let file_txt = File::open("../test").expect("Failed to open file");
        let reader = BufReader::new(file_txt);
        let mut word_and_count = HashMap::new();
        let check_test=HashMap::from(("apple",4));

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            let words = line.split_whitespace();
            for word in words {
                if !word.is_empty() {
                    let correct_word = word.trim_matches(|c: char| !c.is_alphabetic());
                    *word_and_count.entry(correct_word.to_lowercase()).or_insert(0) += 1;
                } else { panic!("Word not Found!!!") }

                assert_eq!(word_and_count,check_test);
            }
        }
    }
}
