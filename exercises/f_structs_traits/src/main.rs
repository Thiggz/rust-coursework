// Here we define a single trait named `Bite`, and we define a single a single required method, 'bite'
// This method will be called when we want to 'bite' somethign
trait Bite {
    fn bite(self: &mut Self); 
}


// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//
//  // include this line right before your struct definition
// struct Grapes...

// Here we create a struct named Grapees with a field 'leftovers' that tracks how many grapes are left
#[derive(Debug)] // #[derive(Debug)] asks the compiler to auto-generate a suitable implementation of 
// the Debug trait
struct Grapes{
    leftovers: u16,
}

// We implement Bite for grapes and subtract 1 from the leftovers
impl Bite for Grapes {
    fn bite(self: &mut Self){
        self.leftovers -= 1;
    }
}

fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    let mut grapes = Grapes { leftovers: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}