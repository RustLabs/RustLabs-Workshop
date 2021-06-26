fn main() {
    for i in 1..5{ //outer loop
     println!("Multiplication Table of : {}", i);
      for j in 1..5 { // inner loop
          println!("{} * {} = {}", i, j, i * j);
      }
    }
   }
   