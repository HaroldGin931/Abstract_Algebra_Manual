use crate::modules::algebra_trait;

fn check_closed<T, A>(s: A, op: algebra_trait::BinaryOp<T>) -> bool 
where
    A: algebra_trait::Set<T>,
    T: algebra_trait::AlgebraicElement, 
{
    for &a in s.elements() {
        for &b in s.elements() {
            if !s.contains(&op(a, b)) || 
               !s.contains(&op(b, a)) {
                return false;
            }
        }
    }
    true
}

fn check_associative<T, A>(mag: A) -> bool
where
    A: algebra_trait::Magma<T>,
    T: algebra_trait::AlgebraicElement, 
{
    for &a in mag.elements() {
        for &b in mag.elements() {
            for &c in mag.elements() {
                if !(mag.op(a, mag.op(b, c)) == 
                     mag.op(mag.op(a, b), c)
                    ) {
                    return false;
                }
            }
        }
    }
    true
}

fn find_identity<T, A>(simi: A) -> Option<T>
where
    A: algebra_trait::Simigroup<T>,
    T: algebra_trait::AlgebraicElement, 
{
    for &a in simi.elements() {
        if simi.elements().iter().all(|&x| simi.op(a, x) == x) {
            return Some(a);
        }
    }
    None
}

fn check_inverse<T, A>(mono: A) -> bool
where
    A: algebra_trait::Monoid<T>,
    T: algebra_trait::AlgebraicElement, 
{
    for &a in mono.elements() {
        if !mono.elements().iter().any(|&x| mono.op(a, x) == mono.get_identity()) {
            return false;
        }
    }
    true
}

fn check_commutative<T, A>(g: A) -> bool
where
    A: algebra_trait::Group<T>,
    T: algebra_trait::AlgebraicElement, 
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