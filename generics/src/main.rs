fn main() {
    let num = vec![100, 20, 233, 23, 42];
    let large = get_largest(num);
    println!("the largest number is {:?}", large);
    let cha = vec!['t', 'y', 'z', 'w'];
    let la = get_largest(cha);
    println!("the largest character is {:?}", la);
}

fn get_largest<T: PartialOrd + Copy>(num_len: Vec<T>) -> T {
    let mut largest = num_len[0];

    for num in num_len {
        if num > largest {
            largest = num;
        }
    }
    largest
}
