fn main(){
    let course: &str = "Rust Programming";
    display_course_name(course); 
    println!("{}",course); // string literal is used after the function call
 }
 fn display_course_name(my_course: &str){
    println!("Course : {}", my_course);
 }
 