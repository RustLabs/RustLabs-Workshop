fn main() {
    // define an integer variable
    let mut var = 1; 
    // define a loop
    loop {
      var = var + 1;
      println!("{}", var);
      
       if var == 4 {
          println!("I encoutered continue statement");
          continue;
        }
        println!("I did not encounter continue statement");
    }
  }