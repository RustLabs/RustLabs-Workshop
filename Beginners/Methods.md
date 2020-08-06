---
layout: default
title: Solution 2 - Display Output Using Placeholders
parent: Rust for Beginners
nav_order: 9
---

# Solution 2: Display Output Using Placeholders

Solution:

```
fn test() {
    println!("{}", 1);
    println!("{}{}", 2, 2);
    println!("{}{}{}", 3, 3, 3);
    println!("{}{}{}{}", 4, 4, 4, 4);
    println!("{}{}{}{}{}", 5, 5, 5, 5, 5);
}

```

# Explanation 

   - On line 2, println! takes a placeholder {} and 1.

   - On line 3, println! takes two placeholders {}{} and two values 2 and 2.

   - On line 4, println! takes three placeholders {}{}{} and three values 3, 3, and 3.

   - On line 5, println! takes four placeholders {}{}{}{} and four values 4, 4, 4, and 4.

   - On line 6, println! takes five placeholders {}{}{}{}{} and five values 5, 5, 5, 5, and 5.

Now you have learned the basics of Rust and how to print on the console. But what if you want to print and store your results?
so lets learn with rustlabs - variable 




