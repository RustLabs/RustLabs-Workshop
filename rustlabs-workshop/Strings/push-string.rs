fn main() {
    // define a string object
    let mut course = String::from("Rust");
    // push a string
    course.push_str(" Programming");
    println!("This is a beginner course in {}.", course);
  }