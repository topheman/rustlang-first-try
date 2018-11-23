extern crate colored;

use self::colored::*;

#[derive(Debug)]
struct Project {
    repo: String,
    language: String,
}

pub fn run() -> i32 {
    println!("{}", "fp".bold());
    closures();
    iterators();
    return 0;
}

fn closures() {
    let x = 42;
    // Sometimes type annotations can be needed
    let equals_x = |y: i32| -> bool { x == y };
    let not_equals_x = |y| x != y;
    assert!(equals_x(42));
    assert!(not_equals_x(41));
}

fn iterators() {
    let mut numbers_1_to_20 = Vec::<i32>::new();
    // `n` will take the values: 1, 2, ..., 20 in each iteration
    for n in 1..21 {
        numbers_1_to_20.push(n);
    }
    let numbers_1_to_20_even: Vec<&i32> = numbers_1_to_20.iter().filter(|x| **x % 2 == 0).collect();
    let numbers_1_to_20_even_sum: i32 = numbers_1_to_20.iter().sum();
    println!("1 to 20 even numbers: {:?}", numbers_1_to_20_even);
    println!("1 to 20 even sum: {:?}", numbers_1_to_20_even_sum);

    let projects = vec![
        Project {
            repo: String::from("facebook/react"),
            language: String::from("JavaScript"),
        },
        Project {
            repo: String::from("Microsoft/TypeScript"),
            language: String::from("TypeScript"),
        },
        Project {
            repo: String::from("symfony/symfony"),
            language: String::from("php"),
        },
        Project {
            repo: String::from("nodejs/node"),
            language: String::from("JavaScript"),
        },
        Project {
            repo: String::from("docker/engine"),
            language: String::from("Go"),
        },
        Project {
            repo: String::from("Microsoft/TypeScript"),
            language: String::from("TypeScript"),
        },
    ];
    println!(
        "- All projects : {:?}",
        projects
            .iter()
            .map(|project| &project.repo)
            .collect::<Vec<_>>() // turbofish syntax (no temp variable)
    );
    // type anotation on variable (Vec<String> - String part is infered)
    let javascript_projects: Vec<_> = projects
        .iter()
        .filter(|project| project.language == "JavaScript")
        .map(|project| &project.repo)
        .collect();
    println!("- JavaScript projects : {:?}", javascript_projects);
}
