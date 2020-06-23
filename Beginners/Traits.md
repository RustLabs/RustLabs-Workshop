# Traits

Traits are a way to specify that a type must implement some methods and/or some types. 
They are similar to interfaces in Java. We can implement a trait on a type and we'll be able to use the methods of this trait on this type as 
long as this trait is imported. This is how we can add methods to types defined in other crates or even the standard library.

Let's write a trait representing a bit set:
```

trait BitSet {
    fn clear(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);
}

```

Here, we don't write the body of the methods, as they will be defined when we implement this trait for a type.

Now, let's implement this trait for the `u64` type:
```

impl BitSet for u64 {
    fn clear(&mut self, index: usize) {
        *self &= !(1 << index);
    }

    fn is_set(&self, index: usize) -> bool {
        (*self >> index) & 1 == 1
    }

    fn set(&mut self, index: usize) {
        *self |= 1 << index;
    }
}


```

As you can see, the bitwise not operator is `!` in Rust, as opposed to `~ `in other languages. With this code, we can call these methods on `u64`:

```
let mut num = 0;
num.set(15);
println!("{}", num.is_set(15));
num.clear(15);

```

Remember the` #[derive(Debug)]` attribute? This actually implements the Debug trait on the following type. We could also manually implement 
the Debug trait on our type, using the same `impl` syntax, if the default implement does not suit our use case.


# Default methods

Traits can contain default methods, which can be convenient for 
the implementor of the trait since fewer methods will need to be implemented. Let's add a `toggle()` default method in the trait:

```
trait BitSet {
    fn clear(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);

    fn toggle(&mut self, index: usize) {
        if self.is_set(index) {
            self.clear(index);
        } else {
            self.set(index);
        }
    }
}

```

Since the new method has a body, we don't need to update our previous implementation. However, we could do it to provide a more efficient implementation, for instance:

```
impl BitSet for u64 {
    // The other methods are the same as before.

    fn toggle(&mut self, index: usize) {
        *self ^= 1 << index;
    }
}



```

# Associated types

We can also have types in a trait that need to be specified. For instance, 
let's implement the Add trait from the standard library on our Point type that we declared earlier, which allows us to use the + operator on our own types:

```
use std::ops::Add;

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, point: Point) -> Self::Output {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}


```

The first line is to import the Add trait from the standard library so that we can implement it on our type.
Here we specify that the associated Output type is Point. Associated types are most useful for return types. Here, the Output of the `add()` method is 
the associated `Self::Output` type.

Now, we can use the `+ `operator on Points:

```
let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, y: 4 };
let p3 = p1 + p2;


```

- Having to specify the output parameter with an associated type (instead of setting it to Self) gives us more flexibility. For instance, we could implement the scalar product for the * operator, which takes two Points and returns a number.

- You can find all the operators that can be overloaded on this page, at https://doc.rust-lang.org/stable/std/ops/index.html.

- Since Rust 1.20, Rust also supports associated constants in addition to associated types.

# Rules

- There are some rules that must be followed in order to use traits. The compiler will throw an error if they are not respected:

   - The trait must be imported in order to use its methods
   - The implementation of a trait must be in the same crate as the trait or the type

The second rule is to avoid conflicts that could otherwise happen when using multiple libraries. We can have such a conflict when two imported traits provide the same method for the same type.


# lab
```

fn main() {
    let mut num = 0;
    println!("{}", num.is_set(5));
    num.set(5);
    println!("{}", num.is_set(4));
    println!("{}", num.is_set(5));
    println!("{}", num.is_set(6));
    num.clear(5);
    println!("{}", num.is_set(4));
    println!("{}", num.is_set(5));
    println!("{}", num.is_set(6));
    num.toggle(5);
    println!("{}", num.is_set(4));
    println!("{}", num.is_set(5));
    println!("{}", num.is_set(6));
    num.toggle(5);
    println!("{}", num.is_set(4));
    println!("{}", num.is_set(5));
    println!("{}", num.is_set(6));

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}

/*trait BitSet {
    fn clear(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);
}*/

impl BitSet for u64 {
    fn clear(&mut self, index: usize) {
        *self &= !(1 << index);
    }

    fn is_set(&self, index: usize) -> bool {
        (*self >> index) & 1 == 1
    }

    fn set(&mut self, index: usize) {
        *self |= 1 << index;
    }

    fn toggle(&mut self, index: usize) {
        *self ^= 1 << index;
    }
}

/*impl BitSet for u64 {
    fn clear(&mut self, index: usize) {
        *self &= !(1 << index);
    }

    fn is_set(&self, index: usize) -> bool {
        (*self >> index) & 1 == 1
    }

    fn set(&mut self, index: usize) {
        *self |= 1 << index;
    }
}*/

trait BitSet {
    fn clear(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);

    fn toggle(&mut self, index: usize) {
        if self.is_set(index) {
            self.clear(index);
        } else {
            self.set(index);
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

use std::ops::Add;

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, point: Point) -> Self::Output {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}



````

[copy code into rust plaground and run ](https://play.rust-lang.org/){: .btn .btn-purple .mr-2 }
