
fn main() {

    // /* Placeholder */
    // //printing hello world
    // println!("Hello World!");

    // //positional placeholder perform likes C
    // println!("The value of opt is: {opt}", opt = 12);

    // //placeholder traits
    // println!("Number : 10 \nBinary:{:b} Hexadecimal:{:x} Octal:{:o}", 10, 10, 10);

    // //placeholdr for a Debug trait
    // println!("{:?}", ("This is a Rust Course", 101, "Sakal", 999));

    // /* Printing Type */
    // // Print!() use to print string onto a console
    // print!("Testing hello world");
    // print!("\n");
    // //println!() same as print!() just append new line at the end of the string
    // println!("Hello world");
    // println!("Hello world on new line");

    // //eprint!() displays the outputas an error
    // eprint!("This is an error");

    /* variable declaration */
    let x = 12;
    println!("The number is {}", x);
    
    //to let varible mutable we must use "mut" before variable name
    let mut mut_x = 12;
    println!("The number that mut is {}", mut_x);
    mut_x = 99;
    println!("The number after mutable is {}", mut_x);

    //assigne multiple variable 
    let (id, name) = (1, "Sakal");
    println!("{}. {}", id, name);
}



