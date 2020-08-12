---
layout: default
title: Solution 1 - Find The Factorial
parent: Rust for Beginners
nav_order: 53
---


# Solution 1 - Find The Factorial

```

fn test(n:i32) {

   let mut factorial = 1; // define a mutable variable factorial



   if n < 0 { // check if factorial is less than zero

      println!("0"); // print 0

   }

   else if n == 0 {  // check if factorial is equal to 0

      println!("1"); // print 1

   }

   else // go here if the above two conditions are false

   {

      for i in 1..n + 1{

         factorial = factorial * i  

      }

      println!("{}", factorial); // print the factorial 

   }

}

fn main(){

    print!("factorial (4) : ");

    test(4);

    print!("factorial (6) : ");

    test(6);

}



```


output 

```
factorial (4) : 24
factorial (6) : 720

```

# Explanation 

 - On line 2 a mutable variable factorial is initialized to 1.
 - if construct
    - As factorials exist only for positive numbers, the condition n < 0 on line 4, checks if the value of input number n is less than 0 , it prints 0 on line 5.
        - if the if condition fails then else if is executed.
    - else if construct
        - As the factorial of 0 is 1, the condition n == 0 on line 7, checks if the value of input number n is equal to 0 then it prints 1 on line 8.
        - if the else if condition fails then else block is executed.
    - else construct
        - A for loop is defined within the else block.
    - for loop definition
        - On line 12, the for loop iterates from 1 to n+1 times.

    - for loop body
           - Within each iteration, on line 13 factorial is multiplied with i and the updated value is saved in factorial.
           - The loop iterates until the i is equal to n + 1.

    - The value of factorial is printed on line 15.
