use super::algebra_trait::*;
use super::algebra_checker::*;

// There is many way to get a Group from Magma,
// but we will use the following:
// (Set + OP) -> Magma -> Simigroup -> Monoid -> Group
// If you want to learn more ways please check:
// https://en.wikipedia.org/wiki/Magma_(algebra)

impl AlgebraicElement for i32 {}
impl AlgebraicElement for u32 {}
impl AlgebraicElement for i64 {}
impl AlgebraicElement for f64 {}
// for ec curve points
impl AlgebraicElement for (u32, u32) {}

pub struct Set<T: AlgebraicElement> {
    pub elements: Vec<T>,
}

impl <T> SetActions<T> for Set<T>
where T: AlgebraicElement {
    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: usize) -> &T {
        &self.elements[index]
    }

    fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

impl <T> Set<T>
where T: AlgebraicElement {
    pub fn new(elements: Vec<T>) -> Set<T>
    where T: AlgebraicElement {
        let algebra = Set {
            elements: elements,
        };
        algebra
    }
}

impl <T> From<Vec<T>> for Set<T>
where T: AlgebraicElement {
    fn from(elements: Vec<T>) -> Self {
        Set::new(elements)
    }
}

pub struct Magma<T: AlgebraicElement> 
where T: AlgebraicElement {
    pub elements: Vec<T>,
    pub op: BinaryOp<T>,
}

impl <T> SetActions<T> for Magma<T>
where T: AlgebraicElement {
    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: usize) -> &T {
            &self.elements[index]
        }

    fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

impl <T> MagmaActions<T> for Magma<T>
where T: AlgebraicElement {
    fn op(&self, left: T, right: T) -> T {
        (self.op)(left, right)
    }
}

impl <T> Magma<T>
where T: AlgebraicElement {
    pub fn new(elements: Vec<T>, op: BinaryOp<T>) -> Option<Magma<T>> {
        if check_closed(&elements, op) {
            Some(Magma { elements, op: op })
        } else {
            println!("Operation is not closed on the set");
            None
        }
    }
}

impl <T> From<(Set<T>, BinaryOp<T>)> for Magma<T>
where T: AlgebraicElement {
    fn from(value: (Set<T>, BinaryOp<T>)) -> Self {
        let (set, op) = value;
        if check_closed(&set.elements(), op) {
            Magma { elements: set.elements, op: op }
        } else {
            panic!("Operation is not closed on the set");
        }
    }
}

pub struct Simigroup<T: AlgebraicElement> 
where T: AlgebraicElement {
    pub elements: Vec<T>,
    pub op: BinaryOp<T>,
}

impl <T> SetActions<T> for Simigroup<T>
where T: AlgebraicElement {
    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: usize) -> &T {
        &self.elements[index]
    }

    fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

impl <T> MagmaActions<T> for Simigroup<T>
where T: AlgebraicElement {
    fn op(&self, left: T, right: T) -> T {
        (self.op)(left, right)
    }
}

impl <T> Associativity<T> for Simigroup<T>
where T: AlgebraicElement {}

impl <T> Simigroup<T>
where T: AlgebraicElement {
    pub fn new(elements: Vec<T>, op: BinaryOp<T>) -> Option<Simigroup<T>> {
        match (check_closed(&elements, op), check_associative(&elements, op)) {
            (false, _) => {
                println!("Operation is not closed on the set");
                None
            }
            (true, false) => {
                println!("Operation is not associative on the set");
                None
            }
            (true, true) => Some(Simigroup { elements, op: op }),
        }
    }
}

impl <T> From<Magma<T>> for Simigroup<T>
where T: AlgebraicElement {
    fn from(value: Magma<T>) -> Self {
        let Magma { elements, op } = value;
        if check_associative(&elements, op) {
            Simigroup { elements, op }
        } else {
            panic!("Operation is not associative on the set");
        }
    }
}

pub struct Monoid<T: AlgebraicElement> 
where T: AlgebraicElement {
    pub elements: Vec<T>,
    pub op: BinaryOp<T>,
    pub identity: T,
}

