use identity_common::{line_error, object, Timestamp};

use std::convert::TryFrom;

macro_rules! error_hack {
    ($expr:expr) => {
        match $expr {
            Ok(_) => {}
            Err(error) => panic!("{}", error),
        }
    };
}

#[test]
fn test_macro_empty() {
    assert!(object!().is_empty());
    assert_eq!(object!(), object!());
}

#[test]
fn test_macro_fields() {
    let obj = object!(foo: 1, bar: 2);

    assert_eq!(obj.len(), 2);
    assert_eq!(obj["foo"], 1);
    assert_eq!(obj["bar"], 2);
}

#[test]
fn test_parse_valid() {
    let original = "2020-01-01T00:00:00Z";
    let timestamp = Timestamp::try_from(original).expect(line_error!());

    assert_eq!(timestamp.to_rfc3339(), original);

    let original = "1980-01-01T12:34:56Z";
    let timestamp = Timestamp::try_from(original).expect(line_error!());

    assert_eq!(timestamp.to_rfc3339(), original);
}

#[test]
#[should_panic = "premature end of input"]
fn test_parse_invalid_empty() {
    error_hack!(Timestamp::try_from(""));
}

#[test]
#[should_panic = "input contains invalid characters"]
fn test_parse_invalid_bad_date() {
    error_hack!(Timestamp::try_from("foo bar"));
}

#[test]
#[should_panic = "input contains invalid characters"]
fn test_parse_invalid_bad_fmt() {
    error_hack!(Timestamp::try_from("2020/01/01 03:30:16"));
}
