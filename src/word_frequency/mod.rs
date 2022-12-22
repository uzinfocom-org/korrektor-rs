use std::collections::HashMap;
use pcre::Pcre;
use itertools::Itertools;

/// Provides a list of words and their number of occurences in a given text.
///
/// Given a String returns a HashMap with words as keys
/// and number of their occurrences in the String as values.
///
/// # Example
///```rust
/// use std::collections::HashMap;
/// use korrektor::word_frequency::word_frequency;
///
/// let output = word_frequency("salom xato quyosh salom mushuk".to_string());
/// let correct_output = HashMap::from([(String::from("salom"), 2), (String::from("xato"), 1), (String::from("quyosh"), 1), (String::from("mushuk"), 1)]);
/// assert_eq!(output, correct_output);
/// ```
pub fn word_frequency(text: String) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();
    let mut pre_result: HashMap<&str, usize> = HashMap::new();

    let mut re = Pcre::compile(r"[\p{Cyrillic}|\p{Latin}|0-9|\-_]+").unwrap();
    let matches = re.matches(&text);

    if matches.count() > 0 {
        let input_vec = text.split_whitespace();
        pre_result = input_vec.into_iter().counts();
    }

    for (word, count) in pre_result {
        result.insert(String::from(word), count);
    }

    result
}

#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn word_frequency_test() {
        assert_eq!(word_frequency(String::from("salom xato quyosh salom mushuk")),
                   HashMap::from([(String::from("salom"), 2), (String::from("xato"), 1), (String::from("quyosh"), 1), (String::from("mushuk"), 1)]));
    }
}