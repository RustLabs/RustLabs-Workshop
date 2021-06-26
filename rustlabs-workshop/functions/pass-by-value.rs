fn square(mut n:i32){
    n = n * n;
    println!("The value of n inside function : {}", n);
  }
  fn main() {
    let n = 4;
    println!("The value of n before function call : {}", n);
    println!("Invoke Function");
    square(n);
    println!("\nThe value of n after function call : {}", n);
  }