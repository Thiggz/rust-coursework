

pub fn area_of(x: i32, y: i32) -> i32 { // All items in a library are private by default
    // Therefore, we add `pub` to make it public
    // Fixed area function
    x * y // Note that this is the idiomatic (normal way a Rust programmer does things)
    //       way to return a value from the function, we don't use `return` for Rust
}

pub fn volume(x: i32, y: i32, z: i32) -> i32 {
    // Added volume function
    x * y * z
}