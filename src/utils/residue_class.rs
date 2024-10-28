struct ResidueClass {
    // This is just a linear residue class, 
    // which means that it is a set of integers that are congruent modulo m
    // x ≡ y (mod m) as well as representative ≡ y (mod modulus)
    // beyond that, there are also residue classes named as quadratic residue classes, 
    // but we will not cover that here
    representative: u64,
    modulus: u64,
    raw_num: u64,
}