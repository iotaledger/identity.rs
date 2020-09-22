use identity_communication::did_comm::*;
#[test]
fn test_create_did_comm() {
    let did_comm = DIDComm {
        id: "123456".into(),
        comm_type: "https://didcomm.org/iota".into(),
        ..Default::default()
    }
    .init()
    .unwrap();

    assert_eq!(did_comm.id, "123456");
    assert_eq!(did_comm.comm_type, "https://didcomm.org/iota");
}
#[test]
fn test_error() {
    let did_comm = DIDComm {
        id: "123456".into(),
        comm_type: "https://didcomm.org/iota".into(),
        ..Default::default()
    }
    .init()
    .unwrap();

    // TODO: Throw and test error
    assert_eq!(did_comm.id, "123456");
}

fn test_id_uniqueness() {
    //unimplemented!();
}
