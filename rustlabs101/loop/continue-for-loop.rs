fn main() {
    // define a for loop
    for var in 0..10 {
       if var == 4 {
          println!("I encoutered a continue statement");
          continue;
        }
        println!("var: {}", var);
        println!("I did not encounter continue statement");
    }
  }
    