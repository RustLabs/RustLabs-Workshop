---
layout: default
title:  Logical Operators
parent: Rust for Beginners
nav_order: 27
---

# Logical Operators


# What Are Logical Operators? 
Logical operators operate on true / false values

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/logical-op.png)

# Types 

The following table summarizes the types and functions of the logical operators:


| operator  	| operation  	| explanation 	|
|-	|-	|-	|
| operand1 && operand2 	| AND 	| Evaluates to true if operand 1 and Operand 2 both evaluates to be true   	|
| operand1 \|\| operand2  	| OR 	| Evaluates to true if operand 1 or Operand 2 true evaluates to be true 	|
|     ! operand1   	| NOT 	| negates the value of single operand  	|


The logical AND and OR are known as Lazy Boolean expressions because the left-hand side operand of the operator is first evaluated. If it is false, there is no need to evaluate the right-hand side operand in 
case of AND. If it is true, there is no need to evaluate the right-hand side operand in case of OR.

The following example shows the use of logical operators in a program:

```
fn main() {
  let a = true;
  let b = false;
  println!("Operand 1:{}, Operand 2:{}", a , b);
  println!("AND:{}", a && b);
  println!("OR:{}", a || b);
  println!("NOT:{}", ! a);
}


```
output:- 

```
Operand 1:true, Operand 2:false
AND:false
OR:true
NOT:false


```

# Quiz

Test your understanding of logical operators in Rust!

1. What is the output of the following code?

```
fn main() {
  let mut a = false;
  let mut b = true;
  a = a && b || ( ! a);
  b = !b;
  println!("a:{}", a);
  println!("b:{}", b); 
}


```

A) a:false <br>

b:true  <br>

B) a:true  <br>

b:false  <br>

C) a:true  <br>

b:true  <br>

D) a:false  <br>

b:false  <br>



