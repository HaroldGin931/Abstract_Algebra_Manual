use crate::algbra_structs::BinaryOp;
use crate::axioms::checker::group::GroupAxiomChecker;

pub struct FieldAxiomChecker<T>
where
    T: std::cmp::PartialEq + std::ops::Neg<Output = T> + Default + Copy + std::fmt::Debug
{
    // Caller's raw input
    raw_set: Vec<T>,
    op_0: BinaryOp<T>,
    claimed_identity_0: Option<T>,
    op_1: BinaryOp<T>,
    claimed_identity_1: Option<T>,
    identity_0: Option<T>,
    identity_1: Option<T>,
    checked_0: bool,
    checked_1: bool,
}

impl <T> FieldAxiomChecker<T>
where
    T: std::cmp::PartialEq + std::ops::Neg<Output = T> + Default + Copy + std::fmt::Debug
{
    pub fn new(raw_set: Vec<T>, 
        op_0: BinaryOp<T>, claimed_identity_0: Option<T>,
        op_1: BinaryOp<T>, claimed_identity_1: Option<T>) -> Self {

            FieldAxiomChecker {raw_set,
                op_0, claimed_identity_0,
                op_1, claimed_identity_1,
                identity_0: None, identity_1: None,
                checked_0: false, checked_1: false}
    }

    fn check_add_group(&mut self) {
        let mut add_group = GroupAxiomChecker::new(self.raw_set.clone(), self.op_0, self.claimed_identity_0);
        if add_group.result() {
            self.identity_0 = add_group.get_identity();
            self.checked_0 = true;
        }
    }

    fn check_mul_group(&mut self) {
        let mut mul_group = GroupAxiomChecker::new(self.raw_set[1..].to_vec(), self.op_1, self.claimed_identity_1);
        if mul_group.result() {
            self.identity_1 = mul_group.get_identity();
            self.checked_1 = true;
        }
    }

    fn is_distributive(&self) -> bool {
        let mut is_distributive = true;
        for &x in self.raw_set.iter() {
            for &y in self.raw_set.iter() {
                for &z in self.raw_set.iter() {
                    let left = (self.op_1)(x, (self.op_0)(y, z));
                    let right = (self.op_0)((self.op_1)(x, y), (self.op_1)(x, z));
                    if left != right {
                        is_distributive = false;
                        break;
                    }
                }
            }
        }
        is_distributive
    }

    pub fn result(&mut self) -> bool {
        self.check_add_group();
        self.check_mul_group();
        self.is_distributive() && self.checked_0 && self.checked_1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_axiom_checker() {
        let set = vec![0, 1, 2, 3, 4, 5, 6];
        let op_0 = |x, y| (x + y) % 7;
        let op_1 = |x, y| (x * y) % 7;
        let id_0 = Some(0);
        let id_1 = Some(1);

        let mut checker = FieldAxiomChecker::new(set, op_0, id_0, op_1, id_1);

        assert_eq!(checker.result(), true);
    }

    #[test]
    fn test_field_axiom_checker_without_id() {
        let set = vec![0, 1, 2, 3, 4, 5];
        let op_0 = |x, y| (x + y) % 7;
        let op_1 = |x, y| (x * y) % 7;
        let id_0 = None;
        let id_1 = None;

        let mut checker = FieldAxiomChecker::new(set, op_0, id_0, op_1, id_1);

        assert_eq!(checker.result(), false);
    }
}