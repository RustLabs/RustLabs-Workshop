fn main() {
    'outer:for i in 1..5 { //outer loop
       println!("Muliplication Table : {}", i);
      'inner:for j in 1..5 { // inner loop
           if i == 3 { continue 'outer; } // Continues the loop over `i`.
           if j == 2 { continue 'inner; } // Continues the loop over `j`.
           println!("{} * {} = {}", i, j, i * j);
      }
    }
   }