// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        // We want to use the command line argument to call a function, we'll use if/else
        //
        // - If arg is "sum", then call the sum() function
        // - If arg is "double", then call the double() function
        // - If arg is anything else, then call the count() function, passing "arg" to it.

        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }

        // 1b. Now try passing "sum", "double" and "bananas" to the program by adding your argument
        // after "cargo run".  For example "cargo run sum"
    }
}

fn sum() {
    let mut sum:i32 = 0;

    // We will iterate through integers from 7 to 23 *inclusive* and add them together.
    // We use a for loop, and the first number is inclusive, the second number is exclusive
    // THerefore we need to go one number up
    for num in 7..24 {
        sum += num;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count:i32 = 0;
    let mut x = 1;

    // We will use the while loop to count the number we can multiply x by 2 until we get 
    // a value that is more than 500, then we will print the number of times possible

    while x <= 500 {
        x *= 2;
        count += 1;
    }

    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    // We'll use an unconditional loop (`loop`) to print `arg` 8 times, and then break

    let mut iter = 0; // Note that iter must be mutable in order for you to add to it every time
    loop {
        print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.
        iter += 1;
        if iter == 8 {
            break; // break out of the loop when the condition is reached
        }
    }

    println!(); // This will output just a newline at the end for cleanliness.
}