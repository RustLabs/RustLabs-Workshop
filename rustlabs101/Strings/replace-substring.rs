fn main() {
    // define a growable string variable
    let str = String::from("Rust Programming"); 
    let replace_from = "Programming";
    let replace_to = "Language"; 
    // find if string contains a substring
    let result = str.replace(replace_from, replace_to);
    println!("{} now becomes {}.", str, result);
  }