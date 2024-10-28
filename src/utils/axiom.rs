// The axiom function is a helper function that checks if a given set of elements and an operation satisfy the four axioms of a operation and set. the implemention of axiom should be hide to the caller. The caller should only be able to use the function to check if a given set of elements and an operation satisfy the four axioms of a operation and set

pub fn axiom<T>(a: (T, T, T), id: T, op: fn(T, T) -> T) -> u8 
where
    T: std::cmp::PartialEq + std::ops::Neg<Output = T> + Copy,
{
    // is_closed
    let _ = op(a.0, a.1);
    // is_associative
    if !(op(a.0, op(a.1, a.2)) == op(op(a.0, a.1), a.2)) {
        return 1;
    }
    // has_identity
    if !(op(id, id) == id) {
        return 2;
    }
    // is_inversable
    if !(op(a.0, -a.0) == id) {
        return 3;
    }
    4
}


// pub fn axiom<T>(a: (T, T, T), id: T, op: fn(T, T) -> T) -> &'static str 
// where
//     T: std::cmp::PartialEq + std::ops::Neg<Output = T> + Copy,
// {
//     // is_closed
//     let _ = op(a.0, a.1);
//     // is_associative
//     if !(op(a.0, op(a.1, a.2)) == op(op(a.0, a.1), a.2)) {
//         return "associative is not satisfied";
//     }
//     // has_identity
//     if !(op(id, id) == id) {
//         return "identity is not satisfied, this is a simigroup";
//     };
//     // is_inversable
//     if !(op(a.0, -a.0) == id) {
//         return "inversable is not satisfied, this is a monoid";
//     }

//     "Axiom is satisfied"
// }
