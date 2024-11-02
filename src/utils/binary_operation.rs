pub trait BinaryOperation<T>
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
