use crate::{MessageId, MessageIdRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageBaseStatus {
    Unread = 0,
    Delivered = 1,
    Read = 2,
    Expired = 3,
    Deleted = 4,
    Hidden = 5,
    Error = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum MessageStatus {
    Base(MessageBaseStatus),
    Ext(u8),
}

impl From<u8> for MessageStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Base(MessageBaseStatus::Unread),
            1 => Self::Base(MessageBaseStatus::Delivered),
            2 => Self::Base(MessageBaseStatus::Read),
            3 => Self::Base(MessageBaseStatus::Expired),
            4 => Self::Base(MessageBaseStatus::Deleted),
            5 => Self::Base(MessageBaseStatus::Hidden),
            6 => Self::Base(MessageBaseStatus::Error),
            _ => Self::Ext(value),
        }
    }
}

impl From<MessageStatus> for u8 {
    fn from(value: MessageStatus) -> Self {
        match value {
            MessageStatus::Base(base_status) => base_status as Self,
            MessageStatus::Ext(val) => val,
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple,
)]
pub struct PerMessageStatus {
    pub message_id: MessageId,
    pub status: MessageStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, serde_tuple::Serialize_tuple)]
pub struct PerMessageStatusRef<'a> {
    pub message_id: MessageIdRef<'a>,
    pub status: &'a MessageStatus,
}

impl crate::MimiContentAsRef for PerMessageStatus {
    type Target<'a> = PerMessageStatusRef<'a>;
    fn as_ref(&self) -> Self::Target<'_> {
        PerMessageStatusRef {
            message_id: self.message_id.as_ref(),
            status: &self.status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MessageStatusReport(pub Vec<PerMessageStatus>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct MessageStatusReportRef<'a>(Vec<PerMessageStatusRef<'a>>);

impl crate::MimiContentAsRef for MessageStatusReport {
    type Target<'a> = MessageStatusReportRef<'a>;
    fn as_ref(&self) -> Self::Target<'_> {
        MessageStatusReportRef(self.0.iter().map(PerMessageStatus::as_ref).collect())
    }
}
