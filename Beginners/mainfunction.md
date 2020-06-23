---
layout: default
title:  main function
parent: Rust for Beginners
nav_order: 3
---

# Main function

Let's look again at our first project source code:
```
fn main() {
    println!("Hello, world!");
}

```
It only contains a main function—this is where the execution of the program begins. 
It is a function that takes no arguments (hence the empty parentheses) and returns a unit, also written (). The body of the function, between curly brackets, contains a call to the println!() macro—we can see this is a macro because it ends with !, as opposed to a function. This macro prints the text between parentheses, followed by a new line. We'll see what is a macro in the Macros section.

# Variables

We'll now change the previous program to add a variable:
```
fn main() {
    let name = "world";
    println!("Hello, {}!", name);
}

```
The `{}` part in the string literal is replaced by the content of the name variable. 
Here, we see the type inference in action—we don't have to specify the type of the name variable and the compiler will infer it for us.
We could have also written the type ourselves:

```
let name: &str = "world";

```
(From now on, I'll omit the main function, but this code should be written inside the function.)

In Rust, variables are immutable by default. As such, writing the following will cause a compile-time error:

```
let age = 42;
age += 1;
```
The compiler gives us a very helpful error message:
```
error[E0384]: cannot assign twice to immutable variable `age`
  --> src/main.rs:16:5
   |
15 |     let age = 42;
   |         --- first assignment to `age`
16 |     age += 1;
   |     ^^^^^^^^ cannot assign twice to immutable variable

```
To make a variable mutable, we need to use the mut keyword:
```
let mut age = 42;
age += 1;

```
