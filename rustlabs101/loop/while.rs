fn main() {
    let mut var = 1; //define an integer variable
    let mut found = false; // define a boolean variable
    // define a while loop
    while !found {
        var=var+1;
        //print the variable
        println!("{}", var);
        // if the modulus of variable is 1 then found is equal to true
        if var % 3 == 1 {
          found = true; 
        }
        println!("Loop runs");
    }
  }