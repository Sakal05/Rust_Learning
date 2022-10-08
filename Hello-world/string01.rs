fn main() {
    // //creating empty string
    // let name = String::new();

    // let s_name = name.to_string();

    // println!("{}", s_name);
    // //the lenght of the string
    // println!("This is the length of string: {}", s_name.len());

    // let course = String::from("Rust");
    // println!("Programming language: {}", course);

    // /* str.capacity() The capacity gives the number of bytes allocated to the String, unlike len which gives the number of bytes taken by the String object. To get the capacity of a variable in bytes, use the built-in function capacity() */
    // println!("Capacity of the string: {}", course.capacity());

    // /* We can use str.contains("substring") function To find if one string contains another string, use the contains() built-in function. */
    // let fullname = String::from("Sakal Samanng");
    // let lastname = String::from("Sakal");

    // println!("{} is the substrin of {}: {}.", lastname, fullname, fullname.contains("Sakal"));

    /* Replacing a substring */
    /* The general syntax is :
    str.replace(replace_from, replace_to)
    Here str is the original string, replace_from is the value which is to be replaced in the string str and replace_to is the value the string is converted to. */

    // let str = String::from("Rust Programming Hello World");
    // let replace_to = "Language";
    // let result = str.replace("Programming", replace_to);
    // println!("{} is now becomming: {}", str, result);

    /* We can use trim to remove leading and ending whitespace */
    // let test_trim = "      Sakal Samnang       ";
    // let result = test_trim.trim();
    // println!("Result after trimming: {}", result);

    // /* We can use split() method in the for loop */
    // let str = String::from("Sakal Samnang");
    // for token in str.split(" ") {   //we can custom the splitting to ' , ' or any character
    //     println!("{}", token);
    // }

    // /* Interact through the String */
    // for c in str.chars(){
    //     print!("{} ", c);
    // }

    /* 
    - We can use push_str() method to concatinate/append the string
    - format!(str1, str2) method to merge two or more string together
    - we can use push() method to push single character
    */

    /* Challenge: The task requires you to find all words starting with a “c” in a string passed as a parameter. 
    Concatenate them together and return the result. */
    let s =
        "This is a comprehensive course in Rust programming language on the Educative. Read it with full concentration to grasp the content of the course".to_string();
    print!("{}", test(s));
}

fn test(my_str: String) -> String {
    // Write code here
    let mut con_word = "".to_string();

    for word in my_str.split(" ") {
        //let t_word = &word[0..2];
        let t_st = &word[..1];   
        // for ch in my_str.chars() {
            if t_st == "c"{ //we can use word.starts_with("c")
                con_word.push_str(word);
                con_word.push(' ');
            }
            
        // }
    }
    return con_word;
}