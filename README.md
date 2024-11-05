# Abstract Algebra Manual

Learn group, ring, and field structures from a Rust programming language perspective.

## Motivation

To understand the algebraic structures commonly used in cryptography, we often need to apply knowledge of abstract algebra. However, this knowledge is typically written in mathematical language, which can present challenges for developers. In essence, though, abstract algebra is about using abstract thinking to analyze and identify properties of mathematical objects, classifying and systematically discussing these properties. Once this perspective is understood, abstract algebra becomes much easier to grasp.

Rust, as a language with many excellent features, allows us to use traits and generics to define these abstract algebraic interfaces. By leveraging the compiler and test functions, we can create an introductory book on abstract algebra from a Rust perspective.

## Usage

1. `cargo run`, 
2. Reading the code
   1. algebra_trait
   2. algebra_checker
   3. algebra_struct
   4. examples folder
3. Play it with ZKlings(Not implemented)

## Challenge

The key challenge is that the target audience differs from that of traditional cryptography libraries.

In traditional cryptographic libraries, the operations defined are derived and proven to satisfy the relevant algebraic structure axioms. These libraries primarily focus on implementing these operations/algorithms in code and optimizing them as much as possible.

However, the goal of this project is to provide a set of traits related to algebraic structures, along with instances implemented based on these traits. It also offers exercises for users to try implementing some simple algebraic structures.

The project therefore needs to complete the relevant interfaces and test functions. However, the traits should not be overly restrictive; instead, users should be allowed the space to make mistakes. The test functions will then help check these implementations, showing them which axioms their defined operations fail to satisfy, and prompting them to reflect on the properties of these operations.

## Roadmap

- [x] Stage 1: Implement a basic Group trait with finite groups and an axiom-check function.

- [x] Stage 2: Implement a basic Field trait with finite groups.
  - [x] Group Axiom Checker
  - [x] Field Axiom Checker 
- [ ] Stage 3: Define self::one and self::zero, ensuring they exist within the set. Traits should be defined more precisely.
  - [x] Element inverse trait with defaut implement
  - [x] Remove `mut checker` new structure should be
    - checker_close_associative(set, op) --> simigroup, 
    - check_exist_id(simigroup) --> monoid, 
    - check_inverse(monoid) --> group,
    - check_commutative(group) --> abelian group
  - [ ] more comment
- [ ] Stage 4:
  - [ ] Element type, eg 'group.sample(0) --> group_element, group_element::new'
  - [ ] Sub group
  - [ ] Gennerator
  - [ ] Order of group, order of element.
  - [ ] Option: default inverse operation of some common operation such like + *
- [ ] Stage 5:
  - [ ] Linear combination
  - [ ] graph example/group product, 
  - [ ] EC Curve example
  - [ ] Intergrate with ZKling
- [ ] Stage 6:
  - [ ] Ring Axiom Checker
  - [ ] Field extension
  - [ ] Polynomial modular
