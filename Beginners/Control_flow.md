# Control flow

We'll now look at how to write conditions and loops in Rust. Conditions are useful to execute a block of code when a certain situation happens, 
and loops allow you to repeat a block of code a number of times, until a condition is met.

# Writing a condition

Similar to other languages, Rust conditions are expressed with the `if` and `else` keywords:


```
fn main(){

let number1 = 24;
let number2 = 42;
if number1 > number2 {
    println!("{} > {}", number1, number2);
 } else {
    println!("{} <= {}", number1, number2);
}
}
```
[copy code into rust plaground and run ](https://play.rust-lang.org/){: .btn .btn-purple .mr-2 }

- However, they do not require parentheses around the conditional expression. Also, this expression must be of the bool type: you cannot use a number as you would in other languages.
One particularity of Rust conditions, like many other constructs, is that they are expressions. The last expression of each branch is the value of this branch. Be careful though, the type of each branch must be the same. 
For instance, we can get the minimum number of the two numbers and put it into a variable:
