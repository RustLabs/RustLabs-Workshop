# Tuples

Tuples and structures are similar, except that tuples' fields are unnamed. Tuples are declared inside parentheses, 
with the element separated by a comma:
```
let tuple = (24, 42);
println!("({}, {})", tuple.0, tuple.1);

```
As you can see on the second line, we can access the elements of a tuple with `.index`, where index is a constant and this index starts at `0`.

Tuples can be used to return multiple values from a function. For instance, the `str::split_at()` method returns two strings:

```
let (hello, world) = "helloworld".split_at(5);
println!("{}, {}!", hello, world);

```
# lab 

```
fn main() {
    let tuple = (24, 42);
    println!("({}, {})", tuple.0, tuple.1);

    let (hello, world) = "helloworld".split_at(5);
    println!("{}, {}!", hello, world);
}

```
[copy code into rust plaground and run ](https://play.rust-lang.org/){: .btn .btn-purple .mr-2 }
