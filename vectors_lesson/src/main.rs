use ::std::collections::HashMap;
use core::panic;
use std::{fs::File, io::ErrorKind};
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    // using enums in vectors

    #[derive(Debug)]
    enum Students {
        Total(u32),
        Class(String),
        Age(u16),
    }

    let std_det = vec![
        Students::Total(20),
        Students::Class(String::from("SS2")),
        Students::Age(18),
    ];
    println!("{:?}", std_det);

    println!("----------------------------------------");

    match &std_det[2] {
        Students::Age(i) => println!("{}", i),
        _ => println!("Not an integer"),
    }

    // hashmaps

    let blue = String::from("Blue house");
    let red = String::from("Red house");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(red, 12);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    println!("---------------------------------------");

    let country1 = String::from("Nigeria");
    let country2 = String::from("United kingdom");

    let mut nations = HashMap::new();

    nations.insert(country1, 100);
    nations.insert(country2, 7000);

    let country_name = String::from("");
    let nation = nations.get(&country_name);
    for (key, value) in &nations {
        println!("{} {}", key, value);
    }

    println!("-----------------------------------------");

    let institution1 = String::from("OAU");
    let institution2 = String::from("Web3Bridge");

    let mut schools = HashMap::new();

    schools.insert(institution1, 1997);
    schools.insert(institution2, 2019);

    let school_influence = String::from("");
    let school = schools.get(&school_influence);

    for (key, value) in &schools {
        println!("{} {}", key, value);
    }

    let coin = String::from("penny");
    let coin1 = String::from("quarter");

    let mut coins = HashMap::new();

    coins.insert(coin, 11);
    coins.insert(coin1, 15);

    let relevance = String::from("");
    let coin = coins.get(&relevance);

    for (key, value) in &coins {
        println!("{} {}", key, value);
    }

    let n = 7;
    let (steps, results) = sequence(n);
    println!("Let the results {} be recorded in {} steps", steps, results);

    let u = "qqqybuwwww";
    let counter = repetition(u);
    println!("{}", counter);

    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    // this will fail because the file we are trying to open does not exist

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // but this will pass because the file doesnt exist but then we created it to make the error
    // fail successfully
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let file_creation = File::create("hi.txt");

    let fc = match file_creation {
        Ok(file) => file,
        Err(er) => match er.kind() {
            ErrorKind::NotFound => match File::create("hi.txt") {
                Ok(f) => f,
                Err(e) => panic!("error {:?}", e),
            },
            other => {
                panic!("err {:?}", other)
            }
        },
    };

    // another method using unwrap_or_else and closures
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    {
        let get_file = File::create("hello.txt");
        let fs_p = match get_file {
            Ok(pass) => pass,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fs) => fs,
                    Err(err) => panic!("error {:?}", err),
                },
                other => {
                    panic!("other error {:?}", other);
                }
            },
        };

        // the same expression using closures
        let f = File::open("durh.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("durh.txt")
                    .unwrap_or_else(|error| panic!("unable to create the file {:?}", error))
            } else {
                panic!("unknown error occured{:?}", error)
            }
        });
    }
}
//sequence n!=1
fn sequence(mut n: u64) -> (u64, u32) {
    let mut count = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2
        } else {
            n = n * 3 + 1;
        }
        count += 1;
    }
    (n, count)
}

fn repetition(dna_sequence: &str) -> usize {
    let mut current_length = 0;
    let mut max_length = 0;

    for (i, c) in dna_sequence.chars().enumerate() {
        if i > 0 && dna_sequence.chars().collect::<Vec<_>>()[i - 1] != c {
            current_length = 0;
        }
        current_length += 1;
        max_length = max_length.max(current_length);
    }
    max_length
}
