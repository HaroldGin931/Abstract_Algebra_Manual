// TODO : should define "magma, semigroup, monoid, group, abelian group" 's
// elements and actions, the op's input should be the same type as the elements

pub type BinaryOp<T> = fn(T, T) -> T;
// pub type BinaryOp<T> = Box<dyn Fn(T, T) -> T>;

pub trait AlgebraicElement: std::fmt::Debug + PartialEq + Copy {}

pub trait SetActions<T: AlgebraicElement> {
    fn elements(&self) -> &Vec<T>;
    fn sample(&self, index: usize) -> &T;
    fn contains(&self, element: &T) -> bool;
}

pub trait MagmaActions<T: AlgebraicElement>: SetActions<T> {
    fn op(&self, left: T, right: T) -> T;
}

pub trait Associativity<T: AlgebraicElement> {}

//  Semigroup is a set equipped with an associative binary operation.
pub trait SimigroupAction<T: AlgebraicElement>:
    MagmaActions<T> + Associativity<T>
{
}

pub trait Identity<T: AlgebraicElement> {}

pub trait MonoidActions<T: AlgebraicElement>:
    SimigroupAction<T> + Identity<T>
{
    fn get_identity(&self) -> T;
}

pub trait Inverse<T: AlgebraicElement> {}

pub trait GroupActions<T: AlgebraicElement>:
    MonoidActions<T> + Inverse<T>
{
    fn get_inverse(&self, element: T) -> T;
}

pub trait Commutative<T: AlgebraicElement> {}

pub trait AbelianGroupActions<T: AlgebraicElement>:
    GroupActions<T> + Commutative<T>
{
}
