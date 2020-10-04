use identity_core::did::{Param, DID};
use proptest::prelude::*;
use serde_test::{assert_tokens, Token};

/// test did creation from DID::new
#[test]
fn test_create_did() {
    let did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456".into()],
        ..Default::default()
    }
    .init()
    .unwrap();

    assert_eq!(did.id_segments, vec!["123456"]);
    assert_eq!(did.method_name, "iota");
    assert_eq!(format!("{}", did), "did:iota:123456");
}

/// test a did with multiple id segments.
#[test]
fn test_multiple_ids() {
    let did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456".into(), "789011".into()],
        query: Some(vec![Param::from(("name".into(), Some("value".into())))]),
        ..Default::default()
    }
    .init()
    .unwrap();

    assert_eq!(format!("{}", did), "did:iota:123456:789011?name=value");
}

/// test the DID Param struct.
#[test]
fn test_param() {
    let param = ("name".into(), Some("value".into()));
    let param = Param::from(param);

    assert_eq!(param.key, "name");
    assert_eq!(param.value, Some(String::from("value")));
    assert_eq!(format!("{}", param), "name=value");
}

/// test a did with a fragment.
#[test]
fn test_frag() {
    let mut did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456".into()],
        ..Default::default()
    }
    .init()
    .unwrap();

    did.add_fragment("a-fragment".into());

    assert_eq!(did.fragment, Some(String::from("a-fragment")));
    assert_eq!(format!("{}", did), "did:iota:123456#a-fragment");
}

/// test the params in a DID.
#[test]
fn test_params() {
    let param_a = Param::from(("param".into(), Some("a".into())));
    let param_b = Param::from(("param".into(), Some("b".into())));
    let params = Some(vec![param_a.clone(), param_b.clone()]);

    let mut did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456".into()],
        query: params.clone(),
        ..Default::default()
    }
    .init()
    .unwrap();

    println!("{:?}", did);

    assert_eq!(format!("{}", did), "did:iota:123456?param=a&param=b");
    assert_eq!(did.query, params);

    let param_c = Param::from(("param".into(), Some("c".into())));
    let params = vec![param_c.clone()];
    did.add_query(params);

    assert_eq!(did.query, Some(vec![param_a, param_b, param_c]));
}

/// test a did with path strings.
#[test]
fn test_path_did() {
    let did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456".into()],
        path_segments: Some(vec!["a".into(), "simple".into(), "path".into()]),
        ..Default::default()
    }
    .init()
    .unwrap();

    assert_eq!(format!("{}", did), "did:iota:123456/a/simple/path");
}

/// test a full did with a query, fragment, path and params.
#[test]
fn test_full_did() {
    let param_a = Param::from(("param".into(), Some("a".into())));
    let param_b = Param::from(("param".into(), Some("b".into())));
    let param_c = Param::from(("param".into(), Some("c".into())));
    let params = Some(vec![param_a, param_b, param_c]);

    let did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456".into()],
        query: params,
        path_segments: Some(vec!["some_path".into()]),
        fragment: Some("a_fragment".into()),
    }
    .init()
    .unwrap();

    assert_eq!(
        format!("{}", did),
        "did:iota:123456/some_path?param=a&param=b&param=c#a_fragment"
    );
}

#[test]
fn test_some() {
    let did =
        DID::parse_from_str("did:example:123456789abcdefghi?service=messages&relative-ref=%2Fsome%2Fpath%3Fquery#frag")
            .unwrap();

    println!("{:?}", did);
}

/// test the did parser on a full did.
#[test]
fn test_parser() {
    let did = DID::parse_from_str("did:iota:123456/some_path?param=a&param=b#a_fragment").unwrap();

    assert_eq!(
        format!("{}", did),
        "did:iota:123456/some_path?param=a&param=b#a_fragment"
    );
    assert_eq!(
        did,
        DID {
            method_name: "iota".into(),
            id_segments: vec!["123456".into()],
            query: Some(vec![
                ("param".into(), Some("a".into())).into(),
                ("param".into(), Some("b".into())).into()
            ]),
            path_segments: Some(vec!["some_path".into()]),
            fragment: Some("a_fragment".into()),
        }
        .init()
        .unwrap()
    );
}

