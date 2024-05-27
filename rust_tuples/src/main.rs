use std::io::{self};
#[derive(Debug)]

// struct User{
//     username: String,
//     email:String,
//     sign_in_count:u64,
//     active:bool,
// }
// #[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

// implentation / methods are similiar to functions except that they are tied to structs
//lets create an implementation our rectangle
// the first argument in a method is self which is the instance the method is being called on
// in this case Rectangle... we can also take immutable refrence or even take ownership of the instance in rare cases
// methods makes our code clear that we are making reference to the rectangle and makes it organised.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// IpAddrKind is now a custom data type that can now be used somewhere else
// always specify the data type of a const and know that it must be in caps or cap snake case.
// note that you namespace the value of an enum under its identifier
// const FOUR: IpAddrKind = IpAddrKind::V4;
enum IpAddrKind {
    V4,
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// to eliminalte the stress of having to use an enum and a struct we can instead put the data into
// an enum variant look below to see

// the name of each enum variant that we define also becomes a function that constructs an instance of the enum

#[derive(Debug)]
enum IpAddressKind {
    v4(String),
    v6(String),
}

#[derive(Debug)]
enum Message {
    // Quit has no data associated with it at all.
    Quit,
    // Move has named fields, like a struct does.
    Move { x: i32, y: i32 },
    // Write includes a single String.
    Write(String),
    // ChangeColor includes three i32 values.
    ChangeColor(i32, i32, i32),
}

enum Option<T> {
    None,
    Some(T),
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow(i32),
}

#[derive(Debug)]
enum Cars {
    Toyota(String),
    Honda(String),
    Bmw(String),
    Benz(String),
}

fn main() {
    // enum implementation
    let four: IpAddressKind = IpAddressKind::v4(String::from("127.3..1"));
    println!("{:?}", four);
    let six: IpAddressKind = IpAddressKind::v6(String::from("127..1"));
    println!("{:?}", six);

    // struct implementation of the same enum
    let ip = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    //using pattern matching to destructure in rust we first have a variable called
    //tup that is binded to the tuple then we use the let keyword
    //to assign tup to three separate variables
    let tup: (i32, f64, u8) = (500, 1.6, 1);
    let (x, _y, _z) = tup;

    let mut a = String::from("Hello");
    let b = &mut a;

    println!("shfh c is {}", b);

    println!("the value of a ");
    println!("the value of x is {}", x);

    //we can also access a tuple value by using the dot notation followed by the index
    //of the tuple we want to access and tuples also start from index zero.
    let tup_val: (i32, f64, u8) = (700, 2.5, 16);
    let seven_hundred = tup_val.0;
    println!("The value of seven_hundred is {}", seven_hundred);

    // ARRAYS
    let arr_fixed: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("array fixed number at index six is {}", arr_fixed[5]);

    //in this case arrays are used to replicate a string literal or numbers insteas of
    //writing arr = [josh,josh,josh,josh,josh]. this means that the array named arr will contain
    //5 elements and the values will be Josh
    let arr: [&str; 5] = ["Josh"; 5];
    //to get the element at different indexes in an array you can do it this way
    let josh_0 = arr[0];
    println!("index zero value is {}", josh_0);

    //this is to test that if you input the wrong array index you'll get an error message
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    //this code block below converts the string into a number and checks if the input is a number
    let index: usize = index
        .trim() //this trims the string
        .parse() //this converts it to a usize The usize type
        //is an unsigned integer type with the same number of bits as the platform's pointer type
        .expect("The index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    println!("The value of the other function is {}", another_function());

    // control flow

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let ar: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please input an index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read input");

    let index: usize = index
        .trim()
        .parse()
        .expect("The index entered was not a number");
    let el = ar[index];

    println!("The index entered is index {index}: {el} ");

    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("{}", s);

    // borrow checker, references and ownership
    // ownership in rust is a set of rules that governs how a program is being run
    // there are 2 main memory locations in rust which are the stack and the heap
    // the stack saves data as they come in and pops them out the opposite way ie last in first out
    // the types of values you can store on the stack are values that the compiler can calculate
    // their size at compile time, then the remaining are stored on the heap
    //when you request to store data on the heap the memory allocator checks the size of what you want
    // to store then looks for a place in memory that is big enough to accomodate it then it marks it as
    // being in use then returns the pointer i.e an address to that memory location. now that pointer can
    // be saved on the stack but when ever you want to make reference to the data stored at that location you
    // need to follow the pointer which is the address to where the data is found
    // you can easily copy or integer booleans that are initiated, floats and tuples that contains integers
    // because their sizes are known at compile time and you can assertain that this is the space they will
    // be using
    // this isnt the same for strings, not string literals cos you have to hardcode the value of a string literal
    // but because strings can grow over time you have to use becareful of how they are being used
    // string type manages data asociated with data on the heap and can be used to store data that are unknown to us at
    // compile time,strings can be mutated but literals cannot be mutated.
    // a string is made up of three parts which are a pointer, capacity and the length

    // the length is the amount in bytes that is being used by the string
    // capacity is the total amount of bytes that is allocated to us

    // we have references in rust and how this works it that,

    // shallow copy or move is the concept of copying the pointer and capacity without copying the data.

    // to deeply copy we can use the clone()

    // stack only data copy
    // this can be done with data types that the their sizes can be known at compile time and this because variables that have the
    // known size are stored on the stack and their sizes are known by the compiler and with this the making a copy is not an issue
    //rust has a copy trait that for types that are stored on the stack. if a type implements a copy trait variables that use it do not move but only
    // trivially copies which makes them valid after assignment to another variable.
    //and rust wont allow us make use of the copy trait if any part of it has implemened the drop trait

    //ownership and references
    let mut s = String::from("Hello");
    let the_length = calculate_length(&s);

    println!("The length of `{}` is {}", s, the_length);

    //mutable reference
    // mutable references has a restriction that only allows you to have one mutable reference
    // to a piece of data in a scope. This is to prevent data race at compile time
    // this is when two pointers are pointing to the same piece of data.

    // you can have multiple immutable references but you cant have 2 immutable reference and one mutable
    // reference you'll get an error because the underlying data is about to be changed.
    // but you can add a mutable reference after the immutable reference has gone out of scope.

    // danling references -> this is a reference that points to invalid data.

    modify(&mut s);

    println!("The new word is {}", s);

    // slices -> this lets you references a specific item in a collection rather than the whole collection
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slices = &arr[..2];
    println!("The length of the slice is {:?}", slices);

    // let user1 = User{
    //     email:String::from("slimmy0310@gmail.com"),
    //     username:String::from("Josh_dfG"),
    //     sign_in_count:1,
    //     active:true
    // };

    // println!("user1 {:?}",user1);

    // the implementation checks if rect1 can hold either of the rect3 and rect4
    // this is to show how efficient implementations can be.

    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec3 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec4 = Rectangle {
        width: 20,
        height: 20,
    };

    println!("the area is {:?}", rec1.area());

    println!("rect can hold rect1 {}", rec1.can_hold(&rec3));
    println!("rect can hold rect1 {}", rec1.can_hold(&rec4));

    let new_square = Rectangle::square(25);

    println!("The size of the {:?}", new_square);

    // fn cal (rect: &Rectangle)->u32{
    //     rect.height * rect.width
    // }

    fn calculate_length(s1: &String) -> usize {
        let len = s1.len();
        len
    }

    fn modify(s2: &mut String) {
        s2.push_str(" World");
    }

    //functions
    fn another_function() -> i32 {
        5
    }

    // another advantage of using enums over structs is that every variant of the enum can have different
    // types and amount of data associated with them.
    // If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value,
    // we wouldn’t be able to with a struct. Enums handle this case with ease:

    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }

    // let home = IpAddr::V4(127, 0, 0, 1);

    // let loopback = IpAddr::V6(String::from("::1"));

    // another example
    // This enum has four variants with different types:
    // enum Message {

    //     // Quit has no data associated with it at all.
    // Quit,
    // // Move has named fields, like a struct does.

    // Move { x: i32, y: i32 },
    // // Write includes a single String.

    // Write(String),
    // // ChangeColor includes three i32 values.

    // ChangeColor(i32, i32, i32),

    // }

    // enums can have variants with different types and you can even combine an enum
    // to have a field instead of using a struct you can put the data into the enum
    // variant to make life easy enums can include name fields like a struct and can even contain
    // multiple values

    //     Defining an enum with variants such as the ones in Listing 6-2 is similar to defining different kinds of struct definitions, except the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type. The following structs could hold the same data that the preceding enum variants hold:

    // struct QuitMessage; // unit struct
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String); // tuple struct
    // struct ChangeColorMessage(i32, i32, i32); // tuple struct
    // But if we used the different structs, each of which has its own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum defined in Listing 6-2, which is a single type.
    // we can also use implementations with enums
    // There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. Here’s a method named call that we could define on our Message enum:
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    let w = Message::Move { x: 12, y: 15 };
    w.call();

