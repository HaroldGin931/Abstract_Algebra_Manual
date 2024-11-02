use crate::algbra_structs::BinaryOp;
use crate::algbra_structs::Set;
use crate::algbra_structs::Group;
use crate::utils::axiom::GroupAxiomChecker;

pub struct GroupHandler<T> {
    elements: Vec<T>,
    operation: BinaryOp<T>,
    identity: Option<T>,
}

impl<T> Set<T> for GroupHandler<T> 
where
T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {
    fn new_set(elements: Vec<T>) -> Self {
        GroupHandler { elements, operation: |_, _| panic!("Operation not defined"), identity: None }
    }

    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: Option<usize>) -> Option<&T> {
        let idx = index.unwrap_or(0);
        self.elements().get(idx)
    }

    fn contains(&self, element: &T) -> bool {
        self.elements().contains(element)
    }
}

impl<T> Group<T> for GroupHandler<T> 
where 
T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {
    fn new_group(elements: Vec<T>, operation: BinaryOp<T>, identity: Option<T>) -> Self {
        GroupHandler { elements, operation, identity }
    }

    fn identity(&self) -> T {
        self.identity.unwrap()
    }

    fn get_operation(&self) -> BinaryOp<T> {
        self.operation
    }
}

pub fn create_group<T>(elements: Vec<T>, operation: BinaryOp<T>, id: Option<T>) -> GroupHandler<T> 
where
    T: std::cmp::PartialEq + std::ops::Neg<Output = T> + std::ops::Add<Output = T> + Default + Copy + std::fmt::Debug
{
    let mut checker = GroupAxiomChecker::new(elements.clone(), operation, id);
    if checker.result() {
        GroupHandler::new_group(elements, operation, checker.get_identity())
    } else {
        panic!("The input set and operation do not form a group.")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // use crate::utils::axiom::axiom;

    fn get_test_group() -> GroupHandler<i32> {
        create_group(vec![0, 1, 2, 3, 4, 5, 6], |x, y| (x + y) % 7, Some(0))
    }

    #[test]
    fn test_normal_case() {
        let group = get_test_group();
        assert_eq!(group.elements(), &vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_identity() {
        let group = create_group(vec![0, 1, 2, 3, 4, 5, 6], |x, y| (x + y) % 7, None);
        assert_eq!(group.identity(), 0);
    }
}