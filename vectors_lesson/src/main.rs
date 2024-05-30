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
}
