fn main() {
    // define a growable string variable
    let str = String::from("Rust Programming"); 
    let sub_str = String::from("Rust"); 
    println!("This is a beginner course in {}.", str);
    // find if string contains a substring
    println!("{} is a substring of {}: {}.", sub_str, str, str.contains("Rust"));
  }