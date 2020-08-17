
use identity_vc::vc::VerifiableCredential;

#[test]
fn test_create_vc() {
    let vc = VerifiableCredential::new("test".to_owned());

    assert_eq!(vc.name, "test");

}