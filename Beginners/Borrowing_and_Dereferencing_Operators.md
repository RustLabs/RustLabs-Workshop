---
layout: default
title: Borrowing and Dereferencing Operators
parent: Rust for Beginners
nav_order: 32
---


# Borrowing and Dereferencing Operators

# Borrowing Operator

Borrowing means to reference the original data binding or to share the data.

References are just like pointers in C.

Two variables are involved in a borrowing relationship when the referenced variable holds a value that the referencing variable borrows. 
The referencing variable simply points to the memory location of the referenced variable.

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/mem_ref_var.png)

The following illustration shows that operand 1 borrows the value of operand 2 using two types of operators:

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/operator_mem.png)

# Types 

Borrowing can be of two types:
   - Shared borrowing
   - A piece of data that is shared by single or multiple variables but it cannot be altered
   - Mutable borrowing
   - A piece of data that is shared and altered by a single variable (but the data is inaccessible to other variables at that time)
   - The following table summarizes the function of these two types.
   
  
| operator 	| operation  	| explanation 	|
|-	|-	|-	|
| Operand1 = & Operand2<br> 	| shared borrow  	| operand 1 can read data of  another operand 2   	|
| Operand1 = & mut Operand2  	| mutable borrow  	| Operand 1 can read and alter data of another operand2  	| 

# Example 
The following example shows a shared borrow and mutable borrow:

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/shared_mutable.png)

```
fn main() {
    let x = 10;
    let mut y = 13;
    //immutable reference to a variable
    let a = &x;
    println!("Value of a:{}", a); 
    println!("Value of x:{}", x); // x value remains the same since it is immutably borrowed
    //mutable reference to a variable
    let b = &mut y;
    println!("Value of b:{}", b);
    println!("Value of y:{}", y); // y value is changed since it is mutably borrowed
}


```
output:- 
```
Value of a:10
Value of x:10
Value of b:13
Value of y:13
 
```
# Dereferencing Operator 

Once you have a mutable reference to a variable, dereferencing is the term used to refer to changing the value of the referenced variable using its address stored in the referring variable.

The following illustration shows that operand 1 mutably borrows the value of operand 2 using & mut and then operand 1 dereferences the value of operand 2 using the `* `operator:

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/derefrencing-op.png)

# Type 

The following table shows the dereferencing operator `*` along with its function .

| operator 	| operation  	| explanation 	|
|-	|-	|-	|
| *Operand1 = Operand2<br> 	| Dereferencing a value   	| <br>point to the value of a mutable borrow variable and can also <br><br>update that variable value  	|


# Example 
The following example shows how to dereference a variable:

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/dereferencing.png)

```

fn main() {
    //mutable reference to a variable
    let mut x = 10;
    println!("Value of x:{}", x);
    let a = & mut x;
    println!("Value of a:{}", a);
    //dereference a variable
    *a = 11;
    println!("Value of a:{}", a);
    println!("Value of x:{}", x); // Note that value of x is updated
}


```
output:

```
Value of x:10
Value of a:10
Value of a:11
Value of x:11

```

# Quiz 

Test your understanding of borrowing and dereferencing operators in Rust.

1.A variable can be updated through a dereference operator if it’s a <br>
a) shared borrow <br>
b) mutable borrow <br>

2. What is the output of the following code?  <br>
```
fn main() {
      let a = &10;
      let b = &mut 9;
      *b = 12;
      println!("Value of a:{}",a);
      println!("Value of b:{}",b);   
}


```
A) Value of a:10 <br>
   Value of b:12 <br>
  
B) Value of a:10 <br>
   Value of b:9 <br>
   
   








  
