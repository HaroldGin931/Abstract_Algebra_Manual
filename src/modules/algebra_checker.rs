use crate::modules::algebra_trait::*;

pub fn check_closed<T>(v: &Vec<T>, op: BinaryOp<T>) -> bool
where
    T: AlgebraicElement,
{
    for &a in v {
        for &b in v {
            if !v.contains(&op(a, b)) {
                println!(
                    "The result of op({:?}, {:?}) is {:?} , 
                which is not in the set.",
                    a,
                    b,
                    op(a, b)
                );
                return false;
            } else if !v.contains(&op(b, a)) {
                println!(
                    "The result of op({:?}, {:?}) is {:?} , 
                which is not in the set.",
                    b,
                    a,
                    op(b, a)
                );
                return false;
            }
        }
    }
    true
}

pub fn check_associative<T>(v: &Vec<T>, op: BinaryOp<T>) -> bool
where
    T: AlgebraicElement,
{
    for &a in v {
        for &b in v {
            for &c in v {
                if !(op(a, op(b, c)) == op(op(a, b), c)) {
                    return false;
                }
            }
        }
    }
    true
}

pub fn find_identity<T>(v: &Vec<T>, op: BinaryOp<T>) -> Option<T>
where
    T: AlgebraicElement,
{
    v.iter()
        .find(|&&a| v.iter().all(|&x| op(a, x) == x && op(x, a) == x))
        .copied()
}

pub fn check_identity<T>(v: &Vec<T>, op: BinaryOp<T>, id: T) -> bool
where
    T: AlgebraicElement,
{
    for &a in v {
        if op(a, id) != a || op(id, a) != a {
            return false;
        }
    }
    true
}

// FIXME: check_inverse now can only check the struct which has an identity,
// the struct without identity will return false
// check_inverse should be able to check some structs without identity
// such like Quasigroup
// Actually we can use op(a, a) to go through all the elements in the set
// and we can get the inverse of a during this process
pub fn check_inverse_with_id<T>(v: &Vec<T>, op: BinaryOp<T>, id: T) -> bool
where
    T: AlgebraicElement,
{
    let mut used = vec![false; v.len()];
    for element in v {
        if !check_identity(v, op, id) {
            println!("Un leagal identity {:?} for element {:?}", id, *element);
            return false;
        }
        let mut accumlator: u8 = 0;
        // inverse of an element must be an unique one,
        // if two element have the same inverse, the set is not inversable
        for (i, candidate) in v.iter().enumerate() {
            // println!("{:?} {:?}", *element, *candidate);
            if (op)(*element, *candidate) == id {
                if !used[i] {
                    used[i] = true
                } else {
                    return false;
                };
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

pub fn check_commutative<T, A>(g: A) -> bool
where
    A: GroupActions<T>,
    T: AlgebraicElement,
{
    for &a in g.elements() {
        for &b in g.elements() {
            if !(g.op(a, b) == g.op(b, a)) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::algebra_structs::*;

    #[test]
    fn test_check_closed() {
        let mut elements = Vec::new();
        elements.push(1);
        elements.push(2);
        elements.push(3);
        let op = |x: i32, y: i32| (x + y) % 7;
        assert_eq!(check_closed(&elements, op), false);
    }

    #[test]
    fn test_check_associative() {
        let mut elements = Vec::new();
        elements.push(1);
        elements.push(2);
        elements.push(3);
        let m =
            { Magma { elements: elements, op: |x: i32, y: i32| (x + y) % 7 } };
        assert_eq!(check_associative(&m.elements, m.op), true);
    }

    #[test]
    fn test_find_identity_none() {
        let mut elements = Vec::new();
        elements.push(1);
        elements.push(2);
        elements.push(3);
        let op = |x: i32, y: i32| (x + y) % 3;
        assert_eq!(find_identity(&elements, op), None);
    }

    #[test]
    fn test_find_add_identity() {
        let elements = vec![0, 1, 2, 3, 4, 5, 6];
        let op = |x: i32, y: i32| (x + y) % 7;
        assert_eq!(find_identity(&elements, op), Some(0));
    }

    #[test]
    fn test_find_mod_identity() {
        let elements = vec![0, 1, 2, 3, 4, 5, 6];
        let op = |x: i32, y: i32| (x * y) % 7;
        assert_eq!(find_identity(&elements, op), Some(1));
    }

    #[test]
    fn test_check_inverse() {
        let elements = vec![0, 1, 2, 3, 4, 5, 6];
        let op = |x: i32, y: i32| (x + y) % 7;
        let id = 1;
        assert_eq!(check_inverse_with_id(&elements, op, id), false);
    }

    #[test]
    fn test_check_mul_inverse() {
        let elements = vec![0, 1, 2, 3, 4, 5, 6];
        let op = |x: i32, y: i32| (x * y) % 7;
        let id = 1;
        assert_eq!(check_inverse_with_id(&elements, op, id), false);
    }

    #[test]
    fn test_check_commutative() {
        let elements = vec![0, 1, 2, 3, 4, 5, 6];
        let op = |x: i32, y: i32| (x + y) % 7;
        let identity = 0;
        let g = {
            Group {
                elements,
                op,
                identity,
            }
        };
        assert_eq!(check_commutative(g), true);
    }
}
