pub mod rust_fundamentals {

    pub mod exercise_a_variables {
        pub fn run_exercise_a_variables() {
            const STARTING_MISSILES: i32 = 8;
            const READY_AMOUNT: i32 = 2;

            let mut missiles = STARTING_MISSILES;
            let ready = READY_AMOUNT;

            println!("Firing {}, of my {} missiles...", ready, missiles);

            missiles -= ready;

            println!("{} missiles left", missiles);
        }
    }

    pub mod functions {
        // pub fn run_functions() {}

        // fn do_stuff(qty: f64, oz: f64) -> f64 {
        //     return qty * oz;
        // }
    }

    pub mod exercise_b_functions {
        pub fn run_functions() {
            let width = 4;
            let height = 7;
            let depth = 10;
            // 1. Try running this code with `cargo run` and take a look at the error.
            //
            // See if you can fix the error. It is right around here, somewhere.  If you succeed, then
            // doing `cargo run` should succeed and print something out.

            let area = area_of(width, height);

            println!("Area is {}", area);

            // 2. The area that was calculated is not correct! Go fix the area_of() function below, then run
            //    the code again and make sure it worked (you should get an area of 28).

            // 3. Uncomment the line below.  It doesn't work yet because the `volume` function doesn't exist.
            //    Create the `volume` function!  It should:
            //    - Take three arguments of type i32
            //    - Multiply the three arguments together
            //    - Return the result (which should be 280 when you run the program).
            //
            // If you get stuck, remember that this is *very* similar to what `area_of` does.
            //
            println!("Volume is {}", volume(width, height, depth));
        }

        fn volume(width: i32, height: i32, depth: usize) -> i32 {
            return width * height * depth as i32;
        }

        fn area_of(x: i32, y: i32) -> i32 {
            // 2a. Fix this function to correctly compute the area of a rectangle given
            // dimensions x and y by multiplying x and y and returning the result.
            //
            x * y
            // Challenge: It isn't idiomatic (the normal way a Rust programmer would do things) to use
            //            `return` on the last line of a function. Change the last line to be a
            //            "tail expression" that returns a value without using `return`.
            //            Hint: `cargo clippy` will warn you about this exact thing.
        }
    }
}

pub mod primitives {
    pub mod compound_types {
        pub fn run_tuples() {
            let info = (1, 3.3, 999);
            let jets = info.0;
            let fuel = info.1;
            let ammo = info.2;

            let (jets, fuel, ammo) = info;
        }

        pub fn run_arrays() {
            // arrays are of fixed length and limited to a size of 32
            // stack based.
            let buf: [u8; 3] = [1, 2, 3];
        }
    }

    
}

// use rust_fundamentals::exercise_a_variables::*;
// use design_patterns::greet;
use design_patterns::exercise_c::*;
// use rust_fundamentals::exercise_b_functions::*;

fn main() {
    run_exercise_c();
    // run_exercise_a_variables();
    // run_functions();
    // greet();
}
