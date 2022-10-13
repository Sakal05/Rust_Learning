fn main() {
    // //define a for loop
    // for i in 'a'..'m' {     // the ' .. ' is a Right-exclusive range literal
    //   println!("{}", i);
    // }

    // //we can use enumerate() function to count the loop
    // for (count, variable) in (7..10).enumerate() {
    //     println!("count = {}, variable = {}", count, variable);
    // }

    // /* We can use continue to skip the execution of the rest of the statemnets in thge loop's body and returns to the start of the loop */
    // for var in 0..9 {
    //     if var == 4 {
    //        println!("I encoutered a continue statement");
    //        continue;
    //      }
    //      println!("var: {}", var);
    //      println!("I did not encounter continue statement");
    //  }

    //  let mut var = 1;
    //  let mut found = false;
    //  while !found {
    //      var = var + 1;
    //      println!("{}", var);

    //      if var == 5 {
    //          println! ("I encoutered a continue statement");
    //          continue;
    //        }
    //      println!("I did not encounter continue statement");

    //       if var == 6 {
    //           found = true;
    //       }
    //    }

    // /* Loop Labels */
    // 'outer:for i in 1..5 { //outer loop
    //     println!("Muliplication Table : {}", i);
    //    'inner:for j in 1..5 { // inner loop
    //         if i == 3 { continue 'outer; } // Continues the loop over `i`.
    //         if j == 2 { continue 'inner; } // Continues the loop over `j`.
    //         println!("{} * {} = {}", i, j, i * j);
    //    }
    //  }
    test(4);
}

//funciton to find factorial
fn test(mut n: i32) {

    let mut fact = 1;
    for i in 1..n+1
    {
        fact = fact * i;
    }

    // while (n>0)
    // {
    //     fact = fact*n;
    //     n -= 1;
    // }
    print!("{}", fact);
}

// //challenge 2
// fn test(mut x: i32) {
//     let mut c = 0;
//     while x >= 0 {
//         x = x - 3;
//         c = c+1;
//     }
//     print!("{}", c);
// }

// //challenge: print right-angle triangle
// fn test(n:i32) {
//     //in c++ we write for (int i = 1, i<n+1, i++)
//     for i in 1..n+1 {
//         for j in 1..i+1 {
//             print!("&");
//         }
//         print!("\n");
//     }
//    }