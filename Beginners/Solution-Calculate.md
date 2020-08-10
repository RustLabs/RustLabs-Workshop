---
layout: default
title: Solution - Calculate (a + b)^3
parent: Rust for Beginners
nav_order: 35
---

# Solution : Calculate (a + b)^3


```
fn test() {
    let a = 2;
    let b = 2;
    let c = i32::pow(a,3) + i32::pow(b, 3) + ( 3 * a * b * (a + b)) ; 
    println!("{}",c);  
}

```
output 


```
64

```

# Explanation
  
  - On line 2, a variable a with value 2 is declared. <br>
  - On line 3, a variable b with value 2 is declared. <br>
  - On line 4, addition takes place of : <br>
        
    - a​3​​ ( calculated using the function pow)
    - b​3​​ ( calculated using the function pow)
    - 3∗a∗b( multiplied with (a+b))(a + b))(a+b))
        
        
Now you have learned about operators, what if you want to perform an operation on a specific condition? 


