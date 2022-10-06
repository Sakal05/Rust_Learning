/* Arguments Pass by Reference */

// fn square(n: &mut i32) {
//     *n = *n * *n;
//     println!("The value of n inside function : {}", n);
// }

fn main() {
    // let mut n = 4;
    // println!("The value of n before function call : {}", n);
    // println!("Invoke Function");
    // square(&mut n);
    // println!("The value of n after function call : {}", n);

    // //return function testing
    // println!("This is what return from function: {}", return_func(100));

    /* multiple return value function testing */
    // let ret = mul_return(12, "Sakal");
    // println!("{}: {}", ret.0, ret.1);

    /* Array parameter */
    // let arr = [1, 2, 3, 4, 5];
    // calculate_mean(arr);

    print!("{}", test_divisibility_by_3_4(19));
}

/* Normal return function */
// fn return_func(i: i32) -> i32 {
//     return i + 100;
// }

/* Mulitple return function */
// fn mul_return(x: i32, s: &str) -> (i32, &str) {
//     let area = x * x;
//     let name = "Sakal";
//     (area, name)
// }

/* Array parameter */
// fn calculate_mean(arr: [i32; 5]) {
//     let mut sum = 0;
//     for i in 0..5 {
//         sum += arr[i];
//     }
//     println!("Mean of array values: {}", sum / 5);
// }

/* Challenge: check divisiblity by 3 and 4 */
// fn test_divisibility_by_3_4(a: i32) -> i32 {
//     // Write code here
//     if a % 3 == 0 && a % 4 == 0 {
//         return 0;
//     } else if a % 3 == 0 {
//         return 1;
//     } else if a % 4 == 0 {
//         return 2;
//     } else {
//         return -1;
//     }
// }