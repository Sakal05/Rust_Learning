/* Method in vector 
- Vec::new()  -> creates new vector
- .push()   -> push a value
- .pop()    -> pop a value
- .contain()    -> returns true if the vector contains a particular value
- .remove(i)    -> remove a value at given index
- .len()    -> return the length of the vector
*/

fn main() {
    //     //define a vector of size 4
    //     let mut my_vec = vec![1, 2, 3, 4, 5];
    //     //access a particular value
    //     println!("{}", my_vec[0]);

    //     for i in &my_vec {      //we use & to avoid moving into the for loop
    //         print!("{} ", i);
    //     }

    //     my_vec.push(6);
    //     println!("After adding 6 to the end of the vector {:?}", my_vec);
    //     my_vec.remove(2);
    //     println!("After removing value in index 2 which is number 3 from vector: {:?}", my_vec);

    //     match my_vec.get(9) {   //using get() method to access value at given index
    //         Some(x) => println!("Value at given index:{}", x),
    //         None => println!("Sorry, you are accessing a value out of bound")
    //  }

    /* ========== Interating Over a Vector ========== */

    // // defines a mutable vector
    // let mut my_vec = vec![1, 2, 3, 4, 5];
    // // define the value to be removed
    // let value = 2;
    // // get the index of the value in the vector
    // let index = my_vec
    //     .iter() // .iter() is the built-in function that iterates over the elements of the vector.
    //     .position(|&r| r == value)
    //     .unwrap();
    // // call the built-in remove method
    // my_vec.remove(index);
    // // print the updated vector
    // println!("Updated Vector: {:?}", my_vec);

    // for i in my_vec.iter() {    //work without using iter()
    //     println!("element of the vector: {}", i);
    // }

    // //loop and mutate value
    // for i in my_vec.iter_mut() {
    //     *i *= 2;
    //     println!("element of the vector: {}", i);
    // }

    /* For challenge vector */
    let mut v1 = vec![1, 5, 7, 9];
    print!("{:?}", test(&mut v1));
}

/* Given a vector with an even number of elements, remove the last element from the 
    input vector, and then the middle element. Then insert the sum of the remaining 
    elements to the end of the resulting vector. */

fn test(my_vec: &mut Vec<u32>) -> &mut Vec<u32> {
    // Write code here!
    
    let middle = (my_vec.len())/2;
    
    //remove last element
    my_vec.pop();
    //removing middle element
    my_vec.remove(middle-1);

    //calculat the total of the remaining elements
    let mut total:u32 = 0;
    for s in my_vec.iter() {
        total += s;
    }

    //push the total into the end of the vector

    my_vec.push(total);

    return my_vec;
}