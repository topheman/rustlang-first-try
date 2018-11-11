pub mod libs {
    pub mod greeter;
    pub mod language;
}

mod hello;
mod help;

use std::env;

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
    let args: Vec<String> = env::args().collect();
    let _status_code: i32 = match args.len() {
        1 => {
            help::run();
            0
        }
        2 => match &args[1][..] {
            "help" => {
                help::run();
                0
            }
            "hello" => {
                hello::run();
                0
            }
            cmd => {
                println!("Command not found ({})", cmd);
                126
            }
        },
        _ => {
            help::run();
            0
        }
    };
    return _status_code;
}
