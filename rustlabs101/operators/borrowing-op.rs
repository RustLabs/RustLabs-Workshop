fn main() {
    let x = 10;
    let mut y = 13;
    //immutable reference to a variable
    let a = &x;
    println!("Value of a:{}", a); 
    println!("Value of x:{}", x); // x value remains the same since it is immutably borrowed
    //mutable reference to a variable
    let b = &mut y;
   
    println!("Value of b:{}", b);
    println!("Value of y:{}", y);
     // y value is changed since it is mutably borrowed
    let y = 2;
    println!("Value of y:{}", y);
}