impl <T> SetActions<T> for Monoid<T>
where T: AlgebraicElement {
    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: usize) -> &T {
        &self.elements[index]
    }

    fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

impl <T> MagmaActions<T> for Monoid<T>
where T: AlgebraicElement {
    fn op(&self, left: T, right: T) -> T {
        (self.op)(left, right)
    }
}

impl <T> Associativity<T> for Monoid<T>
where T: AlgebraicElement {}

impl <T> SimigroupAction<T> for Monoid<T>
where T: AlgebraicElement {}

impl <T> Identity<T> for Monoid<T>
where T: AlgebraicElement {}

impl <T> MonoidActions<T> for Monoid<T>
where T: AlgebraicElement {
    fn get_identity(&self) -> T {
        self.identity
    }
}

impl <T> Monoid<T>
where T: AlgebraicElement {
    pub fn new(elements: Vec<T>, op: BinaryOp<T>) -> Option<Monoid<T>> {
        match (check_closed(&elements, op), check_associative(&elements, op), find_identity(&elements, op)) {
            (false, _, _) => {
                println!("Operation is not closed on the set");
                None
            }
            (_, false, _) => {
                println!("Operation is not associative on the set");
                None
            }
            (_, _, None) => {
                println!("No identity found on the set");
                None
            }
            (true, true, Some(identity)) => Some(Monoid { elements, op, identity }),
        }
    }
}

impl <T> From<Simigroup<T>> for Monoid<T>
where T: AlgebraicElement {
    fn from(value: Simigroup<T>) -> Self {
        let Simigroup { elements, op } = value;
        match find_identity(&elements, op) {
            Some(identity) => Monoid { elements, op, identity },
            None => panic!("No identity found on the set"),
        }
    }
}

pub struct Group<T: AlgebraicElement> 
where T: AlgebraicElement {
    pub elements: Vec<T>,
    pub op: BinaryOp<T>,
    pub identity: T,
}

impl <T> SetActions<T> for Group<T>
where T: AlgebraicElement {
    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: usize) -> &T {
        &self.elements[index]
    }

    fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

impl <T> MagmaActions<T> for Group<T>
where T: AlgebraicElement {
    fn op(&self, left: T, right: T) -> T {
        (self.op)(left, right)
    }
}

impl <T> Associativity<T> for Group<T>
where T: AlgebraicElement {}

impl <T> SimigroupAction<T> for Group<T>
where T: AlgebraicElement {}

impl <T> Identity<T> for Group<T>
where T: AlgebraicElement {}

impl <T> MonoidActions<T> for Group<T>
where T: AlgebraicElement {
    fn get_identity(&self) -> T {
        self.identity
    }
}

impl <T> Inverse<T> for Group<T>
where T: AlgebraicElement {}

impl <T> GroupActions<T> for Group<T>
where T: AlgebraicElement {
    fn get_inverse(&self, element: T) -> T {
        for &candidate in self.elements() {
            if (self.op)(element, candidate) == self.identity {
                return candidate;
            }
        }
        unreachable!("Any group element should have an unique inverse");
    }
}

impl <T> Group<T>
where T: AlgebraicElement {
    pub fn new(elements: Vec<T>, op: BinaryOp<T>) -> Option<Group<T>> {
        let identity = match find_identity(&elements, op) {
            Some(id) => id,
            None => {
                println!("No identity found on the set");
                return None;
            }
        };
        match (check_closed(&elements, op), check_associative(&elements, op), check_inverse_with_id(&elements, op, identity)) {
            (false, _, _) => {
                println!("Operation is not closed on the set");
                None
            }
            (_, false, _) => {
                println!("Operation is not associative on the set");
                None
            }
            (_, _, false) => {
                println!("No inverse found on the set");
                None
            }
            (true, true, true) => Some(Group { elements, op, identity }),
        }
    }
    
}

impl <T> From<Monoid<T>> for Group<T>
where T: AlgebraicElement {
    fn from(value: Monoid<T>) -> Self {
        let Monoid { elements, op, identity } = value;
        if check_inverse_with_id(&elements, op, identity) {
            Group { elements, op, identity }
        } else {
            panic!("No inverse found on the set");
        }
    }
}
