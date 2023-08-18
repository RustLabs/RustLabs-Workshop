fn main() {
    let arr = [1, 2, 3, 4, 5];
    modify_my_array(arr);
    println!("Array in Driver Function : {:?}", arr);
 }
 fn modify_my_array(mut arr:[i32;5]){
    arr[2] = 8;
    arr[3] = 9;
    println!("Array in my Function : {:?}", arr);
 }
 

// fn main() {
//    let arr = [1, 2, 3, 4, 5];
//    println!("Array in Driver Function : {:?}", arr);
//    calculate_mean(arr);
// }
// fn calculate_mean(arr:[i32;5]){
//    let mut sum = 0;
//    for i in 0..5 {
//        sum += arr[i];
//    }
//    println!("Mean of array values: {}", sum/5);
// }
 