/// test multiple path strings in a DID.
#[test]
fn test_multiple_paths() {
    let did = DID::parse_from_str("did:iota:123456/some_path_a/some_path_b").unwrap();

    assert_eq!(format!("{}", did), "did:iota:123456/some_path_a/some_path_b");
    assert_eq!(
        did,
        DID {
            method_name: "iota".into(),
            id_segments: vec!["123456".into()],
            path_segments: Some(vec!["some_path_a".into(), "some_path_b".into()]),
            ..Default::default()
        }
        .init()
        .unwrap()
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
    let did = DID::parse_from_str("did:iota:12345").unwrap();

    assert_tokens(&did, &[Token::String("did:iota:12345")]);

    let param_a = Param::from(("param".into(), Some("a".into())));
    let param_b = Param::from(("param".into(), Some("b".into())));
    let params = Some(vec![param_a, param_b]);

    let did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456".into()],
        path_segments: Some(vec!["some_path".into()]),
        query: params,
        fragment: Some("a_fragment".into()),
    }
    .init()
    .unwrap();

    assert_tokens(
        &did,
        &[Token::String("did:iota:123456/some_path?param=a&param=b#a_fragment")],
    )
}

/// logic for the id segment prop test.
fn inner_did_id_seg(s: &str) -> Option<DID> {
    let did_str = format!("did:iota:{}", s);

    DID::parse_from_str(&did_str).unwrap();

    Some(
        DID {
            method_name: "iota".into(),
            id_segments: vec![s.into()],
            ..Default::default()
        }
        .init()
        .unwrap(),
    )
}

/// logic for the did method name prop test.
fn inner_did_name(s: &str) -> Option<DID> {
    let did_str = format!("did:{}:12345678", s);

    DID::parse_from_str(&did_str).unwrap();

    Some(
        DID {
            method_name: s.into(),
            id_segments: vec!["12345678".into()],
            ..Default::default()
        }
        .init()
        .unwrap(),
    )
}

/// logic for the did params prop test.
fn inner_did_query_params(n: &str, v: &str) -> Option<DID> {
    let did_str = format!("did:iota:12345678?{}={}", n, v);

    DID::parse_from_str(did_str).unwrap();

    Some(
        DID {
            method_name: "iota".into(),
            id_segments: vec!["12345678".into()],
            query: Some(vec![(n.into(), Some(v.into())).into()]),
            ..Default::default()
        }
        .init()
        .unwrap(),
    )
}

/// logic for the did path prop test.
fn inner_did_path(p: &str) -> Option<DID> {
    let did_str = format!("did:iota:12345678/{}", p);

    DID::parse_from_str(did_str).unwrap();

    Some(
        DID {
            method_name: "iota".into(),
            id_segments: vec!["12345678".into()],
            path_segments: Some(vec![p.into()]),
            ..Default::default()
        }
        .init()
        .unwrap(),
    )
}

/// logic for the did fragment prop test.
fn inner_did_frag(f: &str) -> Option<DID> {
    let did_str = format!("did:iota:12345678#{}", f);

    DID::parse_from_str(did_str).unwrap();

    Some(
        DID {
            method_name: "iota".into(),
            id_segments: vec!["12345678".into()],
            fragment: Some(f.into()),
            ..Default::default()
        }
        .init()
        .unwrap(),
    )
}

// Property Based Testing for the DID Parser and DID implementation.
proptest! {
    // set proptest config to 10000 cases.  1000 values will be passed into the proceeding tests.
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    // Run cases that match the regex and are ascii as the id_segment.  Check if the parser accepts them and if the DID can be created with them.
    fn prop_parse_did_id_seg(s in "[a-z0-9A-Z._-]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
        inner_did_id_seg(&s);
    }

    #[test]
    // Run cases that match the regex and are ascii as the method_name.  Check if the parser accepts them and if the DID can be created with them.
    fn prop_parse_did_name(s in "[a-z0-9]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
        inner_did_name(&s);
    }

    #[test]
    // Run cases that match the regex and are ascii as the params.  Check if the parser accepts them and if the DID can be created with them.
    fn prop_parse_did_params(n in "[a-zA-Z0-9.=:-]+", v in "[a-zA-Z0-9.=:-]*".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
        inner_did_query_params(&n, &v);
    }

    #[test]
    // Run cases that match the regex and are ascii as the path_segments.  Check if the parser accepts them and if the DID can be created with them.
    fn prop_parse_did_path(p in "[a-zA-Z0-9._!~$&'()*+;,=:@-]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
        inner_did_path(&p);
    }

    #[test]
    // Run cases that match the regex and are ascii as the fragment.  Check if the parser accepts them and if the DID can be created with them.
    fn prop_parse_did_frag(f in "[a-zA-Z0-9._!~$&'()*+;,=/?:@-]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
        inner_did_frag(&f);
    }
}
