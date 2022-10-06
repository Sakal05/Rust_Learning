fn main() {
    // let mut a = 1;
    // let mut b = 2;
    // a = a & b;
    // a = a << 1;
    // b = b >> 3;
    // println!("a: {}", a);
    // println!("b: {}", b);

    // //using "as" to convert the datatype
    // let a = 15;
    // let b = (a as f64) / 2.0;
    // println!("a: {}", a);
    // println!("b: {}", b);

    // /* ===== Borrwoing and dereferencing operators ===== */
    // let x = 10;
    // let mut y = 13;
    // //immutable reference to a variable
    // let a = &x;
    // println!("Value of a:{}", a);
    // println!("Value of x:{}", x); // x value remains the same since it is immutably borrowed
    // //mutable reference to a variable
    // let b = &mut y;
    // println!("Value of b:{}", b);

    // *b = 11; // derefencing. point to the value of mutable borrow variable which is y, and update the value of y to 11
    // println!("Value of b:{}", b); // updated value of b
    // println!("Value of y:{}", y); // y value can be changed as it is mutuably borrowed. will print out the value of 11 instead of 13

    /* 
    Operators are listed below in the order of their precedence from highest to lowest :

    Unary:
    Logical/Bitwise NOT - !
    Derereference - *
    Borrow - &, &mut

    Binary:
    Typecast - as
    Multiplication- *,Division - /, Remainder - %
    Addition -+, Subtraction - -
    Left Shift - <<, Right Shift - >>
    Bitwise AND - &
    Bitwise XOR - ^
    Bitwise OR - |
    Comparison - == != < > <= >=
    Logical AND - &&
    Logical OR - ||
    Range - start .. stop
    Assignment/Compound Assignment - = += -= *= /= %= &= |= ^= <<= >>=
    */

    
    let _a = 2;
    let _b = 2;
    // Write code here!
    let result = i32::pow(_a,3) + i32::pow(_b, 3) + 3*_a*_b*(_a+_b);
    print!("{}", result);
}