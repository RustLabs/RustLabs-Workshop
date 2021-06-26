fn main() {
    //mutable reference to a variable
    let mut x = 10;
    println!("Value of x:{}", x);
    let a = & mut x;
    println!("Value of a:{}", a);
    //dereference a variable
    *a = 11;
    println!("Value of a:{}", a);
    println!("Value of x:{}", x); // Note that value of x is updated
}

