fn main() {
    //define a tuple
    let mut person_data = ("Alex", 48, "35kg", "6ft");
    //print the value of tuple
    println!("The value of the tuple at index 0 and index 1 are {} {}", person_data.0, person_data.1);
    //modify the value at index 0
    person_data.0 = "John";
    //print the modified value
    println!("The value of the tuple at index 0 and index 1 are {} {}", person_data.0, person_data.1);
}