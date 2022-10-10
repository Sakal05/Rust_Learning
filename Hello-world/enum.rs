// // make this `enum` printable with `fmt::Debug`.
// #[derive(Debug)]
// enum KnightMove{
//    Horizontal, Vertical
// }
// fn main() {
//    // use enum
//    let horizontal_move = KnightMove::Horizontal;
//    let vertical_move = KnightMove::Vertical;
//    // print the enum values
//    println!("Move 1: {:?}", horizontal_move);
//    println!("Move 2: {:?}", vertical_move);
// }

// /* Enums with Data Type */
// // make this `enum` printable with `fmt::Debug`.
// #[derive(Debug)]
// enum KnightMove{
//    Horizontal(String), Vertical(String)
// }
// fn main() {
//    // invoke an enum
//    let horizontal_move = KnightMove::Horizontal("Left".to_string());
//    let vertical_move = KnightMove::Vertical("Down".to_string());
//    // print enum
//    println!("Move 1: {:?}", horizontal_move);
//    println!("Movw 2: {:?}", vertical_move);
// }

// /* Methods of enums */

// //#![allow(dead_code)] is declared which helps to remove warning if any variable is left uninitialized.
// #![allow(dead_code)]
// //#[derive(Debug)] is declared which helps to print the values of the enum.
// #[derive(Debug)]
// // declare an enum
// enum TrafficSignal{
//   Red, Green, Yellow
// }
// //implement a Traffic Signal methods
// impl TrafficSignal{
//   // if the signal is red then return
//    fn is_stop(&self)->bool{
//      match self{
//        TrafficSignal::Red=>return true,
//        _=>return false
//      }
//    }
// }
// fn main(){
//   // define an enum instance
//   let action = TrafficSignal::Red;
//   //print the value of action
//   println!("What is the signal value? - {:?}", action);
//   //invoke the enum method 'is_stop' and print the value
//   println!("Do we have to stop at signal? - {}", action.is_stop());
// }

// /* Enums and Match control Flow operator */
// enum KnightMove{
//     Horizontal,Vertical
//  }
//  // print function 
//  fn print_direction(direction:KnightMove) {
//     // match statement
//     match direction {
//        //execute if knight move is horizontal
//        KnightMove::Horizontal => {
//           println!("Move in horizontal direction");
//        },
//         //execute if knight move is vertical
//        KnightMove::Vertical => {
//           println!("Move in vertical direction");
//        }
//     }
//  }
//  fn main() {
//     // invoke function `print_direction`
//     let knight1 = KnightMove::Horizontal;
//     let knight2 = KnightMove::Vertical;
//     print_direction(knight1);
//     print_direction(knight2);
//  }

// /* Enums and Structures */
// // make this `enum` printable with `fmt::Debug`.
// #[derive(Debug)]
// //define an enum
// enum KnightMove{
//    Horizontal, Vertical
// }
// #[derive(Debug)]
// // make this `struct` print values of type `enum`  with `fmt::Debug`.
// struct Player {
//    color:String,
//    knight:KnightMove
// }
// fn main() {
//       // instance 1
//       let p1 = Player{
//       color:String::from("black"),
//       knight:KnightMove::Horizontal
//    };
//       // instance 2
//       let p2 = Player{
//       color:String::from("white"),
//       knight:KnightMove::Vertical
//    };
//    println!("{:?}", p1);
//    println!("{:?}", p2);
// }

/* ========= Option and enum */
//Option is a built-in enum in the Rust standard library. It has two variants Some and None.
// The reason to use some and none is because we can either set the value of the variable to any value or set it to none.
// //declare a struct
// struct Course {
//     code:i32,
//     name:String,
//     level: Option<String>, 
//  }
//  fn main() {
//     //initialize
//     let course1 = Course  {
//        name:String::from("Rust"),
//        level:Some(String::from("beginner")),
//        code:130
//     };
//     let course2 = Course  {
//        name:String::from("Javascript"),
//        level:None,
//        code:122
//     };
//     //access
//     println!("Name:{}, Level:{} ,code: {}", course1.name, course1.level.unwrap_or("Level".to_string()), course1.code);
//     println!("Name:{}, Level:{} ,code: {}", course2.name, course2.level.unwrap_or("No level defined!".to_string()), course2.code);
//  }

