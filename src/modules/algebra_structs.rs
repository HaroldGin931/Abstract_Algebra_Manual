use super::algebra_checker::*;
use super::algebra_trait::*;

// There is many way to get a Group from Magma,
// but we will use the following:
// (Set + OP) -> Magma -> Simigroup -> Monoid -> Group
// If you want to learn more ways please check:
// https://en.wikipedia.org/wiki/Magma_(algebra)

impl AlgebraicElement for i32 {}
impl AlgebraicElement for u32 {}
impl AlgebraicElement for i64 {}
impl AlgebraicElement for f64 {}
// for ec curve points
impl AlgebraicElement for (u32, u32) {}

// FIXME: Use HashMap instead of Vec
pub struct Set<T: AlgebraicElement> {
    pub elements: Vec<T>,
}

impl<T> SetActions<T> for Set<T>
where
    T: AlgebraicElement,
{
    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: usize) -> &T {
        &self.elements[index]
    }

    fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

impl<T> Set<T>
where
    T: AlgebraicElement,
{
    pub fn new(elements: Vec<T>) -> Set<T>
    where
        T: AlgebraicElement,
    {
        Set { elements }
    }
}

impl<T> From<Vec<T>> for Set<T>
where
    T: AlgebraicElement,
{
    fn from(elements: Vec<T>) -> Self {
        Set::new(elements)
    }
}

#[derive(Debug)]
pub struct Magma<T: AlgebraicElement>
where
    T: AlgebraicElement,
{
    pub elements: Vec<T>,
    pub op: BinaryOp<T>,
}

impl<T> SetActions<T> for Magma<T>
where
    T: AlgebraicElement,
{
    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: usize) -> &T {
        &self.elements[index]
    }

    fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

impl<T> MagmaActions<T> for Magma<T>
where
    T: AlgebraicElement,
{
    fn op(&self, left: T, right: T) -> T {
        (self.op)(left, right)
    }
}

impl<T> Magma<T>
where
    T: AlgebraicElement,
{
    pub fn new(elements: Vec<T>, op: BinaryOp<T>) -> Option<Magma<T>> {
        if check_closed(&elements, op) {
            Some(Magma { elements, op })
        } else {
            println!("Operation is not closed on the set");
            None
        }
    }
}

impl<T> From<(Set<T>, BinaryOp<T>)> for Magma<T>
where
    T: AlgebraicElement,
{
    fn from(value: (Set<T>, BinaryOp<T>)) -> Self {
        let (set, op) = value;
        if check_closed(set.elements(), op) {
            Magma { elements: set.elements, op }
        } else {
            panic!("Operation is not closed on the set");
        }
    }
}

pub struct Simigroup<T: AlgebraicElement>
where
    T: AlgebraicElement,
{
    pub elements: Vec<T>,
    pub op: BinaryOp<T>,
}

impl<T> SetActions<T> for Simigroup<T>
where
    T: AlgebraicElement,
{
    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: usize) -> &T {
        &self.elements[index]
    }

    fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

impl<T> MagmaActions<T> for Simigroup<T>
where
    T: AlgebraicElement,
{
    fn op(&self, left: T, right: T) -> T {
        (self.op)(left, right)
    }
}

impl<T> Associativity<T> for Simigroup<T> where T: AlgebraicElement {}

impl<T> Simigroup<T>
where
    T: AlgebraicElement,
{
    pub fn new(elements: Vec<T>, op: BinaryOp<T>) -> Option<Simigroup<T>> {
        match (
            check_closed(&elements, op),
            check_associative(&elements, op),
        ) {
            (false, _) => {
                println!("Operation is not closed on the set");
                None
            }
            (true, false) => {
                println!("Operation is not associative on the set");
                None
            }
            (true, true) => Some(Simigroup { elements, op }),
        }
    }
}

impl<T> From<Magma<T>> for Simigroup<T>
where
    T: AlgebraicElement,
{
    fn from(value: Magma<T>) -> Self {
        let Magma { elements, op } = value;
        if check_associative(&elements, op) {
            Simigroup { elements, op }
        } else {
            panic!("Operation is not associative on the set");
        }
    }
}

pub struct Monoid<T: AlgebraicElement>
where
    T: AlgebraicElement,
{
    pub elements: Vec<T>,
    pub op: BinaryOp<T>,
    pub identity: T,
}

impl<T> SetActions<T> for Monoid<T>
where
    T: AlgebraicElement,
{
    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: usize) -> &T {
        &self.elements[index]
    }

    fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

impl<T> MagmaActions<T> for Monoid<T>
where
    T: AlgebraicElement,
{
    fn op(&self, left: T, right: T) -> T {
        (self.op)(left, right)
    }
}

impl<T> Associativity<T> for Monoid<T> where T: AlgebraicElement {}

impl<T> SimigroupAction<T> for Monoid<T> where T: AlgebraicElement {}

impl<T> Identity<T> for Monoid<T> where T: AlgebraicElement {}

impl<T> MonoidActions<T> for Monoid<T>
where
    T: AlgebraicElement,
{
    fn get_identity(&self) -> T {
        self.identity
    }
}

impl<T> Monoid<T>
where
    T: AlgebraicElement,
{
    pub fn new(elements: Vec<T>, op: BinaryOp<T>) -> Option<Monoid<T>> {
        match (
            check_closed(&elements, op),
            check_associative(&elements, op),
            find_identity(&elements, op),
        ) {
            (false, _, _) => {
                println!("Operation is not closed on the set");
                None
            }
            (_, false, _) => {
                println!("Operation is not associative on the set");
                None
            }
            (_, _, None) => {
                println!("No identity found on the set");
                None
            }
            (true, true, Some(identity)) => {
                Some(Monoid { elements, op, identity })
            }
        }
    }
}

