[![CI](https://github.com/lpenz/copstr/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/copstr/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/copstr/badge.svg?branch=main)](https://coveralls.io/github/lpenz/copstr?branch=main)
[![crates.io](https://img.shields.io/crates/v/copstr)](https://crates.io/crates/copstr)
[![doc.rs](https://docs.rs/copstr/badge.svg)](https://docs.rs/copstr)


# copstr: Yet Another COPy STRing module

copstr's [`Str`] wraps a fixed-size array of `u8` and provides a
string-like interface on top. The size is specified using a const
generic argument.

The internal `u8` array corresponds to UTF-8 encoded `chars`. All
functions guarantee that the contents are valid UTF-8 and return
an error if they are not.

[`Str`]: https://docs.rs/copstr/0/copstr/struct.Str.html
