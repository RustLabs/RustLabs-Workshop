fn main(){
    let course:String = String::from("Rust Programming");
    display_course_name(course); 
    //cannot access course after display
 }
 fn display_course_name(my_course:String){
    println!("Course : {}", my_course);
 }
 