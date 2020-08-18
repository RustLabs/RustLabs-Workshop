---
layout: default
title: What Are Generics ?
parent: Rust for Beginners
nav_order: 103
---

# What Are Generics? 

- Generics are a way of generalizing types; they define the data type at run time. Generics are called parametric polymorphism in type theory. 
‘Poly’ is multiple, ‘morph’ is form over a given parameter (‘parametric’) meaning multiple forms of a given parameter.
- They can be applied to methods, functions, structures, enumerations, collections, and traits. This helps to reuse the same code but with a different type.

- Syntax 

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/generic-syntax.png)

The `<T>` is known as the type parameter and is used to declare a generic construct. T can be any data-type.

- Example 1: Generic Function 

The following example defines a generic function that displays the value passed to it as a parameter. 
The first value type is a string and the second value is an integer of type `i32`.


- Note: For printing a value of the passed parameter, write use `std::fmt::Display` prior to function definition and after the generic function’s name 
write `<T:Display>` following the function name prior to writing the passing parameters.

Note:` Display` is a trait

```
fn main(){
   println!("- Passing a string literal"); 
   concatenate(" Rust ", " Programming "); 
   println!("- Passing an integer"); 
   concatenate(10 as i32, 1 as i32);
   
}
use std::fmt::Display;
fn concatenate<T:Display>(t:T, s:T){
   let result = format!("{}{}", t , s);
   println!("{}", result);
}

```
output 

```
- Passing a string literal
 Rust  Programming 
- Passing an integer
101

```

# Example 2: Generic Vector 

The following example creates a vector `my_int_vector` of type `i32`:

```
fn main(){
   let mut my_int_vector: Vec<i32> = vec![1,2];
   my_int_vector.push(3);
   println!("{:?}",my_int_vector);
   // my_int_vector.push("Rust"); // mismatched types error
}

```
output 

```
[1, 2, 3]

```
Note : while making a vector in the same function, we cannot actually do this. However, a vector of type T can be passed to the function.

```

use std::fmt::Display;
fn print_vec<T:Display>(v: &[T]) {
  for i in v.iter() {
      print!("{}", i)
  }
  println!("");
}

fn main() {
  let int_vec = [1, 2, 3, 4, 5]; // define a vector of type integer
  
  println!("Call to the function with vector of integers");
  
  print_vec(& int_vec); // pass vector of type integer to the function
  
  println!("Call to the function with vector of strings");
  
  let str_vec = ["Rust", "Programming"]; // define a vector of type string

  print_vec(&str_vec); // pass vector of type String to the function
}




```

output 

```

Call to the function with vector of integers
12345
Call to the function with vector of strings
RustProgramming



```

# Example 3: Generic Struct

The following example creates a struct Rectangle. The struct gets invoked with type instances of type i32 and f32 respectively:

```
struct Rectangle<T> {
   width:T,
   height:T
}
fn main() {
   //generic type of i32
   let r1:Rectangle<i32> = Rectangle{width:250, height:150};
   println!("Width:{}, Height:{}", r1.width, r1.height);
   //generic type of String
   let r2:Rectangle<f32> = Rectangle{width:240.0, height:250.0};
   println!("Width:{}, Height:{}", r2.width, r2.height);
   
}

```
output 

```
Width:250, Height:150
Width:240, Height:250

```

# Quiz 

Test your understanding of generics in Rust.
1. Which of the following types are not allowed to be made generic in Rust! <br> 

A) vectors <br> 
B) arrays <br> 
C) structs <br> 
D) enums <br> 









