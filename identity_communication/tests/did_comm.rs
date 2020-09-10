
use identity_communication::did_comm::DIDComm;
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

fn test_id_uniqueness() {
    //unimplemented!();
}