pub fn inspect (s: &String){ // Note the ampersand before String
    // The ampersand before the type indicates a reference to the type
    // This means the function borrows the reference to the value

    // Here we check whether or not the string refers to something plural or singular
    if s.ends_with("s") {
        println!("Plural");
    } else {
        println!("Singular");
    }
}

pub fn change(s: &mut String) {
    if !s.ends_with("s") {
        // push_str appends a &str to the String (note that there is a difference between 
        // &str and String)
        s.push_str("s");
    }
}

pub fn eat(s: String) -> bool { // Note the return type is declared (bool)
    // eat checks if if the string contains "a" and "b"
    if s.starts_with("b") && s.contains("a") {
        // Note: It is important to provide a true and false return to ensure no errors
        true
    } else {
        false
    }
}

pub fn bedazzle(s: &mut String) {
    // Here, we dereference the mutable reference in order to assign a new value to it
    *s = String::from("sparkly");
}