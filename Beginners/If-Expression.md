---
layout: default
title:  If Expression
parent: Rust for Beginners
nav_order: 37
---


# If Expression

There can be multiple conditional constructs using an if statement.

  -  If expression

   - If…else expression

   -  If…else if…else expression

   -  Nested if expression

   -  Shorthand if expression

Let’s discuss each one of them in detail:-

# If Expression 

If expression takes a condition. If the condition within the if expression evaluates to be true, then the block of code is executed.

- Syntax 
The general syntax is:

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/if_condition.png)

# Illustration

The following flow chart explains the concept of an if statement:
![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/if-illu-flow.png)

```
fn main() {
      //define a variable  
      let learn_language = "Rust";
      // if construct 
      if learn_language == "Rust" { 
         println!("You are learning Rust language!");
      }
}



```

output:- 

```
You are learning Rust language!
```

# If…else Expression

In an `if..else` construct, if the condition within the if expression evaluates to be false, then the statement within the else block is executed.

- Syntax 

The general syntax is:

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/if-else-condi.png)


# Illustration 

The following flow chart explains the concept of an if..else statement:
![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/if-else-flow.png)


```
fn main() {
    //define a variable 
    let learn_language = "Rust";
    // if else construct 
    if learn_language == "Rust" { 
        println!("You are learning Rust language!");
    }
    else {
      println!("You are learning some other language!");
    } 
}

```
Output
```
You are learning Rust language!
```

# if…else if…else Expression

If there are multiple conditions to be checked, then if..else if..else construct is used.

Syntax 
The general syntax is:
![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/if-else-if-else-condi.png)

# llustration 
The following flow chart explains the concept of an if..else if..else expression:

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/if-else-if-else-flow.png)


```
fn main() {
      //define a variable 
      let learn_language="Rust";
      // if..elseif..else construct 
      if learn_language == "Rust" { 
         println!("You are learning Rust language!");
      }
      else if learn_language == "Java" { 
         println!("You are learning Java language!");
      }
      else {
         println!("You are learning some other language!");
      } 
}



```
Output
```
You are learning Rust language!

```

# Nested if Expression 
An if expression inside the body of another if expression is referred to as a nested if expression.
- Syntax 
An if construct is enclosed within an if construct.
The general syntax is:
![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/nested-if.png)

Note: The nested if expression can also be written with a AND expression in an if.
```
if condition1 && condition2 
{
 //statement
}

```
This is true only if the second if statement is the only thing inside the first if.

# Illustration

The following flow chart explains the concept of a nested if statement .

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/nested-if-flow.png)


Note: There can be as many levels of nesting as you want.

```
fn main() {
    //define a variable 
    let learn_language1 = "Rust";
    let learn_language2 = "Java";
    // outer if statement
    if learn_language1 == "Rust" {  // inner if statement
        if learn_language2 == "Java"{
              println!("You are learning Rust and Java language!");
        }
    }
    else {
      println!("You are learning some other language!");
    } 
}


```

output:-

```
You are learning Rust and Java language!

```

