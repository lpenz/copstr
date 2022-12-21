[![CI](https://github.com/lpenz/copstr/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/copstr/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/copstr/badge.svg?branch=main)](https://coveralls.io/github/lpenz/copstr?branch=main)
[![crates.io](https://img.shields.io/crates/v/copstr)](https://crates.io/crates/copstr)
[![doc.rs](https://docs.rs/copstr/badge.svg)](https://docs.rs/copstr)


# copstr: COPy STRing module using const generic for capacity

[`copstr::Str`] wraps a fixed-size array of `u8` and provides a
string-like interface on top. The size is specified using a const
generic argument.

The internal `u8` array corresponds to UTF-8 encoded `chars`. All
functions guarantee that the contents are valid UTF-8 and return
an error if they are not. Truncation only happens at UTF-8
boundaries.

[`copstr`] is very useful when we want to add a string-like field
to a struct that implements `Copy` but we don't want to give up
this trait.

# Example usage

```rust
use copstr;
use std::convert::TryFrom;

// Create an owned fixed-size string with size 6 *on the stack*:
let mut string = copstr::Str::<6>::try_from("string")?;

// Use it as a regular string:
println!("contents: {}", string);

// Replace the contents with another string that fits the size 6:
string.replace("str")?;

// Append a letter:
string.push('i')?;

// Instead of returning a potential error, we can instead use
// truncating methods:
string.replace_trunc("stringification");
assert_eq!(string.as_str(), "string");

// `copstr::Str` implements Deref<Target=str>, so all `str`
// methods are available:
let split = format!("{:?}", string.split_at(3));
assert_eq!(split, r#"("str", "ing")"#);

// We can add a `copstr` to a struct without having to give up the
// `Copy` trait:
#[derive(Clone, Copy)]
pub struct Mystruct {
    // ...
    comment: copstr::Str<10>,
}

// We can create `copstr` in const contexts:
const TEST: copstr::Str<4> = copstr::Str::<4>::new_const("TEST");

// We check that they fit in const context - the following doesn't compile:
// const TEST_BAD: copstr::Str<3> = copstr::Str::<3>::new_const("TEST");

```

[`copstr`]: https://docs.rs/copstr/0/copstr/
[`copstr::Str`]: https://docs.rs/copstr/0/copstr/struct.Str.html
[`Str`]: https://docs.rs/copstr/0/copstr/struct.Str.html
