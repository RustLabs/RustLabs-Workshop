fn main() {
    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // access value of a tuple
    println!("The value of the tuple at index 0 and index 1 are {} {}",person_data.0,person_data.1);

    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // get individual values out of tuple
    let (w ,x, y, z) = person_data;
    //print values
    println!("Name : {}",w);
    println!("Age : {}",x);
    println!("Weight : {}",y);
    println!("Height : {}",z);
}
 
