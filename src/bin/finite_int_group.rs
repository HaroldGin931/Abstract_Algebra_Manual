
use abstract_algebra_manual::utils::set::Set;
use abstract_algebra_manual::utils::axiom::axiom;
use abstract_algebra_manual::group::Group;

struct FiniteIntGroup {
    set: Set<i32>,
    operation: fn(i32, i32) -> i32,
    identity: i32
}

impl FiniteIntGroup {
    fn new(set: Set<i32>, operation: fn(i32, i32) -> i32, identity: i32) -> Self {
        FiniteIntGroup {
            set,
            operation,
            identity
        }
    }
}
impl Group<i32> for FiniteIntGroup {
    fn get_set(&self) -> &Set<i32> {
        &self.set
    }

    fn get_identity(&self) -> i32 {
        self.identity
    }

    fn get_operation(&self) -> fn(i32, i32) -> i32 {
        self.operation
    }
}

fn main() {
	let set = Set::new(vec![0, 1, 2, 3, 4, 5, 6]);

    let group = FiniteIntGroup::new(set, |x, y| (x + y) % 7, 0);

    println!("{:?}", axiom((1, 2, 3), group.get_identity(), |x, y| x + y));

}