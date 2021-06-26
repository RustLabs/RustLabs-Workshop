fn main() {
    //define a mutable array of size 4
    let mut arr:[i32;4] = [1, 2, 3, 4]; 
    println!("The value of array at index 1: {}", arr[1]);
    arr[1] = 9;
    println!("The value of array at index 1: {}", arr[1]);
}
