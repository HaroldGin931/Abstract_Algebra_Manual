# Abstract_Algebra_Manual
Learn group ring and field from a rust programming language perspective.

## Motvation
To understand the algebraic structures commonly used in cryptography, we often need to apply knowledge of abstract algebra. However, this knowledge is typically written in mathematical language, which can present challenges for developers. In essence, though, abstract algebra is about using abstract thinking to analyze and identify properties of mathematical objects, classifying and systematically discussing these properties. Once this perspective is understood, abstract algebra becomes much easier to grasp.

Rust, as a language with many excellent features, allows us to use traits and generics to define these abstract algebraic interfaces. By leveraging the compiler and test function, we can create an introductory book on abstract algebra from a prespective from Rust.


## Challage
The ket challage is the target audience are different with the current cryptography library.

In traditional cryptographic libraries, the operations defined are derived and proven to satisfy the relevant algebraic structure axioms. These libraries primarily focus on implementing these operations/algorithms in code and optimizing them as much as possible.

However, the goal of this project is to provide a set of traits related to algebraic structures, along with instances implemented based on these traits. It also offers exercises for users to try implementing some simple algebraic structures.

The project therefore needs to complete the relevant interfaces and test functions. However, the traits should not be overly restrictive; instead, users should be allowed the space to make mistakes. The test functions will then help check these implementations, showing them which axioms their defined operations fail to satisfy, and prompting them to reflect on the properties of these operations.

## Roadmap
Stage 1: Implement basic Group trait with finite Group and Axiom check function.
Stage 2: Implement basic Field trait with finite Group 
Stage 3: Define self::one and self::zero, those should be checked thate they are exist in the set or not. (Trait should be defined more precisily.
Stage 4: TBD (Linear combation, graph example/group product, field extention)