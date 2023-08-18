//function definition
fn my_func(param_1:i32, param_2:i32) {
    println!("The first value passed inside function : {}", param_1);
    println!("The second value passed inside function : {}", param_2);
  }
  fn main() {
    let value_1 = 1;
    let value_2 = 2;
    //calling the function
    my_func( value_1, value_2 );
    println!("Function ended");
  }
  