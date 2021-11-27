// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
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

use core::convert::TryFrom;
use std::error;
use std::fmt;
use std::str;

/// Copy String type
///
/// Fixed-size string-like type that derives Copy. Size is specified
/// via a const generic.
#[derive(Clone, Copy, Debug)]
pub struct Str<const SIZE: usize>([u8; SIZE], usize);

impl<const SIZE: usize> Str<SIZE> {
    /// Returns a new [`Str`] with the contents specified by the
    /// provided string-like entity.
    pub fn new<S: AsRef<str>>(string: S) -> Result<Self, Error> {
        let mut copstr = Self::default();
        copstr.replace(string)?;
        Ok(copstr)
    }

    /// Returns the capacity of this specific [`Str`] type.
    pub fn capacity(&self) -> usize {
        SIZE
    }

    /// Length of the wrapped string in bytes (not in chars).
    pub fn byte_len(&self) -> usize {
        self.1
    }

    /// Adds a char to the end of the string, if possible.
    ///
    /// Returns [`Error`] if the string would overflow.
    pub fn push(&mut self, ch: char) -> Result<(), Error> {
        let mut buffer = [0; 4];
        let result = ch.encode_utf8(&mut buffer).as_bytes();
        if result.len() > self.capacity() - self.byte_len() {
            Err(Error::Overflow)
        } else {
            let fromlen = self.0.split_at_mut(self.1).1;
            let dest = fromlen.split_at_mut(result.len()).0;
            dest.copy_from_slice(result);
            self.1 += result.len();
            Ok(())
        }
    }

    /// Replaces the string in-place.
    pub fn replace<S: AsRef<str>>(&mut self, string: S) -> Result<(), Error> {
        let s = string.as_ref();
        let bytes = s.as_bytes();
        let byteslen = bytes.len();
        if byteslen > self.capacity() {
            Err(Error::Overflow)
        } else {
            let dest = self.0.split_at_mut(byteslen).0;
            dest.copy_from_slice(bytes);
            self.1 = byteslen;
            Ok(())
        }
    }

    /// Extracts a string slice containing the entire `Str`.
    pub fn as_str(&self) -> &str {
        // We can do this because we guarante self.0 is UTF-8
        unsafe { str::from_utf8_unchecked(&self.0[0..self.1]) }
    }
}

impl<const SIZE: usize> Default for Str<SIZE> {
    fn default() -> Self {
        Str([0; SIZE], 0)
    }
}

impl<const SIZE: usize> TryFrom<&str> for Str<SIZE> {
    type Error = Error;
    fn try_from(string: &str) -> Result<Self, Error> {
        Self::new(string)
    }
}

impl<const SIZE: usize> TryFrom<&[u8]> for Str<SIZE> {
    type Error = Error;
    fn try_from(arr: &[u8]) -> Result<Self, Error> {
        let s = str::from_utf8(arr)?;
        Self::new(s)
    }
}

impl<const SIZE: usize> AsRef<str> for Str<SIZE> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const SIZE: usize> AsRef<[u8]> for Str<SIZE> {
    fn as_ref(&self) -> &[u8] {
        &self.0[0..self.byte_len()]
    }
}

impl<const SIZE: usize> fmt::Display for Str<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const SIZE: usize> PartialEq for Str<SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl<const SIZE: usize> Eq for Str<SIZE> {}

/* Errors: **********************************************************/

/// copstr errors enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The provided input overflows the size of the [`Str`]
    Overflow,
    /// The provided input was not valid UTF-8
    Utf8(str::Utf8Error),
}

impl error::Error for Error {}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Overflow => write!(f, "overflow"),
            Error::Utf8(ref e) => write!(f, "{}", e),
        }
    }
}