    println!("m is {:?}", m);
    println!("m is {:?}", w);

    // The Option type encodes the very common scenario in which a value could be something or it could be nothing.
    // However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason. Rust doesnt have null but does have an enum that can encode thet a value is present or absent and the enum is Option<T>
    // the T syntax is a generic type parameter
    // all you need to know is that <T> means that the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets used in place of T makes the overall Option<T> type a different type.
    //     The type of some_number is Option<i32>. The type of some_char is Option<char>, which is a different type. Rust can infer these types because we’ve specified a value inside the Some variant. For absent_number, Rust requires us to annotate the overall Option type: the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value. Here, we tell Rust that we mean for absent_number to be of type Option<i32>.

    // When we have a Some value, we know that a value is present and the value is held within the Some. When we have a None value, in some sense it means the same thing as null: we don’t have a valid value. So why is having Option<T> any better than having null?

    // In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value. For example, this code won’t compile, because it’s trying to add an i8 to an Option<i8>:
    // This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.
    //You want some other code to run only if you have a None value, and that code doesn’t have a T value available. The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

    let some_number = Some(5);
    let some_char = Some('e');

    let color = Color::Yellow(10);
    match color {
        Color::Red => println!("The color is red"),
        Color::Green => println!("The color is green"),
        Color::Blue => println!("The color is blue"),
        Color::Yellow(t) => println!("The color is yellow with value {}", t),
    }

    let brand = Cars::Bmw(String::from("M4"));
    match brand {
        Cars::Benz(t) => println!("This is a mercedes benz"),
        Cars::Toyota(s) => println!("This is a toyoa"),
        Cars::Honda(c) => println!("This is a supra"),
        Cars::Bmw(b) => println!("This is a BMW {}", b),
    }

    // let absent_number: Option<i32> = None;
}
