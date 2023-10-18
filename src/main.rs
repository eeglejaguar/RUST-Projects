// Basic File I/O:
// ○ Develop a Rust program that reads a text file and counts the occurrences of
// each word in the file.
// ○ Utilize vectors and hash maps for data storage.
// ○ Demonstrate error handling using Result for file operations.
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main() {
    // Opening the file test.txt
    let file_txt = File::open("test").expect("Failed to open file");
    // Reading the file
    let reader = BufReader::new(file_txt);
    // Using a Hashmap to store the value with its label which will be the word
    let mut word_and_count = HashMap::new();

    // This for loop is reading every line from file_txt
    for line in reader.lines() {

        // So instead of writing another line of code for error handling i used expect here to make sure if the line is not readable it will give a n error
        let line = line.expect("Failed to read line");

        // Here i used split_whitespaces function to make the words separate by whitespaces or gaps and store each of them in word variable
        let words = line.split_whitespace();

        // This loops checks if there is a matching word in the file and then add a count in the Hashmap
        for word in words {
            // So im using here a method is_empty() as my value is a &str slice so i can't match it with using operators using null brackets so i used this method

            // This case will only run if there will be a word and it will be saved in the Hashmap with adding a 1 to its count
            if !word.is_empty() {
                // Here its checking if the word is written correctly in the file if it has any nums attached to it this will simply trim it and only take the alphabets
                let correct_word=word.trim_matches(|c:char|!c.is_alphabetic());
                // Here im just entering the value of the labels(words) and set its default values as 0 using or_insert and adding a count to the value
                *word_and_count.entry(correct_word.to_lowercase()).or_insert(0) += 1;
            }
            // If there is no word the program will panic and print the msg Word not Found
            else { panic!("Word not Found!!!") }
        }
    }

    // Printing the values of the Hashmap
    println!("{:?}", word_and_count);
}
