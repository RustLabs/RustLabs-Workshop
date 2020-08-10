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






