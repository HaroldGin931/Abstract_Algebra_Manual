use std::ops::{Add, Neg};
// pub trait Group<T>:BinaryOperation<T>

pub trait AddOperation<T>
where
    T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy ,
{
    fn add(a: T, b: T) -> T {
        a + b
    }

    fn neg(a:T) -> T {
        -a
    }
}

pub trait Set<T> {
    fn new_set(elements: Vec<T>) -> Self;
    fn elements(&self) -> &Vec<T>;
    fn sample(&self, index: Option<usize>) -> Option<&T>;
    fn contains(&self, element: &T) -> bool;
}

pub trait Group<T>: Set<T>// + BinaryOperation<T>
where
    T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy,
{
    fn new_group(elements: Vec<T>, operation: fn(T, T) -> T) -> Self;

    fn has_identity(&self) -> T;

    fn get_operation(&self) -> fn(T, T) -> T;
} 

pub trait Field<T>: Group<T> 
where
    T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy,
{
    fn new_field(elements: Vec<T>, op_0: fn(T, T) -> T, op_1: fn(T, T) -> T) -> Self;

    fn get_sec_opeartion(&self) -> fn(T, T) -> T;
}