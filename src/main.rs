//! This project is a little command line interface that exposes a few rust use cases.
//!
//! You will find at the root of the `src` folder the entry points containing a `run()` function to call for each command.
//!
//! # Examples
//!
//! ```shell
//! cargo run
//! cargo run hello
//! ```
//!
//! # Notes
//!
//! This documentation has been generated with `cargo doc --all`.
//!
//! To generate it and read it yourself in local, run `cargo doc --open`.
//!
//! * 📚 More infos on the [repo's README](https://github.com/topheman/rustlang-first-try#readme)
//! * 👇 Check out the Modules section

pub mod libs {
    pub mod cli;
    pub mod greeter;
    pub mod language;
}

mod fp;
mod hello;
mod help;
mod string;

/**
 * Inspired by https://stackoverflow.com/questions/30281235/how-to-cleanly-end-the-program-with-an-exit-code
 *
 * Everything happens in real_main() which returns a status code.
 * That way, we can cleanly exit (if needed)
 */
fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}

/**
 * Inspired by https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html
 */
fn real_main() -> i32 {
    let config = libs::cli::Config::new(std::env::args());
    // must convert to slice before matching
    let status_code = match &config.task[..] {
        "help" => help::run(),
        "hello" => hello::run(),
        "string" => string::run(config.task_args),
        "fp" => fp::run(),
        cmd => {
            println!("Command not found ({})", cmd);
            126 // return failing statusCode - other ::run() task return i32 statusCodes
        }
    };
    return status_code; // return success statusCode
}

// Extract arguments passed to a tasks like Vec["foo", "bar"] in `cargo run string foo bar`
#[allow(dead_code)]
fn extract_task_args(args: &Vec<String>) -> Vec<&str> {
    args.iter()
        .enumerate() // adds indexes i as (index, elem) tuple in map/filter
        .filter(|(i, _)| *i > 1)
        .map(|(_, s)| &**s)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::extract_task_args;

    #[test]
    fn it_works_extract_task_args() {
        let args_no_tasks = vec![String::from("target/debug/rustlang-first-try")];
        let args_one_task = vec![
            String::from("target/debug/rustlang-first-try"),
            String::from("string"),
        ];
        let args_one_task_plus_arguments = vec![
            String::from("target/debug/rustlang-first-try"),
            String::from("string"),
            String::from("foo"),
            String::from("bar"),
        ];
        // Use the "turbofish" `::<>` syntax to explicitly declare type of vector
        // Rust can't infer type from empty vector
        assert_eq!(extract_task_args(&args_no_tasks), Vec::<String>::new());
        assert_eq!(extract_task_args(&args_one_task), Vec::<String>::new());
        assert_eq!(
            extract_task_args(&args_one_task_plus_arguments),
            vec![String::from("foo"), String::from("bar")]
        );
    }
}
