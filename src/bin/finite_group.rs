
use abstract_algebra_manual::utils::set::Set;
use abstract_algebra_manual::utils::axiom::axiom;
use abstract_algebra_manual::group::Group;

struct Group7<T> {
    elements: Vec<T>,
    operation: fn(T, T) -> T,
}

impl<T> Set<T> for Group7<T>
where T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {
    fn new_set(elements: Vec<T>) -> Self {
        Group7 { elements, operation: |_, _| panic!("Operation not defined") }
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

impl<T> Group<T> for Group7<T>
where T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {

    fn new_group(elements: Vec<T>, operation: fn(T, T) -> T) -> Self {
        Group7 { elements, operation }
    }

    fn get_identity(&self) -> T {
        self.elements[0]
    }

    fn get_operation(&self) -> fn(T, T) -> T {
        self.operation
    }
}

fn main() {
    // let group = Group7::<i32>::new(vec![0, 1, 2, 3, 4, 5, 6]);
    let group = Group7::new_group(vec![0, 1, 2, 3, 4, 5, 6], |x, y| (x + y) % 7);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let group = Group7::new_group(vec![0, 1, 2, 3, 4, 5, 6], |x, y| (x + y) % 7);

        assert_eq!(axiom((1, 2, 3), group.get_identity(), |x, y| x + y), 4);
    }
}