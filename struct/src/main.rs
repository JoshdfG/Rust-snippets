#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}




impl Rectangle {
    fn area(&self)->u32{
        self.width * self.height
    }
    
    fn double_width(&mut self) ->u32{
        self.width *2 
    }
}


#[derive(Debug)]
struct Student  {
    name: String,
    age:u32,
    class:String,
}

fn main(){
    
    let student1 = Student{
        name:String::from("Josh"),
        age:23,
        class:String::from("Masters Student"),
    };
    
    println!("The value of the student struct is {:?}.",student1);
    
    let rect1 = (20,30);

    let rect2 = Rectangle {
        width:30,
        height:70,
    };

    let impl_struct = Rectangle{
        width : 40,
        height : 50,
    };

    println!("The width of the rectangle implentation is {} sqm.",impl_struct.area());


    println!("The area of the rectangle is {} square metres.", rect(rect1));

    println!("The area of the rectangle2 is {:?} sqm.",rect2);

}

fn rect(dimensions:(u32,u32))-> u32{
    dimensions.0 * dimensions.1
}

fn rectwan(rectangle:&Rectangle)-> u32 {
    rectangle.width * rectangle.height
}


