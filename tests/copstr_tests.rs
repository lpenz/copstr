// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use copstr;

use std::convert::TryFrom;

use anyhow::Result;

type Str = copstr::Str<5>;

/* Basic tests */

#[test]
fn test_basic() -> Result<()> {
    let yes = Str::new("yes")?;
    assert_eq!(yes.capacity(), 5);
    assert_eq!(yes.byte_len(), 3);
    assert_eq!(yes.as_str(), "yes");
    assert_eq!(yes.as_ref() as &str, "yes");
    assert_eq!(yes.as_ref() as &[u8], [b'y', b'e', b's']);
    assert_eq!(yes.to_string(), "yes".to_string());
    let mut no = yes;
    no.replace("no")?;
    assert_eq!(no.as_str(), "no");
    assert_eq!(no, Str::new("no")?);
    no.push('w')?;
    assert_eq!(no.as_str(), "now");
    let mut basic = no.clone();
    basic.replace("basic")?;
    assert_eq!(basic.as_str(), "basic");
    let result = basic.push('s');
    if let Err(e) = &result {
        eprintln!("{:?} {}", result, e);
    }
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), copstr::Error::Overflow);
    assert_eq!(basic.as_str(), "basic");
    assert_eq!(
        Str::try_from("string").unwrap_err(),
        copstr::Error::Overflow
    );
    assert_eq!(
        Str::try_from("length".as_bytes()).unwrap_err(),
        copstr::Error::Overflow
    );
    let sparkle_heart = vec![240, 159, 146, 150];
    assert!(Str::try_from(sparkle_heart.as_slice()).is_ok());
    let invalid = vec![0, 159, 146, 150];
    let result = Str::try_from(invalid.as_slice());
    if let Err(e) = &result {
        eprintln!("{:?} {}", result, e);
    }
    assert!(result.is_err());
    Ok(())
}
