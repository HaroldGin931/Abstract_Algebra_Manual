use std::{io, process::exit};
mod modules;

use modules::algebra_structs::*;
use crate::modules::algebra_trait::*;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn add_with_modulator_7(x: i32, y: i32) -> i32 {
    (x + y) % 7
}

fn multiply_with_modulator_7(x: i32, y: i32) -> i32 {
    (x * y) % 7
}

fn isbigger(x: i32, y: i32) -> i32 {
    if x > y {
        1
    } else {
        0
    }
}

fn main() {
    let v = vec![0, 1, 2, 3, 4, 5, 6];
    let v_no_zero = vec![1, 2, 3, 4, 5, 6];
    println!("Welcome to the algebraic group checker!\n\
             This program will check if the given set and operation satisfy the group axioms\n\
             You can combine the set and the operation which provided below\n\
             to check if they satisfy the group axioms\n");
    loop{
        println!("Please pick one of the sets below:"); 
        println!("1. Z_7 = {{0, 1, 2, 3, 4, 5, 6}}");
        println!("2. Z_7_no_zero = {{1, 2, 3, 4, 5, 6}}");

        // Get set selection
        let set_selection: i32 = match io::stdin()
            .lines()
            .next()
            .and_then(|l| l.ok())
            .and_then(|l| l.parse().ok()) {
            Some(num @ 1..=2) => num,
            _ => {
                println!("Invalid set selection");
                continue;
            }
        };

        println!("\nPlease pick one of the binary operations below:");
        println!("1. integer add, a + b ");
        println!("2. integer add with modulator_7, (a + b) mod 7");
        println!("3. integer multiply, a * b");
        println!("4. integer multiply with modulator_7, (a * b) mod 7");
        println!("5. integer isbigger (a > b) ");

        // Get operation selection
        let op_selection: i32 = match io::stdin()
            .lines()
            .next()
            .and_then(|l| l.ok())
            .and_then(|l| l.parse().ok()) {
            Some(num @ 1..=5) => num,
            _ => {
                println!("Invalid operation selection");
                continue;
            }
        };

        // Select the appropriate set
        let set = if set_selection == 1 { &v } else { &v_no_zero };

        // Select the appropriate operation
        let operation = match op_selection {
            1 => add,
            2 => add_with_modulator_7,
            3 => multiply,
            4 => multiply_with_modulator_7,
            5 => isbigger,
            _ => unreachable!()
        };

        // Create and check group
        match Group::new(set.clone(), operation) {
            Some(g) => {
            println!("Group axioms are satisfied");
            println!("The group is: {:?}", g.elements());
            exit(0);
            }
            None => println!("Can not fit the group axioms, please try again")
        }
    }
}