impl<T> From<Simigroup<T>> for Monoid<T>
where
    T: AlgebraicElement,
{
    fn from(value: Simigroup<T>) -> Self {
        let Simigroup { elements, op } = value;
        match find_identity(&elements, op) {
            Some(identity) => Monoid { elements, op, identity },
            None => panic!("No identity found on the set"),
        }
    }
}

pub struct Group<T: AlgebraicElement>
where
    T: AlgebraicElement,
{
    pub elements: Vec<T>,
    pub op: BinaryOp<T>,
    pub identity: T,
}

impl<T> SetActions<T> for Group<T>
where
    T: AlgebraicElement,
{
    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: usize) -> &T {
        &self.elements[index]
    }

    fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

impl<T> MagmaActions<T> for Group<T>
where
    T: AlgebraicElement,
{
    fn op(&self, left: T, right: T) -> T {
        (self.op)(left, right)
    }
}

impl<T> Associativity<T> for Group<T> where T: AlgebraicElement {}

impl<T> SimigroupAction<T> for Group<T> where T: AlgebraicElement {}

impl<T> Identity<T> for Group<T> where T: AlgebraicElement {}

impl<T> MonoidActions<T> for Group<T>
where
    T: AlgebraicElement,
{
    fn get_identity(&self) -> T {
        self.identity
    }
}

impl<T> Inverse<T> for Group<T> where T: AlgebraicElement {}

impl<T> GroupActions<T> for Group<T>
where
    T: AlgebraicElement,
{
    fn get_inverse(&self, element: T) -> T {
        for &candidate in self.elements() {
            if (self.op)(element, candidate) == self.identity {
                return candidate;
            }
        }
        unreachable!("Any group element should have an unique inverse");
    }
}

impl<T> Group<T>
where
    T: AlgebraicElement,
{
    pub fn new(elements: Vec<T>, op: BinaryOp<T>) -> Option<Group<T>> {
        let identity = match find_identity(&elements, op) {
            Some(id) => id,
            None => {
                println!("No identity found on the set");
                return None;
            }
        };
        match (
            check_closed(&elements, op),
            check_associative(&elements, op),
            check_inverse_with_id(&elements, op, identity),
        ) {
            (false, _, _) => {
                println!("Operation is not closed on the set");
                None
            }
            (_, false, _) => {
                println!("Operation is not associative on the set");
                None
            }
            (_, _, false) => {
                println!("No inverse found on the set");
                None
            }
            (true, true, true) => Some(Group { elements, op, identity }),
        }
    }
}

impl<T> From<Monoid<T>> for Group<T>
where
    T: AlgebraicElement,
{
    fn from(value: Monoid<T>) -> Self {
        let Monoid { elements, op, identity } = value;
        if check_inverse_with_id(&elements, op, identity) {
            Group { elements, op, identity }
        } else {
            panic!("No inverse found on the set");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_set_and_op() -> (Set<i32>, BinaryOp<i32>) {
        let set = Set::new(vec![0, 1, 2, 3, 4, 5, 6]);
        let op = |a: i32, b: i32| a + b;
        (set, op)
    }

    fn get_test_set_and_op_mod_6() -> (Set<i32>, BinaryOp<i32>) {
        let set = Set::new(vec![0, 1, 2, 3, 4, 5]);
        let op = |a: i32, b: i32| (a + b) % 6;
        (set, op)
    }


    fn get_test_set_and_op_mod_7() -> (Set<i32>, BinaryOp<i32>) {
        let set = Set::new(vec![0, 1, 2, 3, 4, 5, 6]);
        let op = |a: i32, b: i32| (a + b) % 7;
        (set, op)
    }

    #[test]
    fn test_set() {
        let set = Set::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(set.elements, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_magma_not_closure() {
        let (set, op) = get_test_set_and_op();
        let magma = Magma::new(set.elements.clone(), op);
        assert!(magma.is_none());
    }

    #[test]
    fn test_magma() {
        let (set, op) = get_test_set_and_op_mod_6();
        let magma = Magma::new(set.elements, op);
        assert!(magma.is_some());
    }

    #[test]
    fn test_simigroup() {
        let (set, op) = get_test_set_and_op_mod_6();
        let simigroup = Simigroup::new(set.elements, op);
        assert_eq!(simigroup.unwrap().elements, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_simigroup_by_compare() {
        let (set, _) = get_test_set_and_op_mod_6();
        fn bigger_one(a: i32, b: i32) -> i32 {
            if a > b {
                1
            } else {
                0
            }
        }
        let op = bigger_one;
        let simigroup = Simigroup::new(set.elements, op);
        assert!(simigroup.is_none());
    }

    #[test]
    fn test_monoid() {
        let (set, op) = get_test_set_and_op_mod_7();
        let monoid = Monoid::new(set.elements, op);
        assert_eq!(monoid.unwrap().elements, vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_group() {
        let (set, op) = get_test_set_and_op_mod_7();
        let group = Group::new(set.elements, op);
        assert_eq!(group.unwrap().elements, vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_group_no_inverse() {
        let (set, _) = get_test_set_and_op_mod_6();
        let op = |a: i32, b: i32| (a * b) % 6;
        let group = Group::new(set.elements, op);
        assert!(group.is_none());
    }
}
