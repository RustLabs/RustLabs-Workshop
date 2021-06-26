#[allow(unused_variables, unused_mut)]
fn main(){
   // define a String object 
   let course = "Rust".to_string();
   // define a String object
   let course_type = " beginner course".to_string();
   // concatenate using the + operator
   let result = course + &course_type;
   println!("{}", result);
}