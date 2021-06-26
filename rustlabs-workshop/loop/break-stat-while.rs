fn main() {
    let mut i = 1;
    let found = false;
    // define a while loop
    while !found {
      println!("i:{}", i);
      if i == 5 {
        break;
      }
      i = i + 1;    
    }
  }
  
  