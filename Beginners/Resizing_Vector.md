---
layout: default
title: Resizing a Vector
parent: Rust for Beginners
nav_order: 80
---

# Resizing a Vector

- Add Elements to the Vector 
   - Define a mutable vector variable.
   - To add elements to the vector, use the push method.

The following illustration shows how the size of the vector grows by adding an element:

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/push-vector.png)

```

fn main() {
   // define a vector of size 5
   let mut my_vec = vec![1, 2, 3, 4, 5];
   // print vector
   println!("Vector : {:?}", my_vec);
   // print the capacity of vector
   println!("Capacity of vector: {}", my_vec.capacity());
   // print the length of vector
   println!("Length of the vector : {}",my_vec.len());
   my_vec.push(6);
   my_vec.push(8); 
   // print vector
   println!("Vector : {:?}",my_vec);
   // print the capacity of vector
   println!("Capacity of vector: {}", my_vec.capacity());
   // print the length of vector
   println!("Length of the vector : {}", my_vec.len());
}



```

output 

```
Vector : [1, 2, 3, 4, 5]
Capacity of vector: 5
Length of the vector : 5
Vector : [1, 2, 3, 4, 5, 6, 8]
Capacity of vector: 10
Length of the vector : 7

```

# Remove Elements from the Vector 
 - Define a mutable vector variable.
 - Elements can be removed from the tail or at specific index of the vector.
       - To remove elements from the tail of the vector, use the pop method.
       - To remove elements at a specific position of the vector, specify the index number within the `remove()` method.
           
 ![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/remove-vector.png) 
 

 ```
 
 fn main() {
   // define a vector of size 5
   let mut my_vec = vec![1, 2, 3, 4, 5];
   // print vector
   println!("Vector : {:?}", my_vec);
   // print the capacity of vector
   println!("Capacity of vector: {}", my_vec.capacity());
   // print the length of vector
   println!("Length of the vector : {}", my_vec.len());
   my_vec.pop();
   my_vec.pop(); 
   // print vector
   println!("Vector : {:?}",my_vec);
   // print the capacity of vector
   println!("Capacity of vector: {}", my_vec.capacity());
   // print the length of vector
   println!("Length of the vector : {}", my_vec.len());
}
 

 ```
 
 output:-
 
 ```
 Vector : [1, 2, 3, 4, 5]
Capacity of vector: 5
Length of the vector : 5
Vector : [1, 2, 3]
Capacity of vector: 5
Length of the vector : 3
 
 ```
 
 Note that the `remove()` function requires the index of the vector element to be removed. However, if it is desired to pass the element to be removed, 
 then we need to know the index of the particular element of the vector and then remove it. Let’s explore that in the next lesson using the `.iter()` method.


# Quiz 

Test your understanding of resizing a vector in Rust.


```

fn main() {
  
   let mut my_vec = vec![1, 2, 3, 4, 5];
   println!("Vector : {:?}",my_vec);
   println!("Length of the vector : {}",my_vec.len());
   my_vec.push(8);
   my_vec.push(7);
   my_vec.remove(2);
   my_vec.remove(1); 
   //print vector
   println!("Vector : {:?}",my_vec);
   //print the length of vector
   println!("Length of the vector : {}",my_vec.len());
}
 
   ```
   
A) 

```
Vector : [1, 2, 3, 4, 5]

Length of the vector : 5

Vector : [1, 4, 5, 8, 7]

Length of the vector : 5


```
B) 

```
Vector : [1, 2, 3, 4, 5]

Length of the vector : 5

Vector : [1, 4, 5, 7, 8]

Length of the vector : 5


```



       
             
       
       
       
       
       

