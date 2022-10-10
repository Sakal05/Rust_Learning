/* ======= Trait ======= */
//Traits are used to define a standard set of behavior for multiple structs.

/* There can be two types of methods in traits

- Concrete Method: The method that has a body meaning that implementation of the method is done within the method.

- Abstract Method: The method that does not have a body meaning that implementation of the method is done by each struct in its own impl construct. */

// fn main() {
//     //create an instance of the structure
//     let c = Circle {
//         radius: 2.0,
//     };
//     let r = Rectangle {
//         width: 2.0,
//         height: 2.0,
//     };
//     println!("Area of Circle: {}", c.shape_area());
//     println!("Area of Rectangle:{}", r.shape_area());
// }
// //declare a structure
// struct Circle {
//     radius: f32,
// }
// struct Rectangle {
//     width: f32,
//     height: f32,
// }
// //declare a trait
// trait Area {
//     fn shape_area(&self) -> f32;
// }
// //implement the trait
// impl Area for Circle {
//     fn shape_area(&self) -> f32 {
//         3.13 * self.radius * self.radius
//     }
// }
// impl Area for Rectangle {
//     fn shape_area(&self) -> f32 {
//         self.width * self.height
//     }
// }

/* ====== Generics ====== */

//To put it simply generic is same as template in c++, we can use different datatype for the paramemter or argument
/* What Are Generics?#
Generics are a way of generalizing types; they define the data type at run time. Generics are called parametric polymorphism in type theory. ‘Poly’ is multiple, ‘morph’ is form over a given parameter (‘parametric’) meaning multiple forms of a given parameter.

They can be applied to methods, functions, structures, enumerations, collections, and traits. This helps to reuse the same code but with a different type. */

// fn main(){
//     println!("- Passing a string literal");
//     concatenate(" Rust ", " Programming ");
//     println!("- Passing an integer");
//     concatenate(10 as i32, 1 as i32);

//  }
//  use std::fmt::Display;
//  fn concatenate<T:Display>(t:T, s:T){
//     let result = format!("{}{}", t , s);
//     println!("{}", result);
//  }

// use std::fmt::Display;
// fn print_vec<T:Display>(v: &[T]) {
//   for i in v.iter() {
//       print!("{}", i)
//   }
//   println!("");
// }

// fn main() {
//   let int_vec = [1, 2, 3, 4, 5]; // define a vector of type integer

//   println!("Call to the function with vector of integers");

//   print_vec(& int_vec); // pass vector of type integer to the function

//   println!("Call to the function with vector of strings");

//   let str_vec = ["Rust", "Programming"]; // define a vector of type string

//   print_vec(&str_vec); // pass vector of type String to the function
// }

#![allow(dead_code)]
//declare a structure
struct Car {
    owner_age: i32,
}
struct Motorbike {
    owner_age: i32,
}
//declare a trait
trait Drive {
    fn can_drive(&self) -> i32;
}
//implement the trait
impl Drive for Car {
    fn can_drive(&self) -> i32 {
        if self.owner_age < 18 {
            return 0;
        } else {
            return 1;
        }
    }
}
impl Drive for Motorbike {
    fn can_drive(&self) -> i32 {
        if self.owner_age < 14 {
            return 0;
        }
        else {
            return 1;
        }
    }
}

fn main() {
    let c1 = Car { owner_age: 18 };
    let m1 = Motorbike { owner_age: 15};
    println!("{}", c1.can_drive());
    println!("{}", m1.can_drive());
}