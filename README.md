Karel in Rust
-------------
Framework for building well-known Karel the Robot exercises, this time in Rust.


## What is Karel the Robot

Karel the Robot is series of exercises designed to learn programming basics in a simple 
environment. Originally designed to learn Python, as can be read in the 
[original Karel the Robot description][karel-stanford], they are redesigned in Rust.   


## Goal

These exercises give an introduction to the programming concept of control flow. 
They can be used for absolute beginners in learning programming concepts with language Rust. 


## Copyright & License

Please read the [LICENSE][license] document for details about copyright and license.


## Student guide

Karel the Robot will be used as part of a mentoring program to learn Rust, offered by
Tweede Golf and the Dutch Electoral Council. A student guide to the program is being 
developed and will be linked here. 


## Getting started

- Install Rust as described in the [Install Rust guide][install-rust].
- Fork this repository.
- Run an exercise by executing the command "cargo run --bin" followed by the exercise you want
to execute, e.g. 0-welcome.
- The exercises can be found in src/bin.
- Change the content of the exercise files to change Karel's behaviour in the graphic environment.


## Rules for running Karel

- Help Karel explore by making functions as described at the "todo!();" places in the exercises.
- Karel has only 7 possible actions to take, as mentioned in the "use karel ::{...}" section
of the exercises.
- Per run, Karel can only execute one function. Find the place where this is set in the exercise
 and change it to the function you want Karel to execute.


[karel-stanford]: https://compedu.stanford.edu/karel-reader/docs/python/en/chapter1.html
[license]: https://github.com/tweedegolf/karel/blob/main/LICENSE
[install-rust]: https://rust-lang.org/tools/install/
