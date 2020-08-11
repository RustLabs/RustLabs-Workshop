---
layout: default
title: Solution 1 - Check If Even or Odd
parent: Rust for Beginners
nav_order: 42 
---

# Solution 1: Check If Even or Odd

- Solution:
```
fn test(_a:i32) { 
    // check divisibility by 2
    if _a % 2 == 0 {  // execute if divisible
      println!("Number {} is even", _a);
    }
    else { // execute if not divisible
      println!("Number {} is odd", _a);
    }
}

```
Output
```
Number 4 is even
Number 9 is odd

```

# Explanation 
Note: The statement a % b evaluates to the remainder of the division of variable a by variable b.

- if construct
    -  On line 3 and 4, if the condition, i.e., modulus 2 of _a is equal to zero then, prints that the Number is even.
- else construct
    - On line 6 and 7, the else block is executed. If the if block does not execute, then prints that the Number is odd.
