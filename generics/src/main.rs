use core::panic;
use std::{fs::File, io::ErrorKind};

fn main() {
    let num = vec![100, 20, 233, 23, 42];
    let large = get_largest(num);
    println!("the largest number is {:?}", large);
    let cha = vec!['t', 'y', 'z', 'w'];
    let la = get_largest(cha);
    println!("the largest character is {:?}", la);

    // closures
    let fs = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Failed to create file {:?}", error);
            })
        } else {
            panic!("unknown error occured {:?}", error);
        }
    });
}

fn get_largest<T: PartialOrd + Copy>(num_lg: Vec<T>) -> T {
    let mut large_num = num_lg[0];

    for num in num_lg {
        if num > large_num {
            large_num = num;
        }
    }
    large_num
}

// generics can also be used in structs by defining the type to be a type <T>
// now you can use any type but you cant mix two types in a struct
// but to make this work as desired you can use the struct Student <T,U> now when this is set
//the struct is expecting different types and then can now accept two types in the provided field
