// Welcome! Meet Karel, a robot that likes to explore the world. 
// It's world exists of [TODO: beschrijving van Karel's wereld: 2d, vierkant, muren, ...]
//
// Help Karel explore by making functions as described below at the "todo!();" places.
// A first function has been created as example under "first_steps".
//
// Please note:
// - Karel has only 7 possible actions that can be used in a function, as 
// mentioned in the "use karel ::{...}" section.
// - Per run, Karel can only execute one function. Find the place where this is
// set and change it to the function you want Karel to execute.

#![allow(unused)]

// These are the senses of Karel, i.e. they are either true or false.
use karel::{facing_north, on_crab, wall_ahead};

// These are the actions that Karel can do.
use karel::{pick_crab_up, put_crab_down, step, turn_clockwise};

// Helper functions you defined yourself.
use karel::helpers;

fn first_steps() {
    step();
    turn_clockwise();
    step();
    turn_clockwise();
    step();
    turn_clockwise();
    step();
    turn_clockwise();
}

fn turn_counter_clockwise() {
    todo!();
}

fn ahead_four_steps() {
    todo!();
}

fn make_line_of_four_crabs() {
    todo!();
}

fn main() {
    let world = karel::World::new(30, 50).fenced();

    let robot = karel::Robot {
        pos: (15, 24),
        in_hold: 1000,
        ..Default::default()
    };

    karel::run(world, robot, first_steps);
}
