fn main() {
    let string = "Rust Programming".to_string();
    let slice = &string[5..12]; 
    // get characters at 5,6,7,8,9,10 and 11 indexes
    println!("Slice : {}", slice);
 }
    