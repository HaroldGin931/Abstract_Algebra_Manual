// pub struct Field {
//     // 分配律意味着我有一个方法能够批量操作整个集合元素的能力？
//     // eg a x (b0 + b1 + b2 + b3) = a x b0 + a x b1 + a x b2 + a x b3
// }
use crate::group::Group;

pub trait Field<T>: Group<T> 
where
    T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy,{
        fn get_sec_opeartion(&self) -> fn(T, T) -> T;
}