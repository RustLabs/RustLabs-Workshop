//declare a struct
struct Course {
    code:i32,
    name:String,
    level:String, 
 }
 fn display_mycourse_info(c:Course) {
 println!("Name:{}, Level:{} ,code: {}", c .name, c .level, c.code);
 }
 fn main() {
    //initialize
    let course1 = Course  {
       name:String::from("Rust"),
       level:String::from("beginner"),
       code:130
    };
    display_mycourse_info(course1);
     let course2 = Course  {
       name:String::from("Java"),
       level:String::from("beginner"),
       code:130
    };
    display_mycourse_info(course2);
 }
 