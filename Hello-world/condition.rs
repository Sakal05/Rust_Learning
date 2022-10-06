/* ===== If Let Expression ====== */
/* if let is a conditional expression that allows pattern matching. The block of code in the construct executes if the pattern in the condition matches with that of scrutinee expression. */

fn main() {
    //  // define a scrutinee expression
    //  let course = ("Rust", "beginner","course");
    //  // pattern matches with the scrutinee expression
    //  if let ("Rust", "beginner","course") = course {
    //      println!("Wrote all values in pattern to be matched with the scrutinee expression");
    //  } else {
    //      // do not execute this block
    //      println!("Value unmatched");

    // define a scrutinee expression
    // //If the first value or second value matches, it can guess the third value.
    // let course = ("Rust", "beginner", "course");
    // // pattern matches with the scrutinee expression
    // if let ("Rust", "beginner", c) = course {
    //     println!("Wrote first two values in pattern to be matched with the scrutinee expression : {}", c);
    // } else {
    //     // do not execute this block
    //     println!("Value unmatched");
    // }

    // //guessing the second and third value
    // let course = ("Rust", "beginner", "course");
    // // pattern matches with the scrutinee expression
    // if let (b, "beginner", c) = course {
    //     println!("Guessing the two values in pattern to be matched with the scrutinee expression : {} and {}", b, c);
    // } else {
    //     // do not execute this block
    //     println!("Value unmatched");
    // }

    /* Match expression */

    //    // define a variable
    // let x = 5;
    // // define match expression
    // match x {
    //     1 => println!("Java"),
    //     2 => println!("Python"),
    //     3 => println!("C++"),
    //     4 => println!("C#"),
    //     5 => println!("Rust"),
    //     6 => println!("Kotlin"),
    //     _ => println!("Some other value"),
    // };

    // let (a, b) = (2, 6);
    // let opt: &str = "+";
    // match opt {
    //     "+" =>
    //     {
    //         println!("{}", a + b);
    //     },
    //     "-" =>
    //     {
    //         println!("{}",a - b);
    //     },
    //     "*" =>
    //     {
    //         println!("{}",a * b);
    //     },
    //     "/" =>
    //     {
    //         println!("{}",a / b);
    //     },
    //     _ => {println!("not valid input")}
    // };
    //Challenge for even and odd number
    //     let _a = 1;
    //     if _a % 2 == 0 {
    //         print!("Number {} is even", _a);
    //     } else {
    //         print!("Number {} is odd", _a);
    //     }

    //challenge calculator
    
    // test_calculator(3, '+', 2);
    // test_calculator(3, '/', 0);
}

// fn test_calculator(a: i32, operator: char, b: i32) {
//     // Write code here
//     let mut result: i32 = 0;
//     match operator {
//         '+' => {
//             result = a + b;
//         }
//         '-' => {
//             result = a - b;
//         }
//         '*' | '(' => {
//             result = a * b;
//         }
//         '%' => {
//             result = a % b;
//         }
//         '/' => {
//             if a == 0 || b == 0 {
//                 print!("Division by 0 in undefined");
//                 std::process::exit(1);
//             } else {
//                 result = a / b;
//             }
//         }
//         _ => {
//             print!("invalid operator");
//             std::process::exit(2);
//         }
//     }
//     println!("{}", result);
// }