fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut max_word = String::new();
    let mut max_count = 0;
    
   for(i , &word) in words.iter().enumerate()
   {
     let mut count = 0;
     for &w in &words[i..]
     {
        if w == word 
        {
            count += 1;
        }
     }
     if count > max_count 
     {
        max_count = count;
        max_word = word.to_string();
     }
   }
    
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}