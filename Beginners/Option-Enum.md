---
layout: default
title:  What Is Option? 
parent: Rust for Beginners
nav_order: 98
---

# What Is Option? 

Option is a built-in enum in the Rust standard library. It has two variants Some and None.
![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/enum-option.png)

- Variants:
  - Some(T), returns Some value T
  - None, returns no value

# When to Use Option? 

- Options is a good choice when:
    - The return value is none
        - Rust avoids including nulls in the language, unlike other languages. For instance, the function that returns a value may actually return nothing. So, here the Option variant None comes in handy.
    - The value of the variable is optional
        - The value of any variable can be set to some value or set to none.
    - Out of bound exception is to be displayed
        - This is useful in the case of an array, string or a vector when an invalid index number tries to access it.
    
    
    
# Example 1: Return Value Is None 

The following example shows that if the else construct has no value then it can simply return None.

```
fn main() {

   println!("{:?}", learn_lang("Rust"));

   println!("{:?}", learn_lang("Python"));

}

fn learn_lang(my_lang:&str)-> Option<bool> {

   if my_lang == "Rust" {

      Some(true)

   } else {

      None

   }

}


```

output 

```
Some(true)
None

```
 Note: None does not take a parameter unlike Some.
 
 
# Example 2: Optional Variable Value 

The following example makes level variable of the struct Course as Option of type String. That means that it’s optional
to set any value to it. It can be set to some value or it can be set to none.

```
//declare a struct
struct Course {
   code:i32,
   name:String,
   level: Option<String>, 
}
fn main() {
   //initialize
   let course1 = Course  {
      name:String::from("Rust"),
      level:Some(String::from("beginner")),
      code:130
   };
   let course2 = Course  {
      name:String::from("Javascript"),
      level:None,
      code:122
   };
   //access
   println!("Name:{}, Level:{} ,code: {}", course1.name, course1.level.unwrap_or("Level".to_string()), course1.code);
   println!("Name:{}, Level:{} ,code: {}", course2.name, course2.level.unwrap_or("No level defined!".to_string()), course2.code);
}

```

output 

```
Name:Rust, Level:beginner ,code: 130
Name:Javascript, Level:No level defined! ,code: 12

```

# Example 3: Index Out of Bound Exception #

The example below uses a match statement that takes an index of string
using `match.str.chars().nth(index_no)` and executes the Some block if `index_no` is in range and None block otherwise.


```
fn main() {

  // define a variable

  let str = String :: from("Educative");

  // define the index value to be found

  let index = 12;

  lookup(str, index);

}

fn lookup(str: String, index: usize) {

  let matched_index = match str.chars().nth(index){

    // execute if match found print the value at specified index 

     Some(c)=>c.to_string(),

     // execute if value not found

     None=>"No character at given index".to_string()

     };  

  println!("{}", matched_index);

}



```
output 

```
No character at given index

```

# `is_some()`, `is_none()` Functions 

Rust provides `is_some()` and `is_none()` to identify the return type of variable of type Option, i.e., whether the value of type Option is set to Some or None.


# Example 1 
The following example checks whether the variable value of type Option is set to Some or None.


```
fn main() {
    let my_val: Option<&str> = Some("Rust Programming!");
    print(my_val); // invoke the function
   
}
fn print(my_val: Option<&str>){
     if my_val.is_some(){ // check if the value is equal to some value
        println!("my_val is equal to some value");
    }
    else{
        println!("my_val is equal to none");
    }
}

```
output 

```
my_val is equal to some value


```

We need to do is to ensure that these functions return true or false. That’s where `assert_eq` and `assert_ne` functions come in handy.

- Assert Macros
    - `assert_eq!(left, right)` - evaluates to true if left value is equal to that of right
    - `assert_ne!(left, right)` - evaluates to true if left value is not equal to that of right
    
# Output of assert expression?
 - If the assertion passes no output is displayed, and if doesn’t the code gives an error saying that the assertion failed
 
# Example 2 

The following example uses the `assert_eq!` macro to check whether the variable value of type Option is set to Some or None.

Note: The assertion passes since the expression evaluates to true.

```
fn main() {
    let my_val: Option<&str> = Some("Rust Programming!");
    // pass since my_val is set to some value so left is true, and right is also true
    assert_eq!(my_val.is_some(), true); 
    // pass since my_val is set to some value so left is false, and right is also false
    assert_eq!(my_val.is_none(), false);
}

```







