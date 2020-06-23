---
layout: default
title: Creating Structures
parent: Rust for Beginners
nav_order: 7
---

# Creating structures

- Sometimes, we have multiple values that only make sense together, such as the two coordinates of a point.
Structures are a way to create new types that contains multiple members.

- Here is how we would create the aforementioned Point structure:

```
struct Point {
    x: i32,
    y: i32,
}

```
to create a new point and access its members, we use the following syntax:

```
let point = Point {
    x: 24,
    y: 42,
};
println!("({}, {})", point.x, point.y);


```
What if we want to print the point as a whole?

Let's try the following:

```
println!("{}", point);

```
The compiler does not accept this:
```
error[E0277]: the trait bound `Point: std::fmt::Display` is not satisfied
 --> src/main.rs:7:20
  |
7 |     println!("{}", point);
  |                    ^^^^^ `Point` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
  |
  = help: the trait `std::fmt::Display` is not implemented for `Point`
  = note: required by `std::fmt::Display::fmt`

```
the `{}` syntax is used to display a value to the end user of the application. Nevertheless, there's no standard way to display arbitrary structures. 
We can do what the compiler suggests: using the `{:?}` syntax. That requires you to add an attribute to the structure, so let's change it:
```
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

println!("{:?}", point);
```
The `#[derive(Debug)]` attribute tells the compiler to automatically generate the code to be able to print a debug representation of the structure.
We'll see how this works in the section about traits. It prints the following:
```
Point { x: 24, y: 42 }

```
Sometimes, the structure contains a lot of nested fields and this representation is hard to read.
To remedy that, we can use the `{:#?}` syntax to pretty-print the value:

```
println!("{:#?}", point);

```
This gives the following output:
```
Point {
    x: 24,
    y: 42
}

```
The documentation describes what other formatting syntax can be used: https://doc.rust-lang.org/stable/std/fmt/.

# lab 


```
fn main() {
    let point = Point {
        x: 24,
        y: 42,
    };
    println!("({}, {})", point.x, point.y);
    println!("{:?}", point);
    println!("{:#?}", point);
    println!("{}", point.dist_from_origin());
    println!("{}", Point { x: 3, y: 4 }.dist_from_origin());
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new2(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn dist_from_origin(&self) -> f64 {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        (sum_of_squares as f64).sqrt()
    }
}



```
[copy code into rust plaground and run ](https://play.rust-lang.org/){: .btn .btn-purple .mr-2 }

