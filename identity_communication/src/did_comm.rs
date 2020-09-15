use identity_core::did::DID;
#[derive(Debug, PartialEq, Default, Eq, Clone)]
pub struct DIDComm {
    pub id: String, // MUST be unique to the sender
    pub comm_type: String, // MUST be a valid Message Type URI
    pub from: Option<DID>, // MUST be a string that is a valid DID which identifies the sender of the message
    pub to: Option<Vec<DID>>,// MUST be an array of strings where each element is a valid DID
    pub created_at: Option<String>, // expressed in UTC Epoch Seconds (example: 1599692536) -> https://github.com/iotaledger/identity.rs/blob/952e8c86ff58954b15bb2a4964dfb7b6fe39b122/identity_core/src/common/timestamp.rs
    pub expires_at: Option<String>, // expressed in UTC Epoch Seconds (example: 1599692536) -> https://github.com/iotaledger/identity.rs/blob/952e8c86ff58954b15bb2a4964dfb7b6fe39b122/identity_core/src/common/timestamp.rs
    pub body: Option<String>, // Here can be everything
}

impl DIDComm {
    /// Initializes the DIDComm struct with the filled out fields.
    pub fn init(self) -> crate::Result<DIDComm> {
        let did_comm = DIDComm {
            id: self.id,
            comm_type: self.comm_type,
            from: self.from,
            to: self.to,
            created_at: self.created_at,
            expires_at: self.expires_at,
            body: self.body
        };
        Ok(did_comm)
    }
}