extern crate colored;

use self::colored::*;

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub fn run() {
    list_months();
    // For more infos, check https://doc.rust-lang.org/book/2018-edition/ch04-02-references-and-borrowing.html
    borrowing_immutable_string();
    borrowing_mutable_string();
    string_slice();
    lifetime();
}

fn list_months() {
    for month in MONTHS.iter() {
        println!("{}", month);
    }
}

fn borrowing_immutable_string() {
    let immutable_string = String::from("Hello");
    // no problem, multiple immutable borrows
    let immutable_string_0 = &immutable_string;
    let immutable_string_1 = &immutable_string;
    let immutable_string_2 = &immutable_string;
    // they can be used after
    let length0 = calculate_length(immutable_string_0);
    let length1 = calculate_length(immutable_string_1);
    let length2 = calculate_length(immutable_string_2);
    println!("{}", "borrowing_immutable_string".bold());
    println!(
        "{:?} {:?}",
        [immutable_string_0, immutable_string_1, immutable_string_2],
        [length0, length1, length2],
    )
}

fn borrowing_mutable_string() {
    println!("{}", "borrowing_mutable_string".bold());
    let mut mutable_string = String::from("Hello");
    let mutable_string_0 = &mut mutable_string;
    // mutable borrow
    change(mutable_string_0);
    // at this point: cannot borrow `mutable_string` as immutable because it is also borrowed as mutable
    println!("mutable_string_0: \"{}\"", mutable_string_0);
}

fn lifetime() {
    println!("{}", "lifetime".bold());
    let string1 = String::from("Hello World");
    let string2 = "Bonjour le monde";
    let result = longest(&string1[..], string2);
    println!("The longest is \"{}\"", result);

    // More infos on https://doc.rust-lang.org/book/2018-edition/ch10-03-lifetime-syntax.html
    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            return s1;
        }
        return s2;
    }
}

// Inspired of https://doc.rust-lang.org/book/2018-edition/ch04-03-slices.html
fn string_slice() {
    println!("{}", "string_slice".bold());
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word0 = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word1 = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word2 = first_word(my_string_literal);

    println!("{} {} {}", word0, word1, word2);

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
}

fn change(s: &mut String) {
    s.push_str(" world!");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::{calculate_length, change};

    #[test]
    fn it_works_calculate_length() {
        let immutable_string = String::from("Hello");
        assert_eq!(calculate_length(&immutable_string), 5);
    }
    #[test]
    fn it_works_change() {
        let mut mutable_string = String::from("Hello");
        change(&mut mutable_string);
        assert_eq!(mutable_string, "Hello world!");
    }
}
