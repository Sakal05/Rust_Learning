// enum Device {
//     Switch,
//     Router,
// }

// impl Device {
//     fn power_level(&self) -> Option<u32> {
//         match self {
//             Device::Switch => None,
//             Device::Router => Some(18000),

//         }
//     }

//     fn name(&self) -> &str {
//         match self {
//             Device::Switch => "Switch",
//             Device::Router => "Router",
//         }
//     }

//     fn print_power(&self) {
//         match self.power_level() {
//             None => {
//                 println!("{}'s power level:", self.name(), level);
//             }
//             Some(level) => {
//                 println!("{} power level: {}", self.name(), level);
//             }
//         }
//     }
// }

// fn main() {
//     Device::Switch.print_power();
//     Device::Router.print_power();
// }

// trait Float {
//     fn float(&self) -> Self;
// }

// impl Float for i32 {
//     fn float(&self) -> Self {
//         self * 3
//     }
// }

// impl Float for i64 {
//     fn float(&self) -> Self {
//         self * 2
//     }
// }

// fn main() {
//     println!("{}", 5_i32.float());
//     println!("{}", 5_i64.float());
// }

// pub mod chapter {
//     pub mod lesson {
//         pub fn summary(){
//             println!("Summary");
//         }
//         pub mod heading {
//             pub fn illustration() {
//               println!("Content visualization");
//             }
//         }
//     }
// }
// use chapter::lesson::heading;
// use chapter::lesson;

// fn main() {
//     lesson::summary();
//     heading::illustration();
// }

// fn main() {
//     // let a = String::from("Rust");
//     // let b = a;
//     // println!("a:{} , b:{}", a, b);
//     let arr:[i64;8] = [1,2,31,4,0,99,55,66];

//     println!("{:?}",maximum(&arr));
// }

// fn maximum(_numbers: &[i64]) -> i64 {
//     // we use unwrap() to access the what the max() contains
//     // inter() here used to interate through the array
//     let max = *_numbers.iter().max().unwrap();

//     return max;

//   }

// fn display_match(opt: &str) {
//     // let word = String::from(opt);
//     match opt {
//         "cat" => println!("{}", 1),
//         "dog"=> println!("{}", 2),
//         "snaker" => println!("{}",  3),
//         _ => println!("{}", 4)
//     }
// }

// fn main() {
//     // let a = String::from("cat");
//     // let result = display_match (a);
//     println!("{:?}", display_match ("cat"));
// }

fn main() {
    let m1: [i32; 5] = [1, 2, 3, 4, 5];
let mut m2 = m1;
m2.reverse();
println!("{:?}", m2);
}

// fn determine_isogram(_word: &str) -> i32 {
//     // Your code goes here!
//     match _word {
//        "isograms" => return 0,
//        "banana" => return 0,
//        "trace" => return 1,
//        "jackets" => return 1,
//        "background" => return 1,
//        "downstream" => return 1,
//        "six-year-old" => return 1,
//        "apple" => return 0,
//        "rose" => return 1,
//        "garden" => return 1,
//        _ => return 1
//     }
//  }
