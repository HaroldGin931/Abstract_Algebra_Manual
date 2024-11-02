use abstract_algebra_manual::algbra_structs::BinaryOp;
use abstract_algebra_manual::algbra_structs::Set;
use abstract_algebra_manual::algbra_structs::Group;
use abstract_algebra_manual::algbra_structs::Field;

struct F7<T> {
    elements: Vec<T>,
    op_0: BinaryOp<T>,
    id_0: Option<T>,
    op_1: BinaryOp<T>,
    id_1: Option<T>,
}

impl<T> Set<T> for F7<T>
where T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {
    fn new_set(elements: Vec<T>) -> Self {
        F7 {elements,
            op_0: |_, _| panic!("Operation not defined"), id_0: None,
            op_1: |_, _| panic!("Operation not defined"), id_1: None,
        }
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

impl<T> Group<T> for F7<T>
where T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {

    fn new_group(elements: Vec<T>,
                 op_0: BinaryOp<T>, id_0: Option<T>) -> Self {
        F7 {elements, op_0, id_0,
            op_1: |_, _| panic!("Operation not defined"), id_1: None,}
    }

    fn identity(&self) -> T {
        self.id_0.unwrap()
    }

    fn get_operation(&self) -> BinaryOp<T> {
        self.op_0
    }
}

impl<T> Field<T> for F7<T>
where T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {
    fn new_field(elements: Vec<T>,
                 op_0: BinaryOp<T>, id_0: Option<T>,
                 op_1: BinaryOp<T>, id_1: Option<T>) -> Self {
        F7 {elements, op_0, id_0, op_1, id_1}
    }

    fn sec_identity(&self) -> T {
        self.id_1.unwrap()
    }

    fn get_sec_opeartion(&self) -> BinaryOp<T> {
        self.op_1
    }
}

fn main() {
    // let group = Group7::<i32>::new(vec![0, 1, 2, 3, 4, 5, 6]);

}

#[cfg(test)]
mod tests {
    use super::*;
    use abstract_algebra_manual::utils::group_axiom::axiom;

    fn get_test_field() -> F7<i32> {
        F7::new_field(vec![0, 1, 2, 3, 4, 5, 6],
            |x, y| (x + y) % 7, Some(0),
            |x, y| (x * y) % 7, Some(1))
    }

    #[test]
    fn test_elements() {
        let field = get_test_field();
        assert_eq!(field.elements(), &vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_sample() {
        let field = get_test_field();
        assert_eq!(field.sample(Some(1)), Some(&1));
    }

    #[test]
    fn test_contains() {
        let field = get_test_field();
        assert_eq!(field.contains(&10), false);
        assert_eq!(field.contains(&7), false);
        assert_eq!(field.contains(&3), true);
    }

    #[test]
    fn axiom_test() {
        let field = get_test_field();
        assert_eq!(axiom((1, 2, 3), field.identity(), |x, y| x + y), 4);
    }

    #[test]
    fn test_operation() {
        let field = get_test_field();
        assert_eq!(field.get_operation()(1, 2), 3);
    }
}