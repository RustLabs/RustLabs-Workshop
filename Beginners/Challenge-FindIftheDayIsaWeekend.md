---
layout: default
title:  Challenge - Find If the Day Is a Weekend
parent: Rust for Beginners
nav_order: 100
---

# Challenge: Find If the Day Is a Weekend

# Problem Statement 

  - An enum Days has been provided to you. It has all the days of the week.
  - A method is_weekend() is incomplete.
    - The task is to complete the method
       - If the day is a weekend it returns 1
       - If the day is a weekday it returns 0

## Input 

```
  Day of the Week
```
 Note: Assume that days of the week are passed to the function

## Output 

```
   1  or  0
```

## Sample Input 
```
Wednesday
```

## Sample Output 

```
   0
```

# Coding Exercise

```
enum Days {
  Monday,Tuesday,Wednesday,Thursday,Friday,Saturday,Sunday
}
impl Days {
   fn is_weekend(&self)->i32{
      // Write code here
      2 
   }
}


```

Good luck!🤞
