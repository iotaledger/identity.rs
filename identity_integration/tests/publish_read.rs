use identity_core::document::DIDDocument;
use identity_integration::tangle_reader::TangleReader;
use identity_integration::tangle_writer::{iota_network, DIDMessage, Payload, TangleWriter};

#[smol_potat::test]
async fn test_publish_read() {
  smol::run(async {
    let node = String::from("https://nodes.comnet.thetangle.org:443");
    let did_root_address = "XVORZASIIP9RCYMREUIXXVPQIPHVCNPQXHZVYKFWYCZRX9JQKG9REPKIASAUUECOSQO9JT9XNMVKWYGVA";
    // 1. Publish demo DID document to the Tangle
    let tangle_writer = TangleWriter {
      node: node.clone(),
      network: iota_network::Comnet,
    };
    let did_doc = DIDDocument::new(
      String::from("https://www.w3.org/ns/did/v1"),
      String::from("did:iota:123456789abcdefghi"),
    )
    .unwrap();
    let did_message = DIDMessage {
      payload: Payload::DIDDocument(serde_json::to_string(&did_doc).unwrap()),
      // payload: Payload::DIDDocument(String::from("Document")),
      address: String::from(did_root_address),
    };
    let _bundle_trytes = tangle_writer.publish_document(&did_message).await.unwrap();

    // Get confirmation status, promote and reattach
    // let (tx_hash, _confirmed) = tangle_writer.is_confirmed(*bundle_trytes[0].bundle()).await.unwrap();
    // tangle_writer.promote(tx_hash).await.unwrap();
    // tangle_writer.reattach(bundle_trytes).await.unwrap();

    // 2. Fetch messages from DID root address
    let tangle_reader = TangleReader { node: node };
    let received_message = tangle_reader.fetch(did_root_address).await.unwrap();
    // Check if sent message is the same as the received one
    let fetched_did_message: DIDMessage = serde_json::from_str(&received_message[0]).unwrap();
    assert_eq!(did_message, fetched_did_message);
  })
}
