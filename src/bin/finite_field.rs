use abstract_algebra_manual::algbra_structs::Set;
use abstract_algebra_manual::algbra_structs::Group;
use abstract_algebra_manual::algbra_structs::Field;
use abstract_algebra_manual::utils::axiom::axiom;

struct F7<T> {
    elements: Vec<T>,
    op_0: fn(T, T) -> T,
    op_1: fn(T, T) -> T,
}

impl<T> Set<T> for F7<T>
where T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {
    fn new_set(elements: Vec<T>) -> Self {
        F7 {elements,
            op_0: |_, _| panic!("Operation not defined"),
            op_1: |_, _| panic!("Operation not defined")
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
                 op_0: fn(T, T) -> T) -> Self {
        F7 {elements, op_0,
            op_1: |_, _| panic!("Operation not defined")}
    }

    fn has_identity(&self) -> T {
        self.elements[0]
    }

    fn get_operation(&self) -> fn(T, T) -> T {
        self.op_0
    }
}

impl<T> Field<T> for F7<T>
where T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {
    fn new_field(elements: Vec<T>,
                 op_0: fn(T, T) -> T,
                 op_1: fn(T, T) -> T) -> Self {
        F7 {elements, op_0, op_1 }
    }

    fn get_sec_opeartion(&self) -> fn(T, T) -> T {
        self.op_1
    }
}

fn main() {
    // let group = Group7::<i32>::new(vec![0, 1, 2, 3, 4, 5, 6]);

}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_field() -> F7<i32> {
        F7::new_field(vec![0, 1, 2, 3, 4, 5, 6],
            |x, y| (x + y) % 7,
            |x, y| (x * y) % 7)
    }

    #[test]
    fn test_elements() {
        let group = get_test_field();
        assert_eq!(group.elements(), &vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_sample() {
        let group = get_test_field();
        assert_eq!(group.sample(Some(1)), Some(&1));
    }

    #[test]
    fn test_contains() {
        let group = get_test_field();
        assert_eq!(group.contains(&10), false);
        assert_eq!(group.contains(&7), false);
        assert_eq!(group.contains(&3), true);
    }

    #[test]
    fn axiom_test() {
        let group = get_test_field();
        assert_eq!(axiom((1, 2, 3), group.has_identity(), |x, y| x + y), 4);
    }

    #[test]
    fn test_operation() {
        let group = get_test_field();
        assert_eq!(group.get_operation()(1, 2), 3);
    }


}