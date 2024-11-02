pub type BinaryOp<T> = fn(T, T) -> T;

pub trait Set<T> {
    fn new_set(elements: Vec<T>) -> Self;
    fn elements(&self) -> &Vec<T>;
    fn sample(&self, index: Option<usize>) -> Option<&T>;
    fn contains(&self, element: &T) -> bool;
}

pub trait Group<T>: Set<T>
where
    T: std::ops::Add<Output = T> + std::cmp::PartialEq + Copy,
{
    fn new_group(elements: Vec<T>, operation: BinaryOp<T>, identity: Option<T>) -> Self;
    fn identity(&self) -> T;
    fn get_operation(&self) -> BinaryOp<T>;
} 

pub trait Field<T>: Group<T> 
where
    T: std::ops::Add<Output = T> + std::cmp::PartialEq + Copy,
{
    fn new_field(elements: Vec<T>, op_0: BinaryOp<T>, identity_0: Option<T>, op_1: BinaryOp<T>, identity_1: Option<T>) -> Self;
    fn sec_identity(&self) -> T;
    fn get_sec_opeartion(&self) -> BinaryOp<T>;
}