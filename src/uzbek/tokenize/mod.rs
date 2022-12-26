//! Functions to split Uzbek words by syllables.
//!
//! Implemented according to grammar rules
//! for both latin and cyrillic modes.
use pcre::Pcre;
use regex::Regex;

const LATIN_EXP: [(&str, &str); 7] = [
    ("singil", "si-ngil"),
    ("dengiz", "de-ngiz"),
    ("pešayvon", "pe-shayvon"),
    ("pešona", "pe-shona"),
    ("maishat", "mai-shat"),
    ("išingizni", "ishi-ngiz-ni"),
    ("išingizda", "ishi-ngiz-da"),
];

const CYRILLIC_EXP: [(&str, &str); 0] = [];

const A_CORRECT: [(&str, &str); 5] = [
    ("g[ʻʼ'‘’‛′ʽ`]", "ğ"),
    ("o[ʻʼ'‘’‛′ʽ`]", "ŏ"),
    ("ʻ|ʼ|'|‘|’|‛|′|ʽ|`", "ʼ"),
    ("sh", "š"),
    ("ch", "č")
];

const I_CORRECT: [(&str, &str); 4] = [
    ("ğ", "gʻ"),
    ("ŏ", "o‘"),
    ("š", "sh"),
    ("č", "ch")
];

const REPLACE_CYR: [(&str, &str); 2] = [
    ("[аоуэияёюеў]", "V"),
    ("[бвгджзйклмнпрстфхцчшқғҳ]", "C")
];
const REPLACE_LAT: [(&str, &str); 2] = [
    ("[aoueiŏ]", "V"),
    ("[bdfghjklmnpqrstvxyzğšč]", "C")
];

///
/// Splits the word by syllables.
///
/// Given a String reference returns a new String
/// containing the word separated by syllables with a delimiter.
///
/// # Example
/// ```rust
/// use korrektor::uzbek::tokenize;
///
/// let output = tokenize::split_word(&"chiroyli".to_string());
/// let expected = "chi-roy-li".to_string();
/// assert_eq!(output, expected);
/// ```
pub fn split_word(word: &str) -> String {
    let mut result = a_correct(&word.to_string());
    result = result.trim().to_string();
    let mut last = result.clone();

    let copy = result.clone();

    // latin implementation
    let mut re = Pcre::compile(r"(?:\p{Latin}+)").unwrap();
    let matches = re.matches(&copy);

    // if found latin word
    for _ in matches {
        let key = LATIN_EXP.iter().find(|k| k.0 == copy);
        if let Some(pair) = key { return pair.1.to_string(); }

        for pair in REPLACE_LAT {
            let re = Regex::new(pair.0).unwrap();
            result = re.replace_all(&result, pair.1).as_ref().to_string();
        }
    }

    // cyrillic implementation
    let mut re = Pcre::compile(r"(?:\p{Cyrillic}+)").unwrap();
    let matches = re.matches(&copy);

    // if found cyrillic word
    for _ in matches {
        let key = CYRILLIC_EXP.iter().find(|k| k.0 == copy);
        if let Some(pair) = key { return pair.1.to_string(); }

        for pair in REPLACE_CYR {
            let re = Regex::new(pair.0).unwrap();
            result = re.replace_all(&result, pair.1).as_ref().to_string();
        }
    }

    let textmap = create_map(&result);

    let mut result = String::new();

    for i in 0..textmap.len() {
        let first: Vec<char> = last.chars().collect();
        let first: String = first[0..textmap[i] as usize].iter().collect();
        let second: Vec<char> = last.chars().collect();
        let second: String = second[textmap[i] as usize..].iter().collect();
        last = second;

        if i == 0 {
            result = result + &first;
        } else {
            result = result + "-" + &first;
        }
    }

    i_correct(&result)
}

fn a_correct(text: &String) -> String {
    let mut input = text.clone();
    input = input.to_lowercase();

    for (pattern, replacement) in A_CORRECT {
        let re = regex::Regex::new(pattern).unwrap();
        input = re.replace_all(&input, replacement).as_ref().to_string();
    }

    input
}

fn i_correct(text: &String) -> String {
    let mut input = text.clone();

    for (pattern, replacement) in I_CORRECT {
        let re = regex::Regex::new(pattern).unwrap();
        input = re.replace_all(&input, replacement).as_ref().to_string();
    }

    input
}

