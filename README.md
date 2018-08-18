# ontime
Zero-cost abstractions that extend the type system to also consider time-complexity. This is a proof-of-concept, and probably has no real applications.

## Idea
The return value of a function has a definite complexity as long as it uses the `O` struct. Wrappers will be provided for common iterable types.

<a href="https://doc.rust-lang.org/std/collections/index.html#performance">Here are the type complexities for some operations on built-in collections in Rust.</a>
```rust
// will compile
fn get_o_hash(hash: O<N<U1>, HashMap<O<N<U1>, u8>, O<N<U1>, u8>>>) -> O<N, O<N<U1>, u8>> {
  // the type signature of 'get' is O<Const, _>, but will be
  // upgraded to O<N, _>, as it is less complex
  hash.get(15).unwrap().clone();
}

// container types cannot contain anything other than O (
fn walk_nested_nest(lists: O<N<U1>, Vec<O<N, Vec<u8>>>) -> O<N<U1>, u32> {
  let z:  = walk
}
```
