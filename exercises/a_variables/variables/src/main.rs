const STARTING_MISSILES: i32 = 8; // We can declare constants outside the main() function's module scope
const READY_AMOUNT: i32 = 2; 
fn main() {
    let mut missiles: i32 = STARTING_MISSILES; // The missiles variable has to be mutable in order to update it's value
    let ready: i32 = READY_AMOUNT;

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;

    println!("{} missiles left", missiles);

    let (missiles_left, ready_new) = (missiles, READY_AMOUNT); // We can also bind both variables, missiles and ready in one line

    println!("Firing {} of my {} missiles...", ready_new, missiles_left);

    println!("{} missiles left", missiles_left - ready_new); // And finially, we can perform the subrtaction within the print statement
    // Note however that performing the above will result in the warning: variable does not need to be mutable
}
