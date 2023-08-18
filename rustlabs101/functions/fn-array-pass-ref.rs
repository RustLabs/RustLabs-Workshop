fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    modify_my_array(&mut arr);
    println!("Array in Driver Function : {:?}", arr);
 }
 fn modify_my_array(arr:&mut [i32;5]){
    arr[2] = 8;
    arr[3] = 9;
    println!("Array in my Function : {:?}", arr);
 }
 