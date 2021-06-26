fn main() {
    // define a growable string variable
    let course = String::from("Rust");
    println!("This is a beginner course in {}.", course);
    //capacity in bytes
    println!("Capacity: {}.", course.capacity());
  }