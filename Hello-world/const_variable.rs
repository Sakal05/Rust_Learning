/*
Naming Convention: By convention, you write a constant variable name in a SCREAMING_SNAKE_CASE, i.e.,
- All letters should be UPPER case.
- All words should be separated using an underscore ( _ ).

const variable cannot be mutable unlike let which can be made mutable using mut keyword.

with const, we must declare datatype unlike the let, and it couldn't be mutable
*/

fn main()
{
    const X:&str = "Sakal";
    println!("{}", X);
}
