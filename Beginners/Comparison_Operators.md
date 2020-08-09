---
layout: default
title: Comparison Operators
parent: Rust for Beginners
nav_order: 28
---

# Comparison Operators


# What Are Comparison Operators?

Comparison Operators are used for comparing the values of two operands.

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/camp_ops.png)

# Types 

Below is the list of comparison operators in Rust.

| operator  	| operation  	| explanation 	|
|-	|-	|-	|
| operand1 > operand2 	| greater then 	| Evaluates to true if operand 1 is greater then Operand 2   	|
| operand1 < operand2  	| lesser then 	| Evaluates to true if operand 1 is lesser then Operand 2 	|
| operand1 <= operand2   	| less then equal to 	| Evaluates to true if operand 1 is lesser or equal to the Operand 2 	|
| operand1 >= operand2 	| greater then equal to 	| Evaluates to true if operand 1 is greater or equal to the Operand 2 	|
| operand1 == operand2 	| equal to 	| Evaluates to true if operand 1 equal to the Operand 2 	|
| operand1 != operand2 	| Not equal to  	| Evaluates to true if operand 1 not equal to the Operand 2 	|

The following example shows the use of comparison operators in a program:

```
fn main() {
    let a = 2;
    let b = 3;
    println!("Operand 1:{}, Operand 2:{}", a , b);
    println!("a > b:{}", a > b);
    println!("a < b:{}", a < b);
    println!("a >= b:{}", a >= b);
    println!("a <= b:{}", a <= b);
    println!("a == b:{}", a == b);
    println!("a != b:{}", a != b);
}

```
output:- 

```
Operand 1:2, Operand 2:3
a > b:false
a < b:true
a >= b:false
a <= b:true
a == b:false
a != b:true

```

# Quiz 

Test your understanding of comparison operators in Rust!

a) What is the output of the following code?

```
fn main() {
  let mut a = true;
  let mut b = true;
  a = a > b && b < a;
  b = !b;
  println!("a: {}", a);
  println!("b: {}", b); 
}

```
A) a: false <br> 
   b: true <br> 
B) a: true <br> 
   b: false <br> 
C) a: true <br> 
   b: true <br> 
D) a: false <br> 
   b: false <br> 


