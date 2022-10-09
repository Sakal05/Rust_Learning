// //declare a struct
// struct Course {
//     code:i32,
//     name:String,
//     level:String,
//  }

//  fn main() {
//     //initialize
//     let mut course1 = Course  {
//        name:String::from("Rust"),
//        level:String::from("beginner"),
//        code:130,
//     };
//     let course2 = Course  {
//        name:String::from("Javascript"),
//        level:String::from("beginner"),
//        code:122,
//     };
//     //access
//     println!("Name:{}, Level:{}, code: {}", course1.name, course1.level, course1.code);
//     println!("Name:{}, Level:{}, code: {}", course2.name, course2.level, course2.code);
//     //update
//     course1.name = "Java".to_string();
//     course1.code = 999;
//     course1.level = "Pro_advance".to_string();
//     println!("Name:{}, Level:{} ,code: {}", course1.name, course1.level, course1.code);
//  }

/* ==========Function and struct========== */
// //declare a struct
// struct Course {
//     code: i32,
//     name: String,
//     level: String,
// }
// fn return_rust_course_info(c1: Course, c2: Course) -> Course {
//     println!("I got into function and return values from there");
//     if c1.name == "Rust" {
//         return c1;
//     } else {
//         return c2;
//     }
// }

// fn main() {
//     //initialize
//     let course1 = Course {
//         name: String::from("Rust"),
//         level: String::from("beginner"),
//         code: 233,
//     };
//     let course2 = Course {
//         name: String::from("Java"),
//         level: String::from("beginner"),
//         code: 130,
//     };

//     let choose_course = return_rust_course_info(course1, course2);
//     println!(
//         "I choose to learn {} {} course with code:{}",
//         choose_course.name,
//         choose_course.level,
//         choose_course.code
//     );
// }

// /* ==========Method of struct========== */
// //declare a struct
// struct Course {
//     name: String,
//     level: String,
//     code:i32
// }
// //impl construct to define struct methods
// impl Course {
//     fn name_code(&self) -> String {
//         format!("{} {}", self.name, self.code)
//     }
// }

// fn main() {
//     let course_1 = Course {
//         name: "Rust".to_string(),
//         level:"beginner".to_string(),
//         code:132
//     };
//     //call the non-static method
//     println!("This is a {} course: {}", course_1.level, course_1.name_code());
// }

// /* ==========Static Method of Structs========== */
// // declare a struct
// struct Course {
//     name: String,
//     level:String,
//     code: i32,
//  }
//  impl Course {
//     // static method
//     fn my_static_method(n: String, l: String, c:i32) -> Course {
//        Course {
//        name: n,
//        level:l,
//        code:c
//         }
//     }
//     //display
//     fn display(&self){
//        println!("name :{} code:{} of type: {}", self.name, self.code, self.level );
//     }
//  }

//  fn main(){
//     // call the static method
//     let c1 = Course::my_static_method("Rust".to_string(), "beginner".to_string(), 132);
//     c1.display();
//  }

// /* ==========Tuple Structs========== */
// //define a tuple struct
// struct FruitQuantity(String, i32);
// // main function
// fn main() {
//     // create an instance
//     let r1 = FruitQuantity("oranges".to_string(), 12);
//     // access values of a tuple struct
//     println!("r1--name:{} quantity:{}", r1.0, r1.1);
//     // create an instance
//     let r2 = FruitQuantity("mangoes".to_string(), 13);
//     // access values of a tuple struct
//     println!("r2--name:{} quantity:{}", r2.0, r2.1);
// }

/* Struct Challenge */
/* Problem Statement#
- A struct Point is given which has two items, x and y.

- The function test is given which has two instances of points initialized with some value of x and y.

- The task is to calculate the distance between the two points. */

// //solution 1
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn test(_point1: Point, _point2: Point) -> f32 {
//     // Write code here!
//     // let x_point: f32 = (_point1.x - _point2.x) as f32;
//     // let y_point: f32 = (_point1.y - _point2.y) as f32;
//     let x_y: f32 = ((_point1.x - _point2.x) + (_point1.y - _point2.y))  as f32;
//     //power by 2
//     let xy_sq = f32::powf(2.0, x_y);
//     //let y_sq = f32::powf(2.0, y_point);

//     let result = xy_sq.sqrt();
//     return result;
// }

// fn main() {
//     let p1 = Point {
//         x: 2,
//         y: 3,
//     };
//     let p2 = Point {
//         x: 4,
//         y: 3,
//     };
//     println!("Result: {}", test(p1, p2));
// }

// //solution 2
// #[derive(Debug)] // prints the value of struct using the debug trait
// struct Point {
// 	x: i32,
// 	y: i32
// }
// fn test(point1: Point, point2: Point)-> f32 {
//     let distance = i32::pow(point1.x - point2.x,2) + i32::pow(point1.y - point2.y,2);
//     let d = distance as f32;
//     d.sqrt()
// }
// fn main(){
//     let point1 = Point { x: 2, y: 3 };
//     let point2 = Point { x: 4, y: 3 }; 
//     println!("point1:{:?}", point1); 
//     println!("point2:{:?}", point2); 
//     print!("Distance between two points:");
//     print!("{}", test(point1, point2));
// }
