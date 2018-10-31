fn main() {
    let first_name = "Tony";
    let last_name = "Stark";
    say_hello(first_name, last_name, Language::English);
    say_hello(first_name, last_name, Language::French);
    println!("3 + 5 = {}", sum(3,5));
}

enum Language {
    English,
    French
}

fn greet_msg(f_name: &str, l_name: &str, language: Language) -> String {
    let greeting: &str = match language {
        Language::English => "Hello",
        Language::French => "Bonjour",
    };
    format!("{} {} {}", greeting, f_name, l_name)
}

fn say_hello(f_name: &str, l_name: &str, language: Language) {
    println!("{}", greet_msg(f_name, l_name, language));
}

// no semi-column = implicit return
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn it_works() {
        say_hello("Peter", "Parker", Language::English);
    }
}