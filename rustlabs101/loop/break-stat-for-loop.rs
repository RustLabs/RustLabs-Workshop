fn main() {
    // define a for loop
    for i in 0..10 {
      println!("i:{}", i);
      if i == 5 {
        break;
      }
    }
  }