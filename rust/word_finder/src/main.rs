use std::collections::HashSet;
use std::fs;

fn main() {
    
    let input = "horsecanarydogworktogether";
    
    let dictionary = load_words("../../words.txt");

    let words = find_words(&input, &dictionary);

    println!("The input: '{input}' contains the following words:");
    for word in words {
        println!("{word}");
    }
    
}

fn load_words(path: &str) -> HashSet<String>{

    let mut dictionary = HashSet::new();
    let content = fs::read_to_string(path).unwrap();
    for line in content.lines() {
        dictionary.insert(line.to_string().to_lowercase());
    }

    dictionary
}

fn find_words<'a>(input: &'a str, dictionary: &HashSet<String>) -> Vec<&'a str> {
    assert!(input.len() > 0);
    
    let mut found_words = Vec::new();

    let mut start = 0;
    let mut end = input.len();

    while start < end {
        let test = &input[start..end];
        if dictionary.contains(&test.to_lowercase()) {
            found_words.push(test);
            start = end;
            end = input.len();
        } else if let Ok(_) = test.parse::<f64>() {
            found_words.push(test);
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
        dictionary.insert(String::from("super"));

        assert_eq!(vec!("super"), find_words(test_input, &dictionary));
    }

    #[test]
    fn two_words() {
        let test_input = "horsecanary";
        let mut dictionary = HashSet::with_capacity(2);
        dictionary.insert(String::from("horse"));
        dictionary.insert(String::from("canary"));

        assert_eq!(vec!("horse","canary"), find_words(test_input, &dictionary));
    }

    #[test]
    fn mixed_case_still_works() {
        let test_input = "hOrSeCaNaRy";
        let mut dictionary = HashSet::with_capacity(2);
        dictionary.insert(String::from("horse"));
        dictionary.insert(String::from("canary"));

        assert_eq!(vec!("hOrSe","CaNaRy"), find_words(test_input, &dictionary));
    }

    #[test]
    fn finds_numerics() {
        let test_input = "one1two2";
        let mut dictionary = HashSet::with_capacity(2);
        dictionary.insert(String::from("one"));
        dictionary.insert(String::from("two"));

        assert_eq!(vec!("one", "1", "two", "2"), find_words(test_input, &dictionary));
    }
}

