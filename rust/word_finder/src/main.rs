use std::collections::HashSet;
use std::fs;

fn main() {
    
    let input = "horsecanarydogworktogether";

    let content = fs::read_to_string("../../words.txt").unwrap();
    let dictionary = HashSet::from_iter(content.lines());
    
    let words = find_words(&input, &dictionary);

    println!("The input: '{input}' contains the following words:");
    for word in words {
        println!("{word}");
    }
    
}

fn find_words<'a>(input: &'a str, dictionary: &HashSet<&str>) -> Vec<&'a str> {
    assert!(input.len() > 0);
    
    let mut found_words = Vec::new();

    let mut start = 0;
    let mut end = input.len();

    while start < end {
        if dictionary.contains(&input[start..end]) {
            found_words.push(&input[start..end]);
            start = end;
            end = input.len();
        } else {
            end -= 1;
        }
    }

    found_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_word() {
        let test_input = "super";
        let mut dictionary = HashSet::with_capacity(1);
        dictionary.insert("super");

        assert_eq!(vec!("super"), find_words(test_input, dictionary));
    }

    #[test]
    fn two_words() {
        let test_input = "horsecanary";
        let mut dictionary = HashSet::with_capacity(2);
        dictionary.insert("horse");
        dictionary.insert("canary");

        assert_eq!(vec!("horse","canary"), find_words(test_input, dictionary));
    }
}

