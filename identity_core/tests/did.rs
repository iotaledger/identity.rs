use identity_core::{
    did::{Param, DID},
    line_error,
};
use proptest::prelude::*;
use serde_test::{assert_tokens, Token};

/// test did creation from DID::new
#[test]
fn test_create_did() {
    let did = DID::new("iota".into(), vec!["123456".into()], None, None, None, None).expect(line_error!());

    assert_eq!(did.id_segments, vec!["123456"]);
    assert_eq!(did.method_name, "iota");
    assert_eq!(format!("{}", did), "did:iota:123456");
}

/// test a did with multiple id segments.
#[test]
fn test_multiple_ids() {
    let did = DID::new(
        "iota".into(),
        vec!["123456".into(), "789011".into()],
        Some(vec![("name".into(), Some("value".into()))]),
        None,
        None,
        None,
    )
    .expect(line_error!());

    assert_eq!(format!("{}", did), "did:iota:123456:789011;name=value");
}

/// test the DID Param struct.
#[test]
fn test_param() {
    let param = Param::new(("name".into(), Some("value".into()))).expect(line_error!());

    assert_eq!(param.name, "name");
    assert_eq!(param.value, Some(String::from("value")));
    assert_eq!(format!("{}", param), "name=value");
}

/// test a did with a fragment.
#[test]
fn test_frag() {
    let mut did = DID::new("iota".into(), vec!["123456".into()], None, None, None, None).expect(line_error!());

    did.add_fragment("a-fragment".into());

    assert_eq!(did.fragment, Some(String::from("a-fragment")));
    assert_eq!(format!("{}", did), "did:iota:123456#a-fragment");
}

/// test the params in a DID.
#[test]
fn test_params() {
    let param_a = Param::new(("param".into(), Some("a".into()))).expect(line_error!());
    let param_b = Param::new(("param".into(), Some("b".into()))).expect(line_error!());
    let params = Some(vec![param_a.clone(), param_b.clone()]);
    let mut did = DID::new(
        "iota".into(),
        vec!["123456".into()],
        Some(vec![
            ("param".into(), Some("a".into())),
            ("param".into(), Some("b".into())),
        ]),
        None,
        None,
        None,
    )
    .expect(line_error!());

    assert_eq!(format!("{}", did), "did:iota:123456;param=a;param=b");
    assert_eq!(did.params, params);

    let param_c = Param::new(("param".into(), Some("c".into()))).expect(line_error!());
    let params = vec![param_c.clone()];
    did.add_params(params);

    assert_eq!(did.params, Some(vec![param_a, param_b, param_c]));
}

/// test a did with path strings.
#[test]
fn test_path_did() {
    let did = DID::new(
        "iota".into(),
        vec!["123456".into()],
        None,
        Some(vec!["a".into(), "simple".into(), "path".into()]),
        None,
        None,
    )
    .expect(line_error!());

    assert_eq!(format!("{}", did), "did:iota:123456/a/simple/path");
}

/// test a full did with a query, fragment, path and params.
#[test]
fn test_full_did() {
    let did = DID::new(
        "iota".into(),
        vec!["123456".into()],
        Some(vec![
            ("param".into(), Some("a".into())),
            ("param".into(), Some("b".into())),
        ]),
        Some(vec!["some_path".into()]),
        Some("some_query".into()),
        Some("a_fragment".into()),
    )
    .expect(line_error!());

    assert_eq!(
        format!("{}", did),
        "did:iota:123456;param=a;param=b/some_path?some_query#a_fragment"
    );
}

/// test the did parser on a full did.
#[test]
fn test_parser() {
    let did =
        DID::parse_from_str("did:iota:123456;param=a;param=b/some_path?some_query#a_fragment").expect(line_error!());

    assert_eq!(
        format!("{}", did),
        "did:iota:123456;param=a;param=b/some_path?some_query#a_fragment"
    );
    assert_eq!(
        did,
        DID::new(
            "iota".into(),
            vec!["123456".into()],
            Some(vec![
                ("param".into(), Some("a".into())),
                ("param".into(), Some("b".into()))
            ]),
            Some(vec!["some_path".into()]),
            Some("some_query".into()),
            Some("a_fragment".into())
        )
        .expect(line_error!())
    );
}

/// test multiple path strings in a DID.
#[test]
fn test_multiple_paths() {
    let did = DID::parse_from_str("did:iota:123456/some_path_a/some_path_b").expect(line_error!());

    assert_eq!(format!("{}", did), "did:iota:123456/some_path_a/some_path_b");
    assert_eq!(
        did,
        DID::new(
            "iota".into(),
            vec!["123456".into()],
            None,
            Some(vec!["some_path_a".into(), "some_path_b".into()]),
            None,
            None,
        )
        .expect(line_error!())
    )
}

/// test the parsing constraints properly throw errors.
#[test]
fn test_parsing_contraints() {
    let did = DID::parse_from_str("did:IOTA:12345");

    assert!(did.is_err());

    let did = DID::parse_from_str("did:iota:%$^@1234");

    assert!(did.is_err());

    let did = DID::parse_from_str("x:iota:123456");

    assert!(did.is_err());
}

