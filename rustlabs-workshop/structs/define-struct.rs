//declare a struct
struct Course {
    code:i32,
    name:String,
    level:String, 
 }
 
 fn main() {
    //initialize
    let mut course1 = Course  {
       name:String::from("Rust"),
       level:String::from("beginner"),
       code:130,
    };
    let course2 = Course  {
       name:String::from("Javascript"),
       level:String::from("beginner"),
       code:122,
    };
    //access
    println!("Name:{}, Level:{}, code: {}", course1.name, course1.level, course1.code);
    println!("Name:{}, Level:{}, code: {}", course2.name, course2.level, course2.code); 
    //update
    course1.name = "Java".to_string();
    course1.code = 134;
    println!("Name:{}, Level:{} ,code: {}", course1.name, course1.level, course1.code);
 }
 