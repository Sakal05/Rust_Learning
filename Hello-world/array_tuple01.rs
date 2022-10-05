fn main() {

    // /* =============== Array =============== */
    // //define an array of size 4 with datatype of integer 32bit type
    // let arr:[i32;4] = [1, 2, 3, 4]; 
    // //print the first element of array
    // println!("The first value of array is {}", arr[0]);
    // // initialize an array of size 4 with 0
    // let arr1 = [0; 4]; 
    // //print the first element of array
    // println!("The first value of array is {}", arr1[0]);

    // //printing the array using debug trait
    // println!("List of array element: {:?}", arr1);

    // //lenght of the array 
    // println!("Lenght of array is: {}", arr1.len());

    // //slice the array 
    // let slice_arr:&[i32] = &arr[0..2];
    // println!("Slice array element: {:?}", slice_arr);


    /* =============== Tuple =============== */

    let person: (&str, i32, char, i32) = ("Sakal", 19, 'M', 2003);
    // defining each element value with names
    let (name, age, gender, birthyear) = person;
    println!("===== Different way of printing data \n =====" );
    //access the whole element of the tuple
    println!("Person Data: {:?}", person);
    //access each element of tuple by their name declared
    println!("Name: {}\nAge: {}\nGender: {}\nBirth Year: {}", name, age, gender, birthyear,);
    //access each element of tuple by index
    println!("Name: {}\nAge: {}\nGender: {}\nBirth Year: {}", person.0, person.1, person.2, person.3);

    //mutable tuple
    let mut person1: (&str, i32, char, i32) = ("Sakal", 19, 'M', 2003);
    person1.0 = "Samanng";
    println!("After modification of the tuple the name become '{}'", person1.0);
    
 }