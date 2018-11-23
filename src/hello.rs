extern crate colored;

use self::colored::*;

use super::libs::greeter;
use super::libs::language::Language;

pub fn run() -> i32 {
    let first_name = "Tony";
    let last_name = "Stark";
    println!("{}", "Basic".bold());
    say_hello_basic(first_name, last_name, Language::English);
    say_hello_basic(first_name, last_name, Language::French);
    println!("{}", "Builder pattern".bold());
    say_hello_builder_pattern(first_name, last_name, Language::English);
    say_hello_builder_pattern(first_name, last_name, Language::French);
    println!("{}", "Default trait".bold());
    say_hello_default(first_name, last_name, Language::English);
    say_hello_default(first_name, last_name, Language::French);
    println!("{}", "Other".bold());
    println!("3 + 5 = {}", sum(3, 5));
    return 0;
}

pub fn greet_msg(f_name: &str, l_name: &str, language: Language) -> String {
    let greeting: &str = match language {
        Language::English => "Hello",
        Language::French => "Bonjour",
    };
    format!("{} {} {}", greeting, f_name, l_name)
}

pub fn say_hello_basic(f_name: &str, l_name: &str, language: Language) {
    println!("{}", greet_msg(f_name, l_name, language));
}

// default params (builder pattern + default trait)

pub fn say_hello_builder_pattern(f_name: &str, l_name: &str, language: Language) {
    let msg = greeter::builder::GreeterBuilder::new()
        .name(f_name.to_owned() + " " + l_name)
        .with_language(language)
        .finish();
    println!("{}", msg);
}

pub fn say_hello_default(f_name: &str, l_name: &str, language: Language) {
    let msg = greeter::default::Greeter {
        name: f_name.to_owned() + " " + l_name,
        language: language,
    };
    println!("{}", msg);
}

// no semi-column = implicit return
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn it_works() {
        say_hello_basic("Peter", "Parker", Language::English);
    }
}
