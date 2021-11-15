// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(rust_2018_idioms)]
#![warn(missing_docs)]

//! copstr is Yet Another COPy STRing module
//!
//! copstr's [`Str`] wraps a fixed-size array of `u8` and provides a
//! string-like interface on top. The size is specified using a const
//! generic argument.
//!
//! The internal `u8` array corresponds to UTF-8 encoded `chars`. All
//! functions guarantee that the contents are valid UTF-8 and return
//! an error if they are not.

mod copstr;
pub use self::copstr::*;
