use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn count_words_in_file(filepath: &str) -> HashMap<String, usize> {
    let contents = fs::read_to_string(filepath).unwrap_or_else(|_| String::new());
    let mut word_count = HashMap::new();
    for word in contents.split_whitespace() {
        let word = word
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();
        *word_count.entry(word).or_insert(0) += 1;
    }

    word_count
}

fn multi_threaded_counter_mutex(file_paths: Vec<&str>) -> HashMap<String, usize> {
    let shared_counter = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];
    for file_path in file_paths {
        let shared_map = Arc::clone(&shared_counter);
        let file_path = file_path.to_string();

        let handle = thread::spawn(move || {
            let local_count = count_words_in_file(&file_path);
            let mut global_count = shared_map.lock().unwrap();

            for (word, count) in local_count {
                *global_count.entry(word).or_insert(0) += count;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = shared_counter.lock().unwrap().clone();
    result
}

fn main() {
    let file_paths = vec!["sample/book.txt", "sample/words.txt"];
    let start = Instant::now();
    let word_count = multi_threaded_counter_mutex(file_paths);
    let duration = start.elapsed();

    println!("Word count completed in: {:?}", duration);
    println!("Total unique words: {}", word_count.len());

    // Print the top 10 words
    let mut top_words: Vec<_> = word_count.iter().collect();
    top_words.sort_by(|a, b| b.1.cmp(a.1));

    println!("\nTop 10 words:");
    for (word, count) in top_words.iter().take(10) {
        println!("{}: {}", word, count);
    }
}
