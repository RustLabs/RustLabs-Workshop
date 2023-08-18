fn square(n:i32)->i32{
    println!("The value of n inside function : {}", n);
    let m = n * n;
    m // return the square of the number n
  }  
  fn main() {
    let  n = 4;
    println!("The value of n before function call : {}", n);
    println!("Invoke Function");
    println!("\nOutput : {}",square(n));
  }
  
  //fn square(n:i32)->i32{
  //  println!("The value of n inside function : {}", n);
  //  return n * n;
  // }  
  // fn main() {
  //   let  n = 4;
  //  println!("The value of n before function call : {}", n);
  //  println!("Invoke Function");
  //  println!("\nOutput : {}", square(n));
 // }
  
  