use abstract_algebra_manual::modules::algebra_structs;
use abstract_algebra_manual::modules::algebra_trait::*;

fn main() {
    let add_group =
        algebra_structs::Group::new(vec![0, 1, 2, 3, 4, 5, 6], |x, y| {
            (x + y) % 7
        })
        .unwrap();
    println!("{:?}", add_group.elements());

    let mul_group =
        algebra_structs::Group::new(vec![1, 2, 3, 4, 5, 6], |x, y| (x * y) % 7)
            .unwrap();
    println!("{:?}", mul_group.elements());
}

#[cfg(test)]
mod tests {
    use super::*;
    use abstract_algebra_manual::modules::algebra_checker;

    fn get_test_group() -> Group7<i32> {
        Group7::new_group(
            vec![0, 1, 2, 3, 4, 5, 6],
            |x, y| (x + y) % 7,
            Some(0),
        )
    }

    #[test]
    fn test_elements() {
        let group = get_test_group();
        assert_eq!(group.elements(), &vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_sample() {
        let group = get_test_group();
        assert_eq!(group.sample(Some(1)), Some(&1));
    }

    #[test]
    fn test_contains() {
        let group = get_test_group();
        assert_eq!(group.contains(&10), false);
        assert_eq!(group.contains(&7), false);
        assert_eq!(group.contains(&3), true);
    }

    #[test]
    fn axiom_test() {
        let group = get_test_group();
        let mut checker = GroupAxiomChecker::new(
            group.elements().clone(),
            group.get_operation(),
            group.identity,
        );
        assert_eq!(checker.result(), true);
    }

    #[test]
    fn test_operation() {
        let group = get_test_group();
        assert_eq!(group.get_operation()(1, 2), 3);
    }
}
