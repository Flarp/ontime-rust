# ontime
Zero-cost abstractions that extend the type system to also consider time-complexity. This is a proof-of-concept, and probably has no real applications.

## Idea
The return value of a function has a definite complexity as long as it uses the `O` struct. Wrappers will be provided for common iterable types.

<a href="https://doc.rust-lang.org/std/collections/index.html#performance">Here are the type complexities for some operations on built-in collections in Rust.</a>

### Constraints

* The type `O` is encapsulating must be a <a href="http://apasel422.github.io/eclectic/eclectic/index.html">Collection</a>.

* `O` structs can only be cast to types of higher complexity
