use identity_integration::tangle_reader::TangleReader;
use identity_integration::tangle_writer::TangleWriter;

#[smol_potat::test]
async fn test_publish_read() {
  smol::run(async {
    let node = String::from("https://nodes.comnet.thetangle.org:443");
    let did_root_address = "XVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA";
    // 1. Publish demo DID Document to the Tangle
    let tangle_writer = TangleWriter {
      node: node.clone(),
      network: iota::client::builder::Network::Comnet,
    };
    let sent_message = String::from("Test");
    let sent_message_option = Some(sent_message.clone());
    tangle_writer
      .publish_document(did_root_address, sent_message_option)
      .await
      .unwrap();

    // 2. Fetch messages from DID root address
    let tangle_reader = TangleReader { node: node };
    let received_message = tangle_reader.fetch(did_root_address).await.unwrap();
    // Check if sent message is the same as the recieved one
    assert_eq!(sent_message, received_message);
  })
}
