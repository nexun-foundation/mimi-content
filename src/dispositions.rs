/// Mimi content's base content dispositions
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
)]
#[repr(u8)]
pub enum BaseDispos {
    #[default]
    Unspecified = 0,
    Render = 1,
    Reaction = 2,
    Profile = 3,
    Inline = 4,
    Icon = 5,
    Attachment = 6,
    Session = 7,
    Preview = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum Disposition {
    Base(BaseDispos),
    Ext(u8),
}

impl Default for Disposition {
    fn default() -> Self {
        Self::Base(BaseDispos::default())
    }
}

impl From<u8> for Disposition {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Base(BaseDispos::Unspecified),
            1 => Self::Base(BaseDispos::Render),
            2 => Self::Base(BaseDispos::Reaction),
            3 => Self::Base(BaseDispos::Profile),
            4 => Self::Base(BaseDispos::Inline),
            5 => Self::Base(BaseDispos::Icon),
            6 => Self::Base(BaseDispos::Attachment),
            7 => Self::Base(BaseDispos::Session),
            8 => Self::Base(BaseDispos::Preview),
            _ => Self::Ext(value),
        }
    }
}

impl From<Disposition> for u8 {
    fn from(value: Disposition) -> Self {
        match value {
            Disposition::Base(base_dispo) => base_dispo as Self,
            Disposition::Ext(ext_dispo) => ext_dispo,
        }
    }
}
impl From<BaseDispos> for u8 {
    fn from(value: BaseDispos) -> Self {
        value as Self
    }
}

impl PartialEq<u8> for Disposition {
    fn eq(&self, other: &u8) -> bool {
        u8::from(*self) == *other
    }
}

impl PartialEq<u8> for BaseDispos {
    fn eq(&self, other: &u8) -> bool {
        u8::from(*self) == *other
    }
}
