// trait MyT<T>: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {

// }

// pub trait BinaryOperation<T>
// where
//     T: MyT<T>,
// {
//     fn add(a: T, b: T) -> T {
//         a + b
//     }
// }

// pub trait X<T>:BinaryOperation<T>
// where
//     T: MyT<T>,{

// }

pub trait BinaryOperation<T>
where
    T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy ,
{
    fn add(a: T, b: T) -> T {
        a + b
    }
}
