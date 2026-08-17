# InString - Simple and efficient string interning

[![Crates.io](https://img.shields.io/crates/v/instring?logo=rust&label=Crates.io&labelColor=black)](https://crates.io/crates/instring)
[![Docs.rs](https://img.shields.io/docsrs/instring?logo=rust&label=Docs.rs&labelColor=black)](https://docs.rs/instring/)
[![Built & Test](https://img.shields.io/github/actions/workflow/status/pliden/instring/build-test.yaml?logo=github&label=Build%20%26%20Test&labelColor=black)](https://github.com/pliden/instring/actions/workflows/build-test.yaml)

## Examples

Using the `Intern` trait.

```rust
use instring::Intern;

let a = "hello world".intern();
let b = String::from("hello world").intern();

assert!(a == b);
assert!(a.as_str() == b.as_str());
assert!(std::ptr::eq(a.as_str(), b.as_str()));
```

Using `InString::from()`.

```rust
use instring::InString;

let a = InString::from("hello world");
let b = InString::from(String::from("hello world"));

assert!(a == b);
assert!(a.as_str() == b.as_str());
assert!(std::ptr::eq(a.as_str(), b.as_str()));
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
