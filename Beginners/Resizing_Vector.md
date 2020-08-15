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
       - To remove elements at a specific position of the vector, specify the index number within the remove() method.
             
       
       
       
       
       

