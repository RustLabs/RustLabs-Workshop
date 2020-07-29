---
layout: default
title:  Printing Styles
parent: Rust for Beginners
nav_order: 4
---

Printing Styles

The table below summarizes the macros used to print in Rust.



| macro 	| Printing Style  	|
|-	|-	|
| print!() 	| prints strings to console  	|
| println!() 	| same as print!() but also appends new line character at end of string 	|
| eprint!() 	| prints anything within the parentheses as an error 	|
| eprintln!() 	| same as eprint!() but also appends new line character at end  	|


Let’s discuss each of the macros in detail.

The print!() macro simply prints the output to the console.
# Example 
The following example prints “Rust Programming Course” in one line

```
fn main() {
    print!("Rust Programming");
    print!(" Course");
}

```
