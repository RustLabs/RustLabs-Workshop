---
layout: default
title: Continue Statement
parent: Rust for Beginners
nav_order: 49
---

# Continue Statement

- What Is a continue Statement? 

The continue statement, when encountered inside a loop, skips the execution of the rest of the statements in the loop’s body for the current 
iteration and returns the control to the start of the loop.

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/continue_flow.png)

# Using With a for Loop 
- Below is an example of a continue expression, using a for loop.
- The range defined in the for loop is from `0 `to `10` with var variable used for iterating over the loop
   - Within the for loop:
      -  The value of var is printed
      -  When the value of `var` is equal to `4`, the control goes to the start of the loop
      -  The loop executes until the upper bound for the defined range is reached
      
      
```
  fn main() {
  // define a for loop
  for var in 0..10 {
     if var == 4 {
        println!("I encoutered a continue statement");
        continue;
      }
      println!("var: {}", var);
      println!("I did not encounter continue statement");
  }
}
  
```
output:- 

```
var: 0
I did not encounter continue statement
var: 1
I did not encounter continue statement
var: 2
I did not encounter continue statement
var: 3
I did not encounter continue statement
I encoutered a continue statement
var: 5
I did not encounter continue statement
var: 6
I did not encounter continue statement
var: 7
I did not encounter continue statement
var: 8
I did not encounter continue statement
var: 9
I did not encounter continue statement


```

# Using With a while Loop 
- Below is an example of continue expression, using a while loop.
   -  A mutable variable var is defined
   -  A boolean variable found is defined
- Within the while loop body:
   - The value of var is printed
   - When the value of var is equal to `4`, the control goes to the start of the loop.
   - The loop executes until the value of found does not equal true.
   
```
fn main() {
    // define an integer variable
    let mut var = 1; 
    // define a boolean variable
    let mut found = false;
    // define a while loop
    while !found {
      var = var + 1;
      println!("{}", var);
      
      if var == 4 {
          println!("I encoutered a continue statement");
          continue;
        }
        println!("I did not encounter continue statement");
        
        if var == 10{
          found = true;
        }
    }
}

```

Output

```
2
I did not encounter continue statement
3
I did not encounter continue statement
4
I encoutered a continue statement
5
I did not encounter continue statement
6
I did not encounter continue statement
7
I did not encounter continue statement
8
I did not encounter continue statement
9
I did not encounter continue statement
10
I did not encounter continue statement

```
# Using With a loop
- Below is an example of continue expression, using a loop .
   - A mutable variable var is defined
   -  A boolean variable found is defined
- Within the loop body:
    - The value of var is printed
    - When the value of var is equal to `4` , the control goes to the start of the loop
    - The loop executes infinitely
    
Note: This code widget will give an error, ❌, due to limitations of our platform but on the local machine, it will run an infinite loop.

```
fn main() {
  // define an integer variable
  let mut var = 1; 
  // define a loop
  loop {
    var = var + 1;
    println!("{}", var);
    
     if var == 4 {
        println!("I encoutered continue statement");
        continue;
      }
      println!("I did not encounter continue statement");
  }
}

```


# Quiz 

Test your understanding of continue statement in Rust.

1. How many times is the statement “I did not encounter continue statement” printed in the code below?

```
fn main() {
      let mut var = 1; 
      let mut found = false;
      while !found {
          var = var + 1;
          println!("{}", var);
 
          if var == 5 {
              println! ("I encoutered a continue statement");
              continue;
            }
          println!("I did not encounter continue statement");
 
           if var == 6 {
               found = true;
           }
        }
}


```

A) 4 <br>
B) 5 <br>
C) 6 <br>