fn create_map(word: &String) -> Vec<i32> {
    let mut text_map: Vec<i32> = Vec::new();

    if word.is_empty() {
        return text_map;
    }

    let mut vector: Vec<char> = word.chars().collect();

    for _ in 0..vector.len() {
        match get_split(&vector)
        {
            Split::One => {
                text_map.push(1);
                vector = vector.split_off(1);
            }
            Split::Two => {
                text_map.push(2);
                vector = vector.split_off(2);
            }
            Split::Three => {
                text_map.push(3);
                vector = vector.split_off(3);
            }
            Split::Four => {
                text_map.push(4);
                vector = vector.split_off(4);
            }
            Split::Five => {
                text_map.push(5);
                vector = vector.split_off(5);
            }
            Split::No => continue
        }
    }

    text_map
}

enum Split {
    One,
    Two,
    Three,
    Four,
    Five,
    No,
}

fn get_split(vector: &Vec<char>) -> Split {
    let mut result = Split::No;
    if (letter(vector, 0) == 'V' && letter(vector, 1) != 'C') ||
        (letter(vector, 0) == 'V' && letter(vector, 1) == 'C' && letter(vector, 2) == 'V') {
        result = Split::One;
    }
    if letter(vector, 0) == 'V' && letter(vector, 1) == 'C' && letter(vector, 2) != 'V' && letter(vector, 3) != 'C' {
        result = Split::Two;
    }
    if letter(vector, 0) == 'V' && letter(vector, 1) == 'C' && letter(vector, 2) == 'C' && letter(vector, 3) != 'V' {
        result = Split::Three;
    }
    if (letter(vector, 0) == 'C' && letter(vector, 1) == 'V' && letter(vector, 2) != 'C') ||
        (letter(vector, 0) == 'C' && letter(vector, 1) == 'V' && letter(vector, 2) == 'C' && letter(vector, 3) == 'V') {
        result = Split::Two;
    }
    if letter(vector, 0) == 'C' && letter(vector, 1) == 'V' && letter(vector, 2) == 'C' && letter(vector, 3) == 'C' && letter(vector, 4) == 'V'
        || letter(vector, 0) == 'C' && letter(vector, 1) == 'V' && letter(vector, 2) == 'C' && letter(vector, 3) != 'C' && letter(vector, 3) != 'V' {
        result = Split::Three;
    }
    if letter(vector, 0) == 'C' && letter(vector, 1) == 'V' && letter(vector, 2) == 'C' && letter(vector, 3) == 'C' && letter(vector, 4) != 'V' {
        result = Split::Four;
    }
    if (letter(vector, 0) == 'C' && letter(vector, 1) == 'C' && letter(vector, 2) == 'V' && letter(vector, 3) != 'C') ||
        (letter(vector, 0) == 'C' && letter(vector, 1) == 'C' && letter(vector, 2) == 'V' && letter(vector, 3) == 'C') && letter(vector, 4) == 'V' {
        result = Split::Three;
    }
    if letter(vector, 0) == 'C' && letter(vector, 1) == 'C' && letter(vector, 2) == 'V' && letter(vector, 3) == 'C' && letter(vector, 4) != 'V' && letter(vector, 5) != 'C' {
        result = Split::Four;
    }
    if letter(vector, 0) == 'C' && letter(vector, 1) == 'C' && letter(vector, 2) == 'V' && letter(vector, 3) == 'C' && letter(vector, 4) == 'C' && letter(vector, 5) != 'V' {
        result = Split::Five;
    }

    result
}

fn letter(word: &Vec<char>, i: usize) -> char {
    if i < word.len() {
        word[i]
    } else {
        'n'
    }
}

#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn a_correct_test() {
        assert_eq!(a_correct(&String::from("G'g' O'o' ShSHsh ChCHch ʻʼ'‘’‛′ʽ`")), String::from("ğğ ŏŏ ššš ččč ʼʼʼʼʼʼʼʼʼ"));
    }

    #[test]
    fn i_correct_test() {
        assert_eq!(i_correct(&String::from("ğ ŏ š č")), String::from("gʻ o‘ sh ch"));
    }

    #[test]
    fn create_map_test() {
        assert_eq!(create_map(&"CVCCVCCVCVC".to_string()), vec![3, 3, 2, 3]);
    }

    #[test]
    fn do_test() {
        assert_eq!(split_word("singil"), "si-ngil");
        assert_eq!(split_word("chiroyli"), "chi-roy-li");
        assert_eq!(split_word("чиройли"), "чи-рой-ли");
    }
}