//Example of using some and none with match
// fn main() {
//     // define a variable
//     let str = String :: from("Educative");
//     // define the index value to be found
//     let index = 5;
//     lookup(str, index);
//   }
//   fn lookup(str: String, index: usize) {
//     let matched_index = match str.chars().nth(index){
//       // execute if match found print the value at specified index 
//        Some(c)=>c.to_string(),
//        // execute if value not found
//        None=>"No character at given index".to_string()
//        };  
//     println!("{}", matched_index);
//   }


//   //example of using option with some
//   fn main() {
//     let my_val: Option<&str> = Some("Rust Programming!");
//     print(my_val); // invoke the function
   
// }
// fn print(my_val: Option<&str>){
//     //execute if there's a value in my_value
//      if my_val.is_some(){ // check if the value is equal to some value
//         println!("my_val is equal to some value");
//     }
//     else{
//         println!("my_val is equal to none");
//     }
// }

// //example: The following example uses the assert_eq! macro to check whether the variable value of type Option is set to Some or None.
// fn main() {
//     let my_val: Option<&str> = Some("Rust Programming!");
//     // pass since my_val is set to some value so left is true, and right is also true
//     assert_eq!(my_val.is_some(), true); 
//     // pass since my_val is set to some value so left is false, and right is also false
//     assert_eq!(my_val.is_none(), false);
// }

/* ========== Result and Enum ========== */
/* 
- Result is a built-in enum in the Rust standard library. It has two variants Ok(T) and Err(E).
Ok(T), returns the success statement of type T
Err, returns the error statement of type E.

===== When to use Result?
Result should be used as a return type for a function that can encounter error situations. 
Such functions can return an Ok value in case of success or an Err value in case of an error.
*/
// fn main() {
//     println!("{:?}",file_found(true)); // invoke function by passing true 
//     println!("{:?}",file_found(false)); // invoke function by passing false
//  }
//  fn file_found(i:bool) -> Result<i32,bool> {
//     if i { // if true
//        Ok(200) // return Ok(200)
//     } else { // if false
//        Err(false) // return Err(false)
//     }
//  }

//  //Example:
//  fn main() {
//     let check1 = divisible_by_3(6);
//     if check1.is_ok(){ // check if the function returns ok
//        println!("The number is divisible by 3");
//     }
//     else{
//        println!("The number is not divisible by 3");
//     }
//     let check2 = divisible_by_3(2);
//     if check2.is_err(){ // check if the function returns error
//        println!("The number is not divisible by 3");
//     }
//     else{
//        println!("The number is divisible by 3");
 
//     }
//  }
//  fn divisible_by_3(i:i32)->Result<String,String> {
//     if i % 3 == 0 { // check i modulus 3
//        Ok("Given number is divisible by 3".to_string())
//     } else {
//        Err("Given number is not divisible by 3".to_string())
//     }
//  }

//Challenge
/* Problem Statement#
An enum Days has been provided to you. It has all the days of the week.
A method is_weekend() is incomplete.
The task is to complete the method
If the day is a weekend it returns 1
If the day is a weekday it returns 0 */

enum Days {
    Monday,Tuesday,Wednesday,Thursday,Friday,Saturday,Sunday
  }
  impl Days {
     fn is_weekend(&self)->i32{
        // Write code here
        match self {
            Days::Saturday => 1,
            Days::Sunday => 1,
            _ => 0,
        }
     }
  }

  fn main() {
    let day = Days::Saturday;
    //println!("{}", Days::is_weekend(&day)); //when passing the variable that is out of scope we need to pass the address of that variable into the argument of the function
    println!("{}", day.is_weekend());
  }

