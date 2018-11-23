//! # Config implementation
//!
//! Better handle command line arguments

#[derive(Debug)]
pub struct Config {
    runner: String,
    pub task: String,
    pub task_args: Vec<String>,
}

impl Config {
    //! Code example:
    //! ```rust
    //! let config = libs::cli::Config::new(std::env::args());
    //! match &config.task[..] {
    //!     "foo" => println!("{}", "foo"),
    //!     "bar" => println!("{}", "bar"),
    //!     _ => println!("{}", "any"),
    //! }
    //! ```
    pub fn new(mut args: std::env::Args) -> Config {
        let runner = args.next().unwrap();
        let task = match args.next() {
            Some(arg) => arg,
            None => String::from("help"),
        };
        let task_args: Vec<String> = args.collect();
        Config {
            runner,
            task,
            task_args,
        }
    }
    pub fn wants_help(&self) -> bool {
        match self.task_args.get(0) {
            Some(subtask) => if *subtask == String::from("help") {
                true
            } else {
                false
            },
            _ => false,
        }
    }
}

// ⚠️ How do you mock std::env::Args for testing ?

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works_config() {
//         // declare fake args
//         let args_no_tasks = vec![String::from("target/debug/rustlang-first-try")];
//         let args_one_task = vec![
//             String::from("target/debug/rustlang-first-try"),
//             String::from("string"),
//         ];
//         let args_one_task_plus_arguments = vec![
//             String::from("target/debug/rustlang-first-try"),
//             String::from("string"),
//             String::from("foo"),
//             String::from("bar"),
//         ];
//         // create config passing args as iter() (like std::env::Args)
//         // This wont work because found std::slice::Iter, was looking for std::env::Args
//         let config_no_tasks = Config::new(args_no_tasks.iter());
//         println!("{:?}", config_no_tasks);
//     }
// }
