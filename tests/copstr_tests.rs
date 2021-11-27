// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use copstr;

use std::convert::TryFrom;
use std::str;

use anyhow::Result;

type Str = copstr::Str<5>;

#[macro_export]
macro_rules! assert_matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("assertion failed: `{:?}` does not match `{}`",
                            e, stringify!($($pattern)+)),
        }
    }
}

/* Basic tests */

fn assert_str(copstr: &Str, s: &str) {
    assert_eq!(copstr.byte_len(), s.as_bytes().len());
    assert_eq!(copstr.as_str(), s);
    assert_eq!(copstr.as_ref() as &str, s);
    assert_eq!(copstr.as_ref() as &[u8], s.as_bytes());
    assert_eq!(copstr.to_string(), s.to_string());
    assert_eq!(copstr, &Str::try_from(s).unwrap());
    assert_eq!(copstr, &Str::new(s).unwrap());
}

#[test]
fn test_basic() -> Result<()> {
    let mut basic = Str::new("basic")?;
    assert_eq!(basic.capacity(), 5);
    assert_str(&basic, "basic");
    basic.replace("basic")?;
    assert_str(&basic, "basic");
    // Test copy:
    let basic2 = basic;
    assert_str(&basic2, "basic");
    // Test clone:
    let basic3 = basic.clone();
    assert_str(&basic3, "basic");
    Ok(())
}

#[test]
fn test_push() -> Result<()> {
    let mut no = Str::new("no")?;
    no.push('w')?;
    assert_str(&no, "now");
    Ok(())
}

#[test]
fn test_push_err() -> Result<()> {
    let mut basic = Str::new("basic")?;
    let result = basic.push('s');
    if let Err(e) = &result {
        eprintln!("{:?} {}", result, e);
    }
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), copstr::Error::Overflow);
    assert_str(&basic, "basic");
    Ok(())
}

#[test]
fn test_tryfrom_err() -> Result<()> {
    assert_eq!(
        Str::try_from("string").unwrap_err(),
        copstr::Error::Overflow
    );
    assert_eq!(
        Str::try_from("length".as_bytes()).unwrap_err(),
        copstr::Error::Overflow
    );
    Ok(())
}

#[test]
fn test_replace() -> Result<()> {
    let mut no = Str::new("yes")?;
    no.replace("no")?;
    assert_str(&no, "no");
    Ok(())
}

#[test]
fn test_replace_err() -> Result<()> {
    let mut basic = Str::new("basic")?;
    let result = basic.replace("string");
    assert_eq!(result.unwrap_err(), copstr::Error::Overflow);
    assert_str(&basic, "basic");
    Ok(())
}

const SPARKLE_HEART: [u8; 4] = [240, 159, 146, 150];
const INVALID_UTF8: [u8; 4] = [0, 159, 146, 150];

#[test]
fn test_utf8_tryfrom() -> Result<()> {
    assert!(Str::try_from(&SPARKLE_HEART as &[u8]).is_ok());
    Ok(())
}

#[test]
fn test_utf8_push() -> Result<()> {
    let mut space = Str::new(" ")?;
    let s = str::from_utf8(&SPARKLE_HEART)?;
    space.push(s.chars().nth(0).unwrap())?;
    assert_str(&space, " \u{1F496}");
    Ok(())
}

#[test]
fn test_utf8_replace() -> Result<()> {
    let mut space = Str::new(" ")?;
    let s = str::from_utf8(&SPARKLE_HEART)?;
    space.replace(s)?;
    assert_str(&space, "\u{1F496}");
    Ok(())
}

#[test]
fn test_utf8_invalid_tryfrom() -> Result<()> {
    let result = Str::try_from(&INVALID_UTF8 as &[u8]);
    eprintln!("{}", result.clone().unwrap_err());
    assert_matches!(result, Err(copstr::Error::Utf8(_)));
    Ok(())
}

#[test]
fn test_utf8_invalid_replace() -> Result<()> {
    let mut basic = Str::new("basic")?;
    let s = format!("ch{}", str::from_utf8(&SPARKLE_HEART)?);
    assert_eq!(basic.replace(s).unwrap_err(), copstr::Error::Overflow);
    assert_str(&basic, "basic");
    Ok(())
}
