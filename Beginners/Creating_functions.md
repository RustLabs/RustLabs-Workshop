---
layout: default
title: Creating functions
parent: Rust for Beginners
nav_order: 6
---


# Creating functions

- We had a brief introduction to functions when we saw the main function. Let's see how to create functions with parameters and a return value.

- Here's how to write a function that returns the maximum of two numbers:

```
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}


```
The parameters are between parentheses and must be explicitly typed since the type inference only infers the types of local variables. 
This is a good thing since this acts as a documentation. Moreover, this can prevent bugs when we change how we use the parameters or change the value
that is returned. The function can be defined after it is used without any issue. The return type is after `->`. When we return` ()`, we can omit the -> and type.
The last expression in the body of a function is the value returned from the function. You don't need to use return. The return keyword is only needed when you want to return early.

# lab

```

fn main() {
    println!("{}", gcd(10, 15));
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn max2(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    b
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}





```
[copy code into rust plaground and run ](https://play.rust-lang.org/){: .btn .btn-purple .mr-2 }
