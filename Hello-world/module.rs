/* ======= Module ========== */
/* What Are Modules?#
Modules are a collection of items that can contain structs, functions, enums, vectors, arrays, etc. */

// ==== Rule 1 =====
/*Rule No: 1#
If an item is public it can be accessed from anywhere, i.e., within main function or any other module. */

// // declare a module
// mod r {
//     //The pub keyword makes the item public and visible outside its scope.
//     pub fn print_statement() {
//         println!("Hi, this a function of module r");
//     }
// }
// // main function
// fn main() {
//     println!("Let's go inside the module");
//     // invoke a module 'r'
//     r::print_statement();
// }

// /* Rule No: 2#
// If an item is private it can be accessed using its parent module meaning it can be accessed within the module but not outside it. */
// // declare a module
// mod r{
//     fn my_private_function(){
//       println!("Hi, I'm a private function within the module");
//     }
//     pub fn my_public_function(){
//       //! also works without writing self i.e.
//       //! my_private_function();
//       println!("Hi,I'm a public function within the module");
//       println!("I'll invoke private function within the module");
//       self::my_private_function(); 
      
//     }
//   }
//   // main function
//   fn main() {
//     println!("Let's go inside the module");
//     // invoke a module 'r'
//      r::my_public_function();
//   }

// //==== Access the private function throug child module
// // main function
// fn main() {
//     println!("Let's go inside the module");
//     outer_module::inner_module::my_public_function();
//   }
//   // declare a module
//   mod outer_module {
//     // function within outer module
//     fn my_private_function() {
//       println!("Hi, I got into the private function of outer module");
//     }
//     // declare a nested module
//     pub mod inner_module {
//       // function within nested module
//       pub fn my_public_function() {
//         println!("Hi, I got into the public function of inner module");
//         println!("I'll invoke private function of outer module");
//         super::my_private_function();
//       }
//     }
//   }

// //===== Access the Root Function
// // super can allow accessing a root function from within the module.
// // main function
// fn main() {
//     println!("Let's go inside the module");
//     my_module ::my_public_function();
//   }
//   fn my_function(){
//     println!("Hi, you came inside the root function using super");
//     }
  
//   // declare a module
//   mod my_module {
//     // function within outer module
//     pub fn my_public_function() {
//       println!("Invoke root function");
//       super::my_function();
//     }
//   }

/* Control Visibility within Different Files Using 'pub' */

/* Implicit Declaration#
A block of code put in a file without wrapping in a mod block implicitly declares a module.

Import the module:
    mod file_name
Call the module:
    file_name::x
Where x can be any construct within the module, i.e., function, array, trait, struct. */
//Example: 
//     === my_mod.rs

// pub fn my_public_function() {
//     println!("I am a public function in my_mod.rs");
// }

//     === main.rs
// mod my_mod; 
// fn main() {
//   println!("Invoke function in my_mod.rs");  
//   my_mod::my_public_function();
// }

/* Explicit Declaration#
The code in a file is wrapped within the mod block. This explicitly declares a module.

Import the module
    mod file_name
Call the module
    file_name::module_name::x
where x can be any construct within the module, i.e., function, array, trait, struct. */
//Example:
//     === my_mod.rs

// pub mod module{  //explicit declaration of the module to be public in original file
//     pub fn my_public_function() {
//         println!("I am a public function in my_mod.rs");
//     }
//     }

//     === main.rs
// mod my_mod; 
// fn main() {
//   println!("Invoke function in my_mod.rs");  
//   my_mod::my_public_function();
// }

/* === Nest Module */
// pub mod chapter {
//     pub mod lesson { // mod level 1
//         pub mod heading { // mod level 2
//             pub fn illustration() {  // mod level 3
//               println!("Hi, I'm a 3rd level nested function");
//             }
//         }
//     }
// }
// fn main() {
//     chapter::lesson::heading::illustration(); // call the function
// }

// === 'use' keyword: use to call the module in the precise way
// pub mod chapter {
//     pub mod lesson { // mod level 1
//         pub mod heading { // mod level 2
//             pub fn illustration() {  // mod level 3
//               println!("Hi, I'm a 3rd level nested function");
//             }
//         }
//     }
// }
// use chapter::lesson::heading; // make use of `use` keyword
// fn main() {
//     heading::illustration(); // call the function
// }

// // Glob Operator (*): helps u to prevent writing EnumName::variant when assigning enum values to the varaible
// // make this `enum` printable with `fmt::Debug`.
// #[derive(Debug)]
// enum KnightMove{
//    Horizontal,Vertical
// }

// use KnightMove::*; // use of globe operator
// fn main() {
//    // use enum
//    let horizontal_move = Horizontal; // Horizontal is shortcut for KnightMove::Horizontal
//    let vertical_move = Vertical; // Vertical is shortcut for KnightMove::Vertical
//    // print the enum values
//    println!("{:?}", horizontal_move);
//    println!("{:?}", vertical_move);
// }

/* ======= Challenge ======== */
/* 
+ Problem Statement:
The task is to call the root function my_area that calculates the area of a triangle.

- A method triangle_area() is defined within the module shapes.
- The task is to complete the method:
    - Implement root function.
    - Call the root function my_area from within the public function triangle_area().
    - Print the return value from the root function, i.e., area of the triangle
 */


mod shapes{
    pub fn triangle_area(base : i32, height : i32) {
        // call the root function 'my_area' and print the return value
        let result:f32 = super::my_area(base, height);
        print!("{}", result);
    }
  }

fn my_area(base:i32, height:i32) -> f32 {
    let b = base as f32;
    let h = height as f32;
    return ((b*h)/2.0) as f32;
}

fn main() {
    let b = 5;
    let h = 5;
    
    shapes::triangle_area(b, h);
}