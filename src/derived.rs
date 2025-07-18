use crate::{Bstr, BstrRef, MessageId, MessageIdRef, Tstr, TstrRef};

pub type MsgUri = Tstr;
pub type MsgUriRef<'a> = TstrRef<'a>;

#[derive(Debug, Clone, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub struct MessageDerivedValues {
    pub message_id: MessageId,
    pub hub_accepted_timestamp: crate::Timestamp,
    pub mls_group_id: Bstr,
    pub sender_leaf_index: u32,
    pub sender_client_url: MsgUri,
    pub sender_user_url: MsgUri,
    pub room_url: MsgUri,
}

#[derive(Debug, Clone, serde_tuple::Serialize_tuple)]
pub struct MessageDerivedValuesRef<'a> {
    pub message_id: MessageIdRef<'a>,
    pub hub_accepted_timestamp: &'a crate::Timestamp,
    pub mls_group_id: BstrRef<'a>,
    pub sender_leaf_index: &'a u32,
    pub sender_client_url: MsgUriRef<'a>,
    pub sender_user_url: MsgUriRef<'a>,
    pub room_url: MsgUriRef<'a>,
}

impl crate::MimiContentAsRef for MessageDerivedValues {
    type Target<'a> = MessageDerivedValuesRef<'a>;
    fn as_ref(&self) -> Self::Target<'_> {
        MessageDerivedValuesRef {
            message_id: self.message_id.as_ref(),
            hub_accepted_timestamp: &self.hub_accepted_timestamp,
            mls_group_id: self.mls_group_id.as_ref(),
            sender_leaf_index: &self.sender_leaf_index,
            sender_client_url: self.sender_client_url.as_ref(),
            sender_user_url: self.sender_user_url.as_ref(),
            room_url: self.room_url.as_ref(),
        }
    }
}
