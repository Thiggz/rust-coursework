// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

use b_functions::area_of; // brings an item from a path to the scope. 
// Note the format, b_functions being the name of the project, :: is the scope operator
// area_of is the function's name
use b_functions::volume;
// Note: this is how we call things from the standard library, look for crates.io for packages
// Add package to Cargo.toml either by absolute path, or bringing item into scope with use statement

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    /* Old: Error identified: area was declared outside of scope
    {
        let area = area_of(width, height);
    }
    */

    let area = area_of(width, height);
    println!("Area is {}", area);
    
    // Added volume function (below)
    println!("Volume is {}", volume(width, height, depth));
}

/* I have moved these to lib.rs

fn area_of(x: i32, y: i32) -> i32 {
    // Fixed area function
    x * y // Note that this is the idiomatic (normal way a Rust programmer does things)
    //       way to return a value from the function, we don't use `return` for Rust
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    // Added volume function
    x * y * z
}

*/