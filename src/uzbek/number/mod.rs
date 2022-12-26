//! Functions to return Uzbek word equivalent of numbers.
//!
//! Only latin mode supported currently.
mod constants;

/// Returns a word representation of a given number.
///
/// Given an integer returns a String with corresponding word equivalent.
///
/// # Example
/// ```rust
///use korrektor::uzbek::number;
///
/// let output = number::integer_to_word(1024);
/// assert_eq!(output, "bir ming yigirma to‘rt".to_string());
/// ```
pub fn integer_to_word(number: i64) -> String {
    //find number to word in constants 0 to 19
    if number == 0 {
        return String::from("nol");
    } else if number < 20 {
        let index = (number - 1) as usize;
        return constants::NUM_1_TO_19[index].1.to_string();
    }
    // find number to word from 0 to 100
    else if number < 100 {
        let index: usize = (number / 10 - 2) as usize;
        return constants::TEEN[index].1.to_string() + " " + &integer_to_word(number % 10);
    }
    // find number to word from 0 to 1000
    else if number < i64::pow(10, 3) {
        return one(number, 2);
    }

    let mut i = 4;
    while i < 27 {
        if number < i64::pow(10, i) { break; }
        i += 1;
    }

    if i % 3 != 0 {
        hundred(number, i - (i % 3))
    } else {
        one(number, i - 3)
    }
}

fn base(number: i64, power: u32) -> String {
    let base = integer_to_word(number / i64::pow(10, power));
    let mult_tuple = constants::MULT.iter().find(|x| x.0 == power as i32);
    let mult = match mult_tuple {
        Some(tuple) => tuple.1,
        None => panic!("Such multiplication value is not found! power is {power}")
    };

    base + " " + mult
}

fn one(number: i64, power: u32) -> String {
    let y = number % i64::pow(10, power);
    let s = integer_to_word(y);
    let separator =
        if power == 2 && !s.is_empty() { " " } else if y < 100 {
            if y == 0 { "" } else { " " }
        } else { " " };

    base(number, power) + separator + &s
}

fn hundred(number: i64, power: u32) -> String {
    let y = number % i64::pow(10, power);
    let sep = if y < 100 {
        if y == 0 { "" } else { " " }
    } else { " " };

    String::new() + &base(number, power) + sep + &integer_to_word(y)
}

#[cfg(test)]
mod as_tests {
    use super::*;

    #[test]
    fn base_test() {
        assert_eq!(base(532, 2), String::from("besh yuz"));
    }

    #[test]
    fn one_test() {
        assert_eq!(one(532, 2), String::from("besh yuz o‘ttiz ikki"));
    }

    #[test]
    fn hundred_test() {
        assert_eq!(hundred(3456, 3), String::from("uch ming to‘rt yuz ellik olti"));
    }

    #[test]
    fn cw_test() {
        assert_eq!(integer_to_word(0), String::from("nol"));
        assert_eq!(integer_to_word(9), String::from("to‘qqiz"));
        assert_eq!(integer_to_word(32), String::from("o‘ttiz ikki"));
        assert_eq!(integer_to_word(104), String::from("bir yuz to‘rt"));
        assert_eq!(integer_to_word(1024), String::from("bir ming yigirma to‘rt"));
        assert_eq!(integer_to_word(3456), String::from("uch ming to‘rt yuz ellik olti"));
    }
}
