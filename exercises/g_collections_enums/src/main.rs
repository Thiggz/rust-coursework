// Silence some warnings that could distract from the exercise
#![allow(unused_variables, unused_mut, dead_code)]

// Someone is shooting arrows at a target.  We need to classify the shots.
// We create an enum "Shot" with the following variants:
// Bullseye, Hit <distance from the center of the target - f64>, Miss
enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot {
    // Method for the shot enum
    fn points(self) -> i32 {
        // The following will return points depending on certain met conditions
        match self {
            // returns 5 points if `self` is a `Shot::Bullseye`
            Shot::Bullseye => 5,
            // - return 2 points if `self` is a `Shot::Hit(x)` where x < 3.0
            // It's much neater to use a guard here:
            Shot::Hit(x) if x < 3.0 => 2,
            // returns 1 point if `self` is a `Shot::Hit(x) where x >= 3.0
            Shot::Hit(x) => 1,
            // - return 0 points if `self` is a Miss
            Shot::Miss => 0,
        }
    }
}

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    // For each coord in arrow_coords:
    //
    //   A. Call `coord.print_description()`
    //   B. Append the correct variant of `Shot` to the `shots` vector depending on the value of
    //   `coord.distance_from_center()`
    //      - Less than 1.0 -- `Shot::Bullseye`
    //      - Between 1.0 and 5.0 -- `Shot::Hit(value)`
    //      - Greater than 5.0 -- `Shot::Miss`
    for coord in arrow_coords {
        coord.print_description();
        let shot = match coord.distance_from_center() {
            // Less than 1.0 -- `Shot::Bullseye`
            x if x < 1.0 => Shot::Bullseye,
            // Between 1.0 and 5.0 -- `Shot::Hit(value)`
            x if x < 5.0 => Shot::Hit(x),
            // All other values, i.e. greater than 5.0: `Shot::Miss`
            _ => Shot:: Miss,
        };
        // we take our shots vector and push it onto it
        shots.push(shot)
    }

    let mut total = 0;
    // We finally loop through each shot in shots and add its points to total
    for shot in shots {
        // Look up for the method created earlier, for points 
        total += shot.points();
    }
    println!("Final point total is: {}", total);
}

// A coordinate of where an Arrow hit
#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y);
    }

}

// Generate some random coordinates
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}