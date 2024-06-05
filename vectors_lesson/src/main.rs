use ::std::collections::HashMap;
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
    let (results, steps) = sequence(n);
    println!("Let the results {} be recorded in {} steps", steps, results);
}

fn sequence(mut n: u64) -> (u64, u32) {
    let mut count = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        count += 1;
    }
    (n, count)
}
