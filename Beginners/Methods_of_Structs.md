---
layout: default
title: Methods of Structs
parent: Rust for Beginners
nav_order: 87
---

# Methods of Structs

- What Are Methods? 

Methods are just like user-defined functions. They are like functions, but the only difference lies in the fact that methods are declared specifically 
within the struct context.

- Declare a Method 

The method is like a regular function except that the &self parameter is passed to it and the items within the function are accessed through it.

```
self.ite
```
Here self is the calling instance, i.e., it is referencing to the struct.

- Call a Method 

An instance of the struct has to be created to invoke it. This is similar to invoking a struct.

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/method-struct-call.png)

 Why struct method? The main advantage is that all the data related to the instance is put inside the `impl` block rather than putting it in different places.
 
 # Example 
 
The example below shows declares a method `name_code` function within the `impl` construct:

```
//declare a struct

struct Course {

    name: String,

    level: String,

    code:i32

}

//impl construct to define struct methods

impl Course {

    fn name_code(&self) -> String {

        format!("{} {}", self.name, self.code)

    }

}



fn main() {

    let course_1 = Course {

        name: "Rust".to_string(),

        level:"beginner".to_string(),

        code:132

    };

    //call the non-static method

    println!("This is a {} course: {}", course_1.level, course_1.name_code());

}


```

output 

```
This is a beginner course: Rust 132

```
# Quiz 

Test your understanding of methods of structs.

1. Which code block has the method enclosed in it? <br>
A) `impl` <br>
B) `fn` <br>







