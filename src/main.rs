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
    let v_no_zero = vec![1, 2, 3, 4, 5, 6, 7];
    println!("Welcome to the algebraic group checker!\n\
             This program will check if the given set and operation satisfy\n\
             the group axioms\n\
             You can combine the set and the operation which provided below\n\
             to check if they satisfy the group axioms\n");
    loop{
        println!("Please pick one of the sets below:"); 
        println!("1. Z_7 = {{0, 1, 2, 3, 4, 5, 6}}");
        println!("2. Z_7_no_zero = {{1, 2, 3, 4, 5, 6, 7}}");
        let group: Option<Group<i32>>;
        let mut selections = (0, 0);
        let mut set_selection = String::new();
        let mut op_selection = String::new();
        io::stdin().read_line(&mut set_selection).expect("Failed to read line");
        let set_selection: i32 = set_selection.trim().parse().expect("Please type a number!");
        match set_selection {
            1 => {
                selections.0 = 1;
            }
            2 => {
                selections.0 = 2;
            }
            _ => {
                println!("Invalid selection");
                return;
            }
            
        }
        println!("1. integer add");
        println!("2. integer add with modulator_7");
        println!("3. integer multiply");
        println!("4. integer multiply with modulator_7");
        println!("5. integer isbigger");
        io::stdin().read_line(&mut op_selection).expect("Failed to read line");
        let op_selection: i32 = op_selection.trim().parse().expect("Please type a number!");
        match op_selection {
            1 => {
                selections.1 = 1;
            }
            2 => {
                selections.1 = 2;
            }
            3 => {
                selections.1 = 3;
            }
            4 => {
                selections.1 = 4;
            }
            5 => {
                selections.1 = 5;
            }
            _ => {
                println!("Invalid selection");
                return;
            }
        }

        match selections {
            (1, 1) => {
                group = Group::new(v, add);
            }
            (1, 2) => {
                group = Group::new(v, add_with_modulator_7);
            }
            (1, 3) => {
                group = Group::new(v, multiply);
            }
            (1, 4) => {
                group = Group::new(v, multiply_with_modulator_7);
            }
            (1, 5) => {
                group = Group::new(v, isbigger);
            }
            (2, 1) => {
                group = Group::new(v_no_zero, add);
            }
            (2, 2) => {
                group = Group::new(v_no_zero, add_with_modulator_7);
            }
            (2, 3) => {
                group = Group::new(v_no_zero, multiply);
            }
            (2, 4) => {
                group = Group::new(v_no_zero, multiply_with_modulator_7);
            }
            (2, 5) => {
                group = Group::new(v_no_zero, isbigger);
            }
            _ => {
                println!("Invalid selection");
                return;
            }
        };
        match group {
            Some(g) => {
                println!("Group axioms are satisfied");
                println!("The group is: {:?}", g.elements());
                exit( 0);
            }
            None => {
                println!("Can not fit the group axioms, please try again");
                return;
            }
        }
    }
}
