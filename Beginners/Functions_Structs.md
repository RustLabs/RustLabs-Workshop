# Functions and Structs
- Often, we need to pass a struct instance to a function. For example, in the previous lesson, every time we wanted to print a 
new struct instance we had to write a new print macro to print it. However, we can avoid multiple print statements by writing one print 
statement within a function and calling it when we need it.

# Pass Structs to a Function 
- The structs can be passed to a function and the function can be invoked when required.

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/pass-struct-fn.png)

```
//declare a struct
struct Course {
   code:i32,
   name:String,
   level:String, 
}
fn display_mycourse_info(c:Course) {
println!("Name:{}, Level:{} ,code: {}", c .name, c .level, c.code);
}
fn main() {
   //initialize
   let course1 = Course  {
      name:String::from("Rust"),
      level:String::from("beginner"),
      code:130
   };
   display_mycourse_info(course1);
    let course2 = Course  {
      name:String::from("Java"),
      level:String::from("beginner"),
      code:130
   };
   display_mycourse_info(course2);
}

```


