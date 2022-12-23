// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(rust_2018_idioms)]
#![warn(missing_docs)]

//! copstr is a COPy STRing module
//!
//! [`copstr::Str`] wraps a fixed-size array of `u8` and provides a
//! string-like interface on top. The size is specified using a const
//! generic argument.
//!
//! The internal `u8` array corresponds to UTF-8 encoded `chars`. All
//! functions guarantee that the contents are valid UTF-8 and return
//! an error if they are not. Truncation only happens at UTF-8
//! boundaries.
//!
//! [`copstr`] is very useful when we want to add a string-like field
//! to a struct that implements `Copy` but we don't want to give up
//! this trait.
//!
//! # Example usage
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use copstr;
//! use std::convert::TryFrom;
//!
//! // Create an owned fixed-size string with size 6 *on the stack*:
//! let mut string = copstr::Str::<6>::try_from("string")?;
//!
//! // Use it as a regular string:
//! println!("contents: {}", string);
//!
//! // Replace the contents with another string that fits the size 6:
//! string.replace("str")?;
//!
//! // Append a letter:
//! string.push('i')?;
//!
//! // Instead of returning a potential error, we can instead use
//! // truncating methods:
//! string.replace_trunc("stringification");
//! assert_eq!(string.as_str(), "string");
//!
//! // `copstr::Str` implements Deref<Target=str>, so all `str`
//! // methods are available:
//! let split = format!("{:?}", string.split_at(3));
//! assert_eq!(split, r#"("str", "ing")"#);
//!
//! // We can add a `copstr` to a struct without having to give up the
//! // `Copy` trait:
//! #[derive(Clone, Copy)]
//! pub struct Mystruct {
//!     // ...
//!     comment: copstr::Str<10>,
//! }
//!
//! // We can (and should) create a type alias:
//! type MyStr = copstr::Str::<4>;
//!
//! // We can create `copstr` in const contexts:
//! const TEST: MyStr = MyStr::new_const("TEST");
//! # Ok(()) }
//! ```
//!
//! When using a const context, strings that don't fit generate a
//! compilation error. For instance, the following doesn't compile:
//!
//! ```compile_fail
//! const TEST_BAD: copstr::Str<3> = copstr::Str::<3>::new_const("TEST");
//! ```
//!

pub mod copstr;
pub use self::copstr::*;
