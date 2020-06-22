# References
Let's try the following code, which would work in other programming languages:
```
let p1 = Point { x: 1, y: 2 };
let p2 = p1;
println!("{}", p1.x);

```
We can see that Rust doesn't accept this. It gives the following error:
```
error[E0382]: use of moved value: `p1.x`
 --> src/main.rs:4:20
  |
3 |     let p2 = p1;
  |         -- value moved here
4 |     println!("{}", p1.x);
  |                    ^^^^ value used here after move
  |
  = note: move occurs because `p1` has type `Point`, which does not implement the `Copy` trait

```
This means that we cannot use a value after it is moved. In Rust, values are moved by default instead of being copied, except in some cases, as we'll see in the next sub-section.

To avoid moving a value, we can take a reference to it by prefixing it with `&`:
```
let p1 = Point { x: 1, y: 2 };
let p2 = &p1;
println!("{}", p1.x);

```
This code compiles and, in this case, `p2 `is a reference to` p1`, which means that it points to the same memory location. Rust ensures that it is always safe to use a reference, since references are not pointers, they cannot be `NULL`.

References can also be used in the type of a function parameter. This is a function that prints a point, without moving the value:
```
fn print_point(point: &Point) {
    println!("x: {}, y: {}", point.x, point.y);
}

```
We can use it this way:
```
print_point(&p1);
println!("{}", p1.x);

```
We can still use the `point` after calling `print_point`, because we send a reference to the function instead of moving the point into the function.

# Clone types

An alternative to using references is to clone values. By cloning a value, we don't move it. To be able to clone a point, we can add derive to it:

```
#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}
```
We can now call the clone() method to avoid moving our p1 point:

```
fn print_point(point: Point) {
    println!("x: {}, y: {}", point.x, point.y);
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1.clone();
print_point(p1.clone());
println!("{}", p1.x);

```
# Copy types

Some types are not moved when we assigned a value of these types to another variable. This is the case for basic types such as integers. For instance, 
the following code is perfectly valid:
```
let num1 = 42;
let num2 = num1;
println!("{}", num1);

```
We can still use `num1` even thought we assigned it to `num2`. This is because the basic types implement a special marker: Copy. Copy types are copied instead of moved.

We can make our own types `Copy` by adding `derive` to them:
```
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

```
Since Copy requires Clone, we also implement the latter for our Point type. We cannot derive Copy for a type containing a value that does not implement Copy. 
Now, we can use a Point without having to bother with references:
```
fn print_point(point: Point) {
    println!("x: {}, y: {}", point.x, point.y);
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1;
print_point(p1);
println!("{}", p1.x);

```

# Mutable references

If we want to be able to mutable thought a reference, we need a mutable reference, since everything is immutable by default in Rust. 
To get a mutable reference, simply replace & with `&mut`. Let's write a function that will increment the `x `field of a Point:
```
fn inc_x(point: &mut Point) {
    point.x += 1;
}
```
Here, we see that the Point type is now` &mut`, which allows us to update the point in the method. To use this method, 
our `p1` variable needs to be mut and we also need to take a mutable reference for this variable:

```
let mut p1 = Point { x: 1, y: 2 };
inc_x(&mut p1);

```

# lab 
```
fn main() {
    let mut p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    print_point(p1.clone());
    inc_x(&mut p1);
    println!("{}", p1.x);

    let num1 = 42;
    let num2 = num1;
    println!("{}", num1);
}

fn print_point(point: Point) {
    println!("x: {}, y: {}", point.x, point.y);
}

fn inc_x(point: &mut Point) {
    point.x += 1;
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

```
[copy code into rust plaground and run ](https://play.rust-lang.org/){: .btn .btn-purple .mr-2 }
