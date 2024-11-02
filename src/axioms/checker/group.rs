// The axiom function is a helper function that checks if a given set of elements and an operation satisfy the four axioms of a operation and set. the implemention of axiom should be hide to the caller. The caller should only be able to use the function to check if a given set of elements and an operation satisfy the four axioms of a operation and set
use crate::algbra_structs::BinaryOp;

pub struct GroupAxiomChecker<T>
where
    T: std::cmp::PartialEq + std::ops::Neg<Output = T> + Default + Copy + std::fmt::Debug
{
    // Caller's raw input
    raw_set: Vec<T>,
    op: BinaryOp<T>,
    claimed_identity: Option<T>,
    // Temperary storage for the output of the bi-operation
    output_matrix: Vec<Vec<T>>,
    identity: Option<T>,
    checked: bool,
}

impl<T> GroupAxiomChecker<T> 
where
    T: std::cmp::PartialEq + std::ops::Neg<Output = T> + Default + Copy + std::fmt::Debug
{
    pub fn new(raw_set: Vec<T>, op: BinaryOp<T>, claimed_identity: Option<T>) -> Self {
        // to verify that the set and operation satisfy the group axioms,
        // we need to compute all possible outcomes for each
        // pairwise combination of elements within the set.

        let output_matrix: Vec<Vec<T>> = raw_set.iter()
        .map(|&x| {
            raw_set.iter().map(|&y| op(x, y)).collect()
        }).collect();

        GroupAxiomChecker { raw_set, op, claimed_identity, output_matrix, identity: None, checked: false }
    }

    pub fn is_closed(&self) -> bool {
        self.output_matrix.iter().enumerate().all(|(row_index, row)| {
            row.iter().enumerate().all(|(col_index, element)|{
                if !self.raw_set.contains(&element) {
                    println!("The result of op({:?}, {:?}) is {:?}", self.raw_set[row_index], self.raw_set[col_index], element);
                    false
                } else { true }
            })
        })
    }

    // a + (b + c) = (a + b) + c
    pub fn is_associative(&self) -> bool {
        self.raw_set.iter().all(|&a| {
            self.raw_set.iter().all(|&b| {
                self.raw_set.iter().all(|&c| {
                    if (self.op)(a, (self.op)(b, c)) != (self.op)((self.op)(a, b), c) {
                        println!("Find out {:?} {:?} {:?} are not satified associative", a, b, c);
                        false
                    } else { true }
                })
            })
        })
    }

    fn find_identity(&mut self) -> Option<T> {
        self.output_matrix.iter().enumerate()
        .find(|&(_,v)|self.raw_set == *v)
        .map(|(i,v)|v[i])
    }

    fn check_claimed_identity(&mut self) {
        if self.raw_set.iter().all(|&x| (self.op)(self.claimed_identity.unwrap(), x) == x) {
            self.identity = self.claimed_identity;
            self.checked = true;
        } else {
            println!("The claimed identity {:?} is not an real identity.", self.claimed_identity.unwrap());
            self.identity = self.find_identity();
            self.checked = true;
        }
    }

    pub fn get_identity(&mut self) -> Option<T> {
        if self.checked {
            return self.identity;
        }

        if let Some(_) = self.claimed_identity {
            self.check_claimed_identity();
        } else {
            self.identity = self.find_identity();
            self.checked = true;
        }
        return self.identity;
    }

    pub fn has_identity(&mut self) -> bool {
        if !self.checked {
            self.get_identity();
        }
        if self.identity.is_some() {
            return true;
        } else {return false};
    }

    pub fn is_inversable(&self) -> bool {
        let mut used = vec![false; self.raw_set.len()];
        for element in self.raw_set.iter() {
            let mut accumlator: u8 = 0;
            // inverse of an element must be an unique one,
            // if two element have the same inverse, the set is not inversable
            for (i, candidate) in self.raw_set.iter().enumerate() {
                // println!("{:?} {:?}", *element, *candidate);
                if (self.op)(*element, *candidate) == self.identity.unwrap() {
                    if !used[i] {used[i] = true} else {return false};
                    accumlator += 1;
                } 
            }
            // if we find more than one inverse, the accumlator will be greater than 1,
            // if we find no inverse, the accumlator will be 0, both cases are not inversable
            if accumlator != 1 {
                return false;
            }
        }
        true
    }

    pub fn result(&mut self) -> bool {
        if !self.is_closed() {
            println!("The set is not closed under the operation.");
            return false;}
        if !self.is_associative() {
            println!("The set is not associativ under the operation.");
            return false;}
        if !self.has_identity() {
            println!("The set has no identity under the operation.");
            return false;}
        if !self.is_inversable()  {
            println!("The set is not inversable under the operation.");
            return false;}
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_case_with_id() {
        let set = vec![0, 1, 2, 3, 4, 5, 6];
        let op = |x, y| (x + y) % 7;
        let identity = Some(0);
        let mut checker = GroupAxiomChecker::new(set, op, identity);
        assert_eq!(checker.result(), true);
    }

    #[test]
    fn test_add_case_without_id() {
        let set = vec![0, 1, 2, 3, 4, 5, 6];
        let op = |x, y| (x + y) % 7;
        let identity = None;
        let mut checker = GroupAxiomChecker::new(set, op, identity);
        assert_eq!(checker.result(), true);
    }

    #[test]
    fn test_add_case_with_wrong_id() {
        // FIXME: Should generate a warning to the user
        let set = vec![0, 1, 2, 3, 4, 5, 6];
        let op = |x, y| (x + y) % 7;
        let identity = Some(1);
        let mut checker = GroupAxiomChecker::new(set, op, identity);
        assert_eq!(checker.result(), true);
    }

    #[test]
    fn test_mult_case_with_id() {
        let set = vec![1, 2, 3, 4, 5, 6];
        let op = |x, y| (x * y) % 7;
        let identity = Some(1);
        let mut checker = GroupAxiomChecker::new(set, op, identity);
        assert_eq!(checker.result(), true);
    }

    #[test]
    fn test_mult_case_with_wrong_id() {
        let set = vec![1, 2, 3, 4, 5, 6];
        let op = |x, y| (x * y) % 7;
        let identity = Some(0);
        let mut checker = GroupAxiomChecker::new(set, op, identity);
        assert_eq!(checker.result(), true);
    }

    #[test]
    fn test_mult_case_with_wrong_op() {
        let set = vec![1, 2, 3, 4, 5, 6];
        let op = |x, y| (x * y);
        let identity = Some(0);
        let mut checker = GroupAxiomChecker::new(set, op, identity);
        assert_eq!(checker.result(), false);
    }

}