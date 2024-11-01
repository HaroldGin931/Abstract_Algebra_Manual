// The axiom function is a helper function that checks if a given set of elements and an operation satisfy the four axioms of a operation and set. the implemention of axiom should be hide to the caller. The caller should only be able to use the function to check if a given set of elements and an operation satisfy the four axioms of a operation and set
use crate::algbra_structs::BinaryOp;

pub fn axiom<T>(a: (T, T, T), id: T, op: fn(T, T) -> T) -> u8 
where
    T: std::cmp::PartialEq + std::ops::Neg<Output = T> + Copy,
{
    // is_closed
    let _ = op(a.0, a.1);
    // is_associative
    if !(op(a.0, op(a.1, a.2)) == op(op(a.0, a.1), a.2)) {
        return 1;
    }
    // has_identity
    if !(op(id, id) == id) {
        return 2;
    }
    // is_inversable
    if !(op(a.0, -a.0) == id) {
        return 3;
    }
    4
}

pub struct GroupAxiomChecker<T> {
    raw_set: Vec<T>,
    op: BinaryOp<T>,
    pool: Vec<Vec<T>>,
}

impl<T> GroupAxiomChecker<T> 
where
    T: std::cmp::PartialEq + std::ops::Neg<Output = T> + Default + Copy + std::fmt::Debug
{
    pub fn new(raw_set: Vec<T>, op: BinaryOp<T> ) -> Self {
        let mut pool = vec![vec![T::default(); raw_set.len()]; raw_set.len()];

        for i in 0..raw_set.len() {
            for k in 0..raw_set.len() {
                pool[i][k] = op(raw_set[i], raw_set[k]);
            }
        }

        GroupAxiomChecker { raw_set, op, pool }
    }

    pub fn find_identity(&self) -> Option<T> {
        for (i, v) in self.pool.iter().enumerate() {
            if self.raw_set == *v {
                return Some(self.raw_set[i]);
            }
        }
        println!("Identity not found");
        None
    }

    pub fn check_identity(&self, id: T) -> bool {
        for i in self.raw_set.iter() {
            if id != (self.op)(id, *i) {
                // println!("Identity check failed at op({}, {})",id, *i);
                return false;
            }
        }
        true
    }

    pub fn check_associative(&self) -> bool {
        for i in 0..self.raw_set.len() {
            for j in 0..self.raw_set.len() {
                for k in 0..self.raw_set.len() {
                    if (self.op)(self.raw_set[i], (self.op)(self.raw_set[j], self.raw_set[k])) != (self.op)((self.op)(self.raw_set[i], self.raw_set[j]), self.raw_set[k]) {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn check_inversable(&self) -> bool {
        true
    }

    pub fn check_closed(&self) -> bool {
        for i in self.pool.iter() {
            for j in i.iter() {
                if !self.raw_set.contains(j) {
                    return false;
                }
            }
        }
        true
    }
}