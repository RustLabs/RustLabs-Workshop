// driver function
fn main() {
    let length = 4;
    let width = 3;
    println!("Rectangle lenth:{}", length);
    println!("Rectangle width:{}", width);
    let (area, perimeter) = calculate_area_perimeter(length, width);
    println!("Area: {}, Perimeter: {}", area, perimeter);
}
// calculate area and perimeter
fn calculate_area_perimeter(x: i32, y: i32) -> (i32, i32) {
    // calculate the area and perimeter of rectangle
    let area = x * y;
    let perimeter = 2 * (x + y);
    // return the area and perimeter of rectangle
    (area, perimeter)
}

