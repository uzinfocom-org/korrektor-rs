//! Functions to remove duplicated words from text.
//!
//! Both latin and cyrillic modes are supported.
use pcre::Pcre;
use itertools::Itertools;

/// Removes word duplicates from a text.
///
/// Given a String returns a new String without word duplications.
///
/// # Example
/// ```rust
/// use korrektor::utils::duplicates;
///
/// let output = duplicates::remove("salom salom xato salom");
/// let expected = "salom xato".to_string();
/// assert_eq!(output, expected);
/// ```
pub fn remove(text: &str) -> String {
    let mut input = text.to_string();

    let mut re = Pcre::compile(r"[\p{Cyrillic}|\p{Latin}|0-9|\-_]+").unwrap();
    let matches = re.matches(&input);

    if matches.count() > 0 {
        let mut text: Vec<&str> = input.split_whitespace().collect();
        text = text.into_iter().unique().collect();
        input = text.join(" ");
    }

    input
}

#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn remove_duplicates_test() {
        assert_eq!(remove("salom salom xato salom"), String::from("salom xato"));
    }
}