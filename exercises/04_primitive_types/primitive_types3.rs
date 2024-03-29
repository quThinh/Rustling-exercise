// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let a = "The sun dipped below the horizon, casting hues of orange and pink across the sky. The gentle breeze rustled the leaves, creating a soothing melody. As the day bid farewell, a sense of tranquility enveloped the world, promising a peaceful night ahead.";

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
