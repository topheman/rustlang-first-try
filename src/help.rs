extern crate colored;

use self::colored::*;

pub fn run() {
    println!();
    println!(
        "  {} {}",
        "ruslang-first-try".bold(),
        " - First steps with rust ..."
    );
    println!();
    println!("  {}", "USAGE".bold());
    println!("      {} {}", "$".bold(), "cargo run");
    println!("      {} {}", "$".bold(), "cargo run help");
    println!("      {} {}", "$".bold(), "cargo run hello");
    println!();
    println!("  {}", "COMMANDS".bold());
    println!("      {}", "help                          display help");
    println!(
        "      {}",
        "hello                         simple hello worlds"
    );
    println!("      {}", "string                        testing strings");
    println!();
}
