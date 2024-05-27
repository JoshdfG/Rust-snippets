use std::io::{self};

#[derive(Debug)]
enum Status {
    Married(u32),
    Single(u32),
}

struct Person {
    first_name: &'static str,
    last_name: &'static str,
    status: Status,
}

const NAMES: [&str; 10] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Helen", "Ivan", "Julia",
];
fn main() {
    let mut people = Vec::new();

    for i in 0..NAMES.len() {
        let first_name = NAMES[i];
        let last_name = NAMES[(i + 1) % NAMES.len()];

        println!("Enter age for {} {}", first_name, last_name);

        let mut age_input = String::new();

        io::stdin()
            .read_line(&mut age_input)
            .expect("failedd to read line");

        let age = age_input
            .trim()
            .parse()
            .expect("Please input the right value");

        let status = if age < 20 {
            Status::Married(age)
        } else {
            Status::Single(age)
        };

        people.push(Person {
            first_name,
            last_name,
            status,
        })
    }

    for person in &people {
        if let Status::Married(age) = person.status {
            println!(
                "{} {} is Married and is {} years old",
                person.first_name, person.last_name, age
            )
        } else if let Status::Single(age) = person.status {
            println!(
                "{} {} is Single and is {} years old",
                person.first_name, person.last_name, age
            )
        }
    }
}
