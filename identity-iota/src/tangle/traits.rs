use crate::tangle::MessageId;

pub trait TangleRef {
    fn message_id(&self) -> &MessageId;

    fn set_message_id(&mut self, message_id: MessageId);

    fn previous_message_id(&self) -> &MessageId;

    fn set_previous_message_id(&mut self, message_id: MessageId);
}
