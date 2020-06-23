# Array

An array is a fixed-size collection of elements of the same type. We declare them with square brackets:

```

let array = [1, 2, 3, 4];
let array: [i16; 4] = [1, 2, 3, 4];

```
The second line shows how to specify the type of an array. An alternative way to do that is to use a literal suffix:

```
let array = [1u8, 2, 3, 4];
```
A literal suffix is the composition of a literal (that is, a constant) and a type suffix, so with the 1 constant and the u8 type, we get 1u8. Literal suffixes can only be used on numbers. This declares an array of 4 elements of the u8 type. Array indexing starts at 0 and bounds checking is done at runtime. Bounds checking is used to prevent accessing memory that is out of bounds, for instance, trying to access the element after the end of an array. While this can slow down the software a bit, it can be optimized in many cases.
The following code will trigger a panic because the 4 index is one past the end of the array:
```
println!("{}", array[4]);

```

At runtime, we see the following message:

```
thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 4', src/main.rs:5:20
note: Run with `RUST_BACKTRACE=1` for a backtrace.

```
Another way to declare an array is:
```
let array = [0u8; 100];

```
This declares an array of 100 elements, where all of them are 0.

# Slices

Arrays are fixed-size, but if we want to create a function that works with arrays of any size, we need to use another type: a slice.

A slice is a view into a contiguous sequence: it can be a view of the whole array, or a part of it. Slices are fat pointers, in addition to the pointer to the data,
they contain a size. Here's a function that returns a reference to the first element of a slice:

```
fn first<T>(slice: &[T]) -> &T {
    &slice[0]
}


```

Here, we use a generic type without bound since we don't use any operation on values of the `T` type. The `&[T]` parameter type is a slice of `T`. The return type is `&T`, which is a reference on values of the` T` type.
The body of the function is `&slice[0]`, 
which returns a reference to the first element of the slice. Here's how to call this function with an array:

```

println!("{}", first(&array));

```

We can create slice for only a portion of an array, as shown in the following example:

```
println!("{}", first(&array[2..]));

```

`&array[2..]` creates a slice that starts at the 2 index until the end of the array (hence no index after ..). Both indices are optional,
so we could also write &array[..10] for the first `10 `elements of the array, `&array[5..10]` for the elements with the 5 to 9 index `(inclusive)`, or `&array[..]` for all the elements.


# For loops

The for loop is another form of loops that can be used in Rust. It is used to loop over elements of an iterator. An iterator is a structure that produces a sequence of value: it could produce the same value indefinitely or produce the elements of a collection.
We can get an iterator from a slice, so let's do that to compute the sum of the elements in a slice:

```
let array = [1, 2, 3, 4];
let mut sum = 0;
for element in &array {
    sum += *element;
}
println!("Sum: {}", sum);

```

The only surprising part here is `* `in sum `+= *element`. Since we get a reference to the elements of the slice, we need to dereference them in order to access the integers. We used & in front of array to avoid moving it, indeed, we may still want to use this variable after the loop.

Let's write a function that returns the index of an element in a slice, or None if it is not in the slice:


```
fn index<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
    for (index, element) in slice.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}

```

Note: A partial equivalence relation is both symmetric and transitive, but not reflexive. The Eq trait is used when these three properties are satisfied.


Here, we use again a generic type, but this time we use the PartialEq trait bound to be able to use the `== `operator on values of the `T` type. This function returns `Option<usize>`, meaning that it can either return no value` (None)` or the index `(Some(index))`. In the first line of the body, we use` slice.iter().enumerate()` to get the index in addition to the element of the slice. We use pattern matching right after the for keyword in order to assign the index and the element to variables. Inside the condition, we use the return keyword to return a value early.
So if the value is found, it will return the index; otherwise, the loop will end and the None value is returned afterward.

Let's write another function that uses a for loop. It returns the minimum and the maximum of a slice, or None if the slice is empty:


```
fn min_max(slice: &[i32]) -> Option<(i32, i32)> {
    if slice.is_empty() {
        return None;
    }
    let mut min = slice[0];
    let mut max = slice[0];
    for &element in slice {
        if element < min {
            min = element;
        }
        if element > max {
            max = element;
        }
    }
    Some((min, max))
}


```

Here we return multiple values from a function by using a tuple. This time, & is on the left side of in, while previously it was on the right side of it; this is because this for loop is pattern matching against a
reference by using &element. This is something we can do in Rust, thus we don't need to dereference the element anymore with *.

# lab 
```
fn main() {
    let array = [1, 2, 3, 4];
    let array: [i16; 4] = [1, 2, 3, 4];
    let array = [1u8, 2, 3, 4];
    //println!("{}", array[4]);
    println!("{}", first(&array));
    println!("{}", first(&array[2..]));

    let array = [1, 2, 3, 4];
    let mut sum = 0;
    for element in &array {
        sum += *element;
    }
    println!("Sum: {}", sum);

    println!("{:?}", index(&array, &3));
    println!("{:?}", index(&array, &6));
    println!("{:?}", min_max(&[]));
    println!("{:?}", min_max(&[5, 2, 7, 9, 3, 1]));
}

fn first<T>(slice: &[T]) -> &T {
    &slice[0]
}

fn index<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
    for (index, element) in slice.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}

fn min_max(slice: &[i32]) -> Option<(i32, i32)> {
    if slice.is_empty() {
        return None;
    }
    let mut min = slice[0];
    let mut max = slice[0];
    for &element in slice {
        if element < min {
            min = element;
        }
        if element > max {
            max = element;
        }
    }
    Some((min, max))
}


```
[copy code into rust plaground and run ](https://play.rust-lang.org/){: .btn .btn-purple .mr-2 }

