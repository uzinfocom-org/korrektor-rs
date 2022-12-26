//! Functions to sort Uzbek words.
//!
//! Both cyrillic and latin modes can be used.
use regex::Regex;

const CHAR_ORDER: [&str; 128] = [
    "ê", "a", "b", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "x", "y", "z", "ŏ", "ğ", "š", "č",
    "Ê", "A", "B", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "X", "Y", "Z", "Ŏ", "Ğ", "Š", "Č",
    "а", "б", "д", "г", "д", "е", "ё", "ж", "з", "и", "й", "к", "л", "м", "н", "о", "п", "р", "с", "т", "у", "ф", "х", "ц", "ч", "ш", "ъ", "ь", "э", "я", "ю", "ў", "қ", "ғ", "ҳ",
    "А", "Б", "Д", "Г", "Д", "Е", "Ё", "Ж", "З", "И", "Й", "К", "Л", "М", "Н", "О", "П", "Р", "С", "Т", "У", "Ф", "Х", "Ц", "Ч", "Ш", "Ъ", "Ь", "Э", "Ю", "Я", "Ў", "Қ", "Ғ", "Ҳ"
];

const TO_SORT: [&str; 11] = [
    ("G[ʻʼ'‘’‛′ʽ`] Ğ"),
    ("g[ʻʼ'‘’‛′ʽ`] ğ"),
    ("O[ʻʼ'‘’‛′ʽ`] Ŏ"),
    ("o[ʻʼ'‘’‛′ʽ`] ŏ"),
    ("ʻ|ʼ|'|‘|’|‛|′|ʽ|` ʼ"),
    ("Sh Š"),
    ("SH Ö"),
    ("sh š"),
    ("Ch Č"),
    ("CH Ü"),
    ("ch č")
];

const FROM_SORT: [&str; 10] = [
    ("Ğ G‘"),
    ("ğ g‘"),
    ("Ŏ O‘"),
    ("ŏ o‘"),
    ("Š Sh"),
    ("Ö SH"),
    ("š sh"),
    ("Č Ch"),
    ("Ü CH"),
    ("č ch")
];

/// Sorts words in alphabetically ascending order.
///
/// Given String of text returns a new String with words sorted and separated with a newline.
///
/// # Example
/// ```rust
/// use korrektor::uzbek::alphabetic;
///
/// let input = "G‘ozal estafeta chilonzor o'zbek chiroyli".to_string();
/// let output = alphabetic::sort(input);
/// assert_eq!(output, "estafeta\no‘zbek\nchilonzor\nchiroyli\nG‘ozal\n".to_string());
///```
pub fn sort(text: String) -> String {
    let sortable = &to_sortable(text);
    let sorted_intermediate = sort_sortable(sortable);

    from_sortable(sorted_intermediate)
}

fn to_sortable(text: String) -> String {
    let mut input: String = text;

    for pair in TO_SORT.into_iter() {
        let pattern = pair.split_whitespace().next().unwrap();
        let replacer = pair.split_whitespace().last().unwrap();

        let re = Regex::new(pattern).unwrap();
        input = re.replace_all(&input, replacer).as_ref().to_string();
    };

    input
}

fn from_sortable(text: String) -> String {
    let mut input: String = text;

    for pair in FROM_SORT.into_iter() {
        let pattern = pair.split_whitespace().next().unwrap();
        let replacer = pair.split_whitespace().last().unwrap();

        let re = Regex::new(pattern).unwrap();
        input = re.replace_all(&input, replacer).as_ref().to_string();
    };

    input
}

fn usort(string1: &str, string2: &str) -> i8 {
    let length = std::cmp::min(string1.len() - 1, string2.len() - 1);

    for i in 0..length {
        let char1 = match string1.chars().nth(i) {
            Some(char) => char,
            None => panic!("Error in usort: no char at the index {i} in &str: {string1}")
        };
        let char2 = match string2.chars().nth(i) {
            Some(char) => char,
            None => panic!("Error in usort: no char at the index {i} in &str: {string2}")
        };

        let mut value1 = 0;
        let mut value2 = 0;

        if is_exceptioned(char1) {
            value1 = get_exceptioned_value(char1);
        } else {
            value1 = match CHAR_ORDER.iter().position(|&r| r == char1.to_string()) {
                Some(num) => num,
                None => panic!("Error in usort: char {char1} is not found and can not be sorted")
            }
        }

        if is_exceptioned(char2) {
            value2 = get_exceptioned_value(char2);
        } else {
            value2 = match CHAR_ORDER.iter().position(|&r| r == char2.to_string()) {
                Some(num) => num,
                None => panic!("Error in usort: char {char2} is not found can not be sorted")
            }
        }

        match value1.cmp(&value2) {
            std::cmp::Ordering::Less => return -1,
            std::cmp::Ordering::Greater => return 1,
            std::cmp::Ordering::Equal => continue,
        };
    }

    match (string1.len()).cmp(&string2.len()) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Equal => 0,
    }
}

fn sort_sortable(text: &str) -> String {
    let mut sortable: Vec<&str> = text.split_whitespace().collect();
    let mut len = sortable.len();

    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..len - 1 {
            match usort(sortable[i], sortable[i + 1]) {
                1 => {
                    sortable.swap(i, i + 1);
                    sorted = false;
                }
                _ => continue,
            }
        }
        len -= 1;
    }

    let mut result = String::new();
    for word in sortable {
        result.push_str(word);
        result.push('\n');
    }

    result
}

fn is_exceptioned(value: char) -> bool {
    if value == 'Ö' || value == 'Ü' { return true; }

    false
}

fn get_exceptioned_value(value: char) -> usize {
    if value == 'Ö' { return 55; }
    if value == 'Ü' { return 56; }

    0
}

#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn to_sortable_test() {
        let result = to_sortable("G'g' O'o' ShSHsh ChCHch ʻʼ'‘’‛′ʽ`".to_string());
        assert_eq!(result, "Ğğ Ŏŏ ŠÖš ČÜč ʼʼʼʼʼʼʼʼʼ");
    }

    #[test]
    fn from_sortable_test() {
        let result = from_sortable("Ğğ Ŏŏ ŠÖš ČÜč".to_string());
        assert_eq!(result, "G‘g‘ O‘o‘ ShSHsh ChCHch");
    }

    #[test]
    fn is_exceptioned_test() {
        assert!(is_exceptioned('Ö'));
        assert!(is_exceptioned('Ü'));
    }

    #[test]
    fn get_exceptioned_value_test() {
        assert_eq!(get_exceptioned_value('Ö'), 55);
        assert_eq!(get_exceptioned_value('Ü'), 56);
    }

    #[test]
    fn usort_test() {
        assert_eq!(usort("čiroyli", "čilonzor"), 1);
        assert_eq!(usort("čiroyli", "čiroyli"), 0);
        assert_eq!(usort("čilonzor", "čiroyli"), -1);
    }

    #[test]
    fn get_sorted_text_test() {
        let input = String::from("G‘ozal estafeta chilonzor o'zbek chiroyli");
        let output = String::from("estafeta\no‘zbek\nchilonzor\nchiroyli\nG‘ozal\n");
        assert_eq!(sort(input), output)
    }
}