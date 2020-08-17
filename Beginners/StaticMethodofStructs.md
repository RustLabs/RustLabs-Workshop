---
layout: default
title: Static Method Structs
parent: Rust for Beginners
nav_order: 89
---

# What Are Static Methods? 
- Static methods are the ones that can be invoked without instantiating the struct.

# Declare a Static Method

The following illustration explains how to declare a static method within the impl construct.

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/static-method.png)

Note: If the construct is declared with an impl keyword, it must have one or both types of methods, static or non-static.

# Invoke a Static Method 

A static method can be invoked by following the struct name with the membership operator:: followed by the method name :
![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/call-method.png)

# Example 

The following example creates a static method `my_static_method` and invokes it from the main function.

```
// declare a struct

struct Course {

   name: String,

   level:String,

   code: i32,

}

impl Course {

   // static method

   fn my_static_method(n: String, l: String, c:i32) -> Course {

      Course { 

      name: n, 

      level:l,

      code:c

       }

   }

   //display

   fn display(&self){

      println!("name :{} code:{} of type: {}", self.name, self.code, self.level );

   }

}

fn main(){

   // call the static method

   let c1 = Course::my_static_method("Rust".to_string(), "beginner".to_string(), 132);

   c1.display();

}

```

output 

```
name :Rust code:132 of type: beginner

```


