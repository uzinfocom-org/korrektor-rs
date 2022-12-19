use pcre::Pcre;
use itertools::Itertools;

pub fn remove_duplicates(text: String) -> String {
    let mut input = text;

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
        assert_eq!(remove_duplicates(String::from("salom salom xato salom")), String::from("salom xato"));
    }
}