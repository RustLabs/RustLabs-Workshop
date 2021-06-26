//declare a struct
struct Course {
    code:i32,
    name:String,
    level:String, 
 }
 fn return_rust_course_info(c1:Course, c2:Course)-> Course{
    println!("I got into function and return values from there");
    if c1.name == "Rust" {
       return c1;
    }
    else{
       return c2;
    }
 }
 
 fn main() {
    //initialize
    let course1 = Course  {
       name:String::from("Rust"),
       level:String::from("beginner"),
       code:130
    };
     let course2 = Course  {
       name:String::from("Java"),
       level:String::from("beginner"),
       code:130
    };
   
   let choose_course = return_rust_course_info(course1, course2);
   println!("I choose to learn {} {} course with code:{}", choose_course.name, choose_course.level, choose_course.code);
 }