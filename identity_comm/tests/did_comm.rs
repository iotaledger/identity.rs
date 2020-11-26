use identity_comm::{did_comm_builder::*, messages::MessageType};
#[test]
fn test_create_did_comm() {
    let did_comm = DIDCommBuilder::new()
        .id("123456")
        .comm_type(MessageType::TrustPing)
        .build()
        .unwrap();

    // println!("{:?}", did_comm);
    assert_eq!(did_comm.id, "123456");
    // assert_eq!(did_comm.comm_type, "https://didcomm.org/iota");
}
// #[test]
// fn test_error() {
//     let did_comm = DIDComm {
//         id: "123456".into(),
//         comm_type: "https://didcomm.org/iota".into(),
//         // ..Default::default()
//     }
//     .init()
//     .unwrap();

//     // TODO: Throw and test error
//     assert_eq!(did_comm.id, "123456");
// }

// fn test_id_uniqueness() {
//     //unimplemented!();
// }
