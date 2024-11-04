pub type BinaryOp<T> = fn(T, T) -> T;

pub trait AlgebraicElement: std::fmt::Debug + PartialEq + Copy {}

pub trait Set<T: AlgebraicElement> {
    fn elements(&self) -> &Vec<T>;
    fn sample(&self, index: usize) -> &T;
    fn contains(&self, element: &T) -> bool;
}

pub trait Magma<T: AlgebraicElement>: Set<T> {
    fn op(&self, left: T, right: T) -> T;
}

pub trait Associativity<T: AlgebraicElement> {}

pub trait Simigroup<T: AlgebraicElement>: Magma<T> + Associativity<T> {}

pub trait Identity<T: AlgebraicElement> {}

pub trait Monoid<T: AlgebraicElement>: Simigroup<T> + Identity<T> {
    fn get_identity(&self) -> T;
}

pub trait Inverse<T: AlgebraicElement> {}

pub trait Group<T: AlgebraicElement>: Monoid<T> + Inverse<T> {
    fn get_inverse(&self, element: T) -> T;
}

pub trait Commutative<T: AlgebraicElement> {}

pub trait AbelianGroup<T: AlgebraicElement>: Group<T> + Commutative<T> {}
// Rust GAT data base; FHE MAT


// impl Magma<T> for Vec<T>{

// }
// impl Vec<T>{
//     fn new () -> Self {
//         Vec::new()
//     }
// }

// impl Magma<T> for HashSet<T> {
//     fn op(left: T, right: T) -> T {
//         left + right
//     }
// }

// impl HashSet<T>{
//     fn new() -> Self {
//         HashSet::new()
//     }
// }

// pub trait A {
//     fn a(&self) -> i32;
//     fn check 
// }

// pub trait associative {

// }

// struct xxx {}

// impl A for xxx {

// }



// struct Simigroup {

// }

// impl A for c {
//     fn a(&self) -> i32 {
//         1
//     }
// }

// impl associative for c {

// }