/// test DID serialization and deserialization.
#[test]
fn test_serde() {
    let did = DID::parse_from_str("did:iota:12345").expect(line_error!());

    assert_tokens(&did, &[Token::String("did:iota:12345")]);

    let did = DID::new(
        "iota".into(),
        vec!["123456".into()],
        Some(vec![
            ("param".into(), Some("a".into())),
            ("param".into(), Some("b".into())),
        ]),
        Some(vec!["some_path".into()]),
        Some("some_query".into()),
        Some("a_fragment".into()),
    )
    .expect(line_error!());

    assert_tokens(
        &did,
        &[Token::String(
            "did:iota:123456;param=a;param=b/some_path?some_query#a_fragment",
        )],
    )
}

/// logic for the id segment prop test.
fn wrapper_did_id_seg(s: &str) -> Option<DID> {
    let did_str = format!("did:iota:{}", s);

    DID::parse_from_str(&did_str).expect(line_error!());

    Some(DID::new("iota".into(), vec![s.into()], None, None, None, None).expect(line_error!()))
}

/// logic for the did method name prop test.
fn wrapper_did_name(s: &str) -> Option<DID> {
    let did_str = format!("did:{}:12345678", s);

    DID::parse_from_str(&did_str).expect(line_error!());

    Some(DID::new(s.into(), vec!["12345678".into()], None, None, None, None).expect(line_error!()))
}

/// logic for the did params prop test.
fn wrapper_did_params(n: &str, v: &str) -> Option<DID> {
    let did_str = format!("did:iota:12345678;{}={}", n, v);

    DID::parse_from_str(did_str).expect(line_error!());

    Some(
        DID::new(
            "iota".into(),
            vec!["12345678".into()],
            Some(vec![(n.into(), Some(v.into()))]),
            None,
            None,
            None,
        )
        .expect(line_error!()),
    )
}

/// logic for the did path prop test.
fn wrapper_did_path(p: &str) -> Option<DID> {
    let did_str = format!("did:iota:12345678/{}", p);

    DID::parse_from_str(did_str).expect(line_error!());

    Some(
        DID::new(
            "iota".into(),
            vec!["12345678".into()],
            None,
            Some(vec![p.into()]),
            None,
            None,
        )
        .expect(line_error!()),
    )
}

/// logic for the did query prop test.
fn wrapper_did_query(q: &str) -> Option<DID> {
    let did_str = format!("did:iota:12345678?{}", q);

    DID::parse_from_str(did_str).expect(line_error!());

    Some(DID::new("iota".into(), vec!["12345678".into()], None, None, Some(q.into()), None).expect(line_error!()))
}

/// logic for the did fragment prop test.
fn wrapper_did_frag(f: &str) -> Option<DID> {
    let did_str = format!("did:iota:12345678#{}", f);

    DID::parse_from_str(did_str).expect(line_error!());

    Some(DID::new("iota".into(), vec!["12345678".into()], None, None, None, Some(f.into())).expect(line_error!()))
}

// Property Based Testing for the DID Parser and DID implementation.
proptest! {
    // set proptest config to 10000 cases.  10000 values will be passed into the proceeding tests.
    #![proptest_config(ProptestConfig::with_cases(10000))]
    #[test]
    // Run cases that match the regex and are ascii as the id_segment.  Check if the parser accepts them and if the DID can be created with them.
    fn prop_parse_did_id_seg(s in "[a-z0-9A-Z._-]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
        wrapper_did_id_seg(&s);
    }

    #[test]
    // Run cases that match the regex and are ascii as the method_name.  Check if the parser accepts them and if the DID can be created with them.
    fn prop_parse_did_name(s in "[a-z0-9]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
        wrapper_did_name(&s);
    }

    #[test]
    // Run cases that match the regex and are ascii as the params.  Check if the parser accepts them and if the DID can be created with them.
    fn prop_parse_did_params(n in "[a-zA-Z0-9.=:-]+", v in "[a-zA-Z0-9.=:-]*".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
        wrapper_did_params(&n, &v);
    }

    #[test]
    // Run cases that match the regex and are ascii as the path_segments.  Check if the parser accepts them and if the DID can be created with them.
    fn prop_parse_did_path(p in "[a-zA-Z0-9._!~$&'()*+;,=:@-]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
        wrapper_did_path(&p);
    }

    #[test]
    // Run cases that match the regex and are ascii as the query.  Check if the parser accepts them and if the DID can be created with them.
    fn prop_parse_did_query(q in "[a-zA-Z0-9._!~$&'()*+;,=/?:@-]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
        wrapper_did_query(&q);
    }

    #[test]
    // Run cases that match the regex and are ascii as the fragment.  Check if the parser accepts them and if the DID can be created with them.
    fn prop_parse_did_frag(f in "[a-zA-Z0-9._!~$&'()*+;,=/?:@-]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
        wrapper_did_frag(&f);
    }
}
