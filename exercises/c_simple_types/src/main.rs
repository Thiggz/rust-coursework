// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

// We bring the functions from lib.rs into the scope here. Note that the package name is defined
// in Cargo.toml at: [package], name = "c_simple_types"
use c_simple_types::print_difference;
use c_simple_types::print_array;
use c_simple_types::ding;
use c_simple_types::on_off;
use c_simple_types::print_distance;

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);

    print_difference(coords.0, coords.1);   // Tuple indexing is used to pass parts of the coords
    // into the print_difference function (defined below)

    // Suppose we want to use an array function `print_array` to print coords:
    // Note: The type annotation for array always uses the semicolon!! see below
    // Also note that arrays are limited to size of 32
    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);


    let series = [1, 1, 2, 3, 5, 8, 13];
 
    // Array indexing uses the square brackets with the index (that starts at 0). 
    // Here we pass the value 13 from the array 'series' by using its index 6, to trigger the ding
    ding(series[6]);


    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");

    // objective: Pass the on_off function the value true from the variable `mess`
    // mess is a tuple, true lies in the 3rd element (index 2), in the 2nd element of the array
    // (index [1]), in the 1st element of the tuple, (index 0)
    on_off(mess.2[1].0);

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.

    print_distance(coords);
}