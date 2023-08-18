// fn main() {
//    let outer_variable = 112;
//    { // start of code block
//          let inner_variable = 213;
//          println!("block variable inner: {}", inner_variable);
//          println!("block variable outer: {}", outer_variable);
//    } // end of code block
//      println!("inner variable: {}", inner_variable); // use of inner_variable outside scope
//   }


fn main() {
    let outer_variable = 112;
    let inner_variable = 213;
    { // start of code block
          println!("block variable inner: {}", inner_variable);
          println!("block variable outer: {}", outer_variable);
    } // end of code block
      println!("inner variable: {}", inner_variable);
    }
  
  