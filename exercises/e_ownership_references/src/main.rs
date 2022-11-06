// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

use e_ownership_references::inspect;
use e_ownership_references::change;
use e_ownership_references::eat;
use e_ownership_references::bedazzle;

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });


    // This function takes a reference to the String the user has input in the command line
    inspect(&arg);

    // This function takes a *mutable* reference to the String and adds an "s" to the end of it
    change(&mut arg);
    println!("I have many {}", arg);

    // This function accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    // This function takes a mutable reference to a String, ignores what is in the string and 
    // replaces the contents of the string with the String "sparkly". 
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}
