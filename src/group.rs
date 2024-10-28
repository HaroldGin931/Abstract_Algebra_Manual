use crate::utils::set::Set;
use crate::utils::binary_operation::BinaryOperation;

use std::ops::{Add, Neg};
// pub trait Group<T>:BinaryOperation<T>


pub trait Group<T>
where
    T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy,
{
    fn get_identity(&self) -> T;

    fn get_set(&self) -> &Set<T>;
    fn get_operation(&self) -> fn(T, T) -> T;

    fn sample_element(&self, index: Option<usize>) -> Option<&T> {
        // self.set.elements().get(index.unwrap_or(0))
        // can not assume the struct has a set field
        let set = self.get_set();
        set.sample(index)
    }
} 
// Group<T> {
//     set: Set,
//     operation: fn(T, T) -> T,
//     identity: T
// }

