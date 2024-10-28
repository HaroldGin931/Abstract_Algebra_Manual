Here’s the corrected version:

Abstract_Algebra_Manual

Learn group, ring, and field structures from a Rust programming language perspective.

Motivation

To understand the algebraic structures commonly used in cryptography, we often need to apply knowledge of abstract algebra. However, this knowledge is typically written in mathematical language, which can present challenges for developers. In essence, though, abstract algebra is about using abstract thinking to analyze and identify properties of mathematical objects, classifying and systematically discussing these properties. Once this perspective is understood, abstract algebra becomes much easier to grasp.

Rust, as a language with many excellent features, allows us to use traits and generics to define these abstract algebraic interfaces. By leveraging the compiler and test functions, we can create an introductory book on abstract algebra from a Rust perspective.

Challenge

The key challenge is that the target audience differs from that of traditional cryptography libraries.

In traditional cryptographic libraries, the operations defined are derived and proven to satisfy the relevant algebraic structure axioms. These libraries primarily focus on implementing these operations/algorithms in code and optimizing them as much as possible.

However, the goal of this project is to provide a set of traits related to algebraic structures, along with instances implemented based on these traits. It also offers exercises for users to try implementing some simple algebraic structures.

The project therefore needs to complete the relevant interfaces and test functions. However, the traits should not be overly restrictive; instead, users should be allowed the space to make mistakes. The test functions will then help check these implementations, showing them which axioms their defined operations fail to satisfy, and prompting them to reflect on the properties of these operations.

Roadmap
- Stage 1: Implement a basic Group trait with finite groups and an axiom-check function.
- Stage 2: Implement a basic Field trait with finite groups.
- Stage 3: Define self::one and self::zero, ensuring they exist within the set. Traits should be defined more precisely.
- Stage 4: TBD (Linear combination, graph example/group product, field extension)
