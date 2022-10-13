// //we can use function clone() to clone data from other variable
// fn main() {
//     let mut a = String::from("Rust"); // define a String and save in 'a'
//     let b = a.clone(); // b clones a
//     a.push('y');
//     println!("a:{} , b:{}", a, b);  // print 'a' and 'b'
// }


/* Borrowing */

// === RULE 1 ===
// Vice versa, outer block perform the shared borrow, and in the inner block perform the mutable borrow.

// cannot mutable borrow b since its already a shared borrow
// mutable borrow a in outer scope and shared borrow in inner scope
// fn main() {
  
//     let mut a = 1; // mutable variable a is defined
//     println!("variable `a` :{}", a);
//     let b = 1;
//     println!("variable `b` :{}", b);
//     {
//         // for the use of borrow in the inner scope, we can only share borrow
//         let r1 = &a; // no problem
//         println!("variable `r1` references `a` in inner scope(SHARED BORROW(a)) :{}",r1);
//         let r2 = &a; // no problem
//         println!("variable `r1` references `a` in inner scope(SHARED BORROW(a) :{}",r2);
//         println!("r1:{}\nr2:{}", r1, r2);
//         // r1 and r2 scope end here
//     }
    
//     //for outer scope we can mutable borrow

//     let r3 = &mut a; // no problem
//     *r3 = 3;
//     println!("variable `r1` references `a` in outer scope(MUTABLE BORROW(a) and derefernced it and changed value to 3) :{}",r3);
//     let r4 = &b;
//     println!("variable `r3` references `b` in outer scope(SHARED BORROW(b)) :{}",r4);
//     let r5 = &b;
//     println!("variable `r3` references `b` in outer scope(SHARED BORROW(b)) :{}",r5);
//     println!("r3:{}\nr4:{}\nr5:{}", r3 , r4 , r5);
//   }                        

// ==== If we wanna change the variable from main function in another function we need to pass the mutable borrow as argument
// // 'a' an owner variable
// // 'b' a shared borrow
// // 'c' a mutable borrow
// fn example(a: i32, b:& i32,c : &mut i32){
//     println!("a: {}, b: {}, c: {}", a , b , c);
//      *c=9;
//   }
//   fn main(){
//      let a = 1;
//      let b = 2;
//      let mut c = 3;
//      example( a, &b , &mut c);
//      println!("a: {}, b: {}, c: {}", a , b , c);
//   }                                                   
        
// /* Borrowing and Slicing#
// It is possible to borrow a slice of an array, vector or string. Recall the syntax of slicing. It used an & before the name of the variable to be borrowed. */

// fn main() {
//     let arr:[i32;4] = [1, 2, 3, 4]; // define an array
//     let borrow_arr = &arr[0..2]; // slice an array
  
//     println!("arr : {:?}", arr); // print the array
//     println!("sliced_arr : {:?}", borrow_arr); // print the sliced array
  
//     let str = String::from("Rust Programming"); // define a String object
//     let borrow_str = &str[0..2]; // slice the String object
    
//     println!("str : {:?}", str); // print the String Object 
//     println!("sliced_str : {:?}", borrow_str); // print the sliced String
  
//     let my_vec = vec![1, 2, 3, 4, 5]; // define a vector
//     let borrow_vec = &my_vec[0..2]; // slice the vector
    
//     println!("vec: {:?}", my_vec);  // print the vector
//     println!("sliced_vec : {:?}", borrow_vec); // print the sliced vector
//   }

// // == Using Life Time Annotation, which start with ' and small case letter
// #![allow(dead_code)] 
// struct Course{
//    name: String,
//    id : i32,
// }

// fn get_course<'a> (c1: &'a Course, c2: &'a Course) -> &'a Course {
//   if c1.name == "Rust" {
//      c1
//   }
//   else {
//      c2
//   }
// }
// fn main(){
//   let c1: Course = Course {
//       name : String::from("Rust"),
//       id:101,
//     };
    
//    let c2: Course = Course {
//       name : String::from("C++"),
//       id:101,
//     };                                                                                                                                                                    
//     get_course(&c1, &c2);   
// }

/* Rules for elision */
/* Rule # 1#
Each input parameter gets its own lifetime. If the lifetime is not specified, then the lifetime of each parameter is different.          
Elided lifetime: */
// fn fun_name( x: &i32, y: &i32){ 
// } 
//Expanded:
// fn fun_name<'a,'b>( x :& 'a i32, y : & 'b i32){
// }

/* Rule #2
If there is only one input parameter, its lifetime is assigned to all the elided output lifetimes. */
// //Elided Lifetime
// fn fun_name(x: i32) -> &i32{
// }
// // Expaned form:
// fn fun<'a>(x: i32) -> & 'a i32 {
// }

/* Rule # 3#
If there are multiple input lifetimes, one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes. */
// // Elided Lifetime:
// fn fun_name(&self, x : &str) -> & str {
// }
// // Expanded:
// fn fun_name<'a>(& 'a self, x : & 'b str) -> & 'a str {
// }


