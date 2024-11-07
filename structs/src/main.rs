// Struct
// Note : Struct declaration doesn't require semicolon at End
#[derive(Debug)]
struct Student {
    name : String,
    email : String,
    age : u8,
    marks : u8,
    favourite_subject : String
}

impl Student {
    // method
    fn getMarks(&self)-> u8{
        self.marks
    }
}

impl Student {
    // associated function
    fn new(name : String,email : String,age : u8,marks : u8,favourite_subject : String) -> Self {
        Student {
            name,
            email,
            age,
            marks,
            favourite_subject
        }
    }
}

// Tuple Struct
struct Vector(i32,i32,i32);

// Unit-like Struct
// struct AlwaysEqual;

fn main() {
    // instance of a struct
    let vec1 = Vector (10,20,30);

    println!("Vector : {}, {}, {}", vec1.0,vec1.1,vec1.2);

    let student1 = Student {
        name : String::from("Mahendra Dani"),
        email : String::from("mahendra@example.com"),
        age : 20,
        marks : 98,
        favourite_subject : String::from("Maths")
    };

    // ..instace syntax is allowed
    let student2 = Student {
        name : String::from("Vikram"),
        email : String::from("vikram@example.com"),
        age : 20,
        marks : 80,
        ..student1
    };

    let student3 = Student::new(String::from("Ram"),String::from("ram@example.com"),20,75,String::from("Physics"));
    println!("Student : {:#?}", student3);

    // Printing using derive(Debug) anotation
    println!("{:#?}",student2);
    println!("Marks: {}",student2.getMarks());

    // accessing fields
    let name1 = student1.name;
    let name2 = student2.name;

    println!("Names : {}, {}",name1,name2);


}
