fn main() {
    let outer_variable = 112;
    { // start of code block
          let inner_variable = 213;
          println!("block variable: {}", inner_variable);
          let outer_variable = 117;
          println!("block variable outer: {}", outer_variable);
    } // end of code block
      println!("outer variable: {}", outer_variable);
    }