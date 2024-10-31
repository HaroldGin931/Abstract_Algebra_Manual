use crate::utils::set::Set;
use crate::utils::binary_operation::BinaryOperation;

use std::ops::{Add, Neg};
// pub trait Group<T>:BinaryOperation<T>


pub trait Group<T>: Set<T>// + BinaryOperation<T>
where
    T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy,
{
    fn new_group(elements: Vec<T>, operation: fn(T, T) -> T) -> Self;

    fn get_identity(&self) -> T;

    // fn get_set(&self) -> &dyn Set<T>;
    fn get_operation(&self) -> fn(T, T) -> T;
} 
