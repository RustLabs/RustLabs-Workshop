---
layout: default
title: Methods
parent: Rust for Beginners
nav_order: 9
---

# Methods

- We can add methods on custom types. Let's write a method to compute the distance of a point to the origin:

```
impl Point {
    fn dist_from_origin(&self) -> f64 {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        (sum_of_squares as f64).sqrt()
    }
}

```
here are a lot of new syntaxes here `(impl Point, as, and .method())`, so let's explain all of them. First of all, methods of a type are declared within 
the impl Type` {}` construct. This method takes a special parameter:` &self`. This parameter is the instance the method is called on, like this in other programming languages.
The `&` operator before self means that the instance is passed by immutable reference. As we can see, it is possible to call methods on basic types in `Rust—self.x.pow(2)` computes the power of two of the` x` field. 
We can find this method, and many others, in the documentation, at https://doc.rust-lang.org/stable/std/primitive.i32.html#method.pow . In the last expression of the method, we cast the `sum_of_squares` integer to `f64` before computing its square root, 
because the `sqrt()` method is defined only on floating points.

Let's create a method that will update the fields of the structure:

```
impl Point {
    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

```
The difference with the previous method is that `self` is now a mutable reference, `&mut`.

# Constructors

Rust does not provide constructors, but a common idiom is to create a `new()` static method, also called an associated function:

```
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }
}

```

- The difference with a normal method is that it does not take &self (or one of its variations) as a parameter.

- Self is the type of the self value; we could have used Point instead of Self.

- When the field name is the same as the value assigned, it is possible to omit the value, as a shorthand:

```
fn new(x: i32, y: i32) -> Self {
    Self { x, y }
}


```
When we create an instance of Point with the call to its constructor `(let point = Point::new();)`, this will allocate the value on the stack.

We can provide multiple constructors:
```
impl Point {
    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }
}
```
