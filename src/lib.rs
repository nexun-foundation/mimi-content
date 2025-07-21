#![warn(clippy::all)]

mod common;
pub mod delivery_report;
pub mod derived;
mod dispositions;
#[cfg(feature = "gfm-mimi")]
pub mod gfm_mimi;
mod message_id;
mod nested_part;
// mod rfc9581; // WIP: this is complex and should probably live in another crate altogether

pub mod reexports {
    pub use ciborium;
    #[cfg(feature = "gfm-mimi")]
    pub use comrak;
}
pub use common::*;
pub use dispositions::*;
pub use message_id::*;
pub use nested_part::*;

use indexmap::IndexMap;

pub const MIMI_CONTENT_MIME: &str = "application/mimi-content";
pub const MIMI_CONTENT_MESSAGE_STATUS_MIME: &str = "application/mimi-message-status";

pub trait MimiContentStandardExtension {
    const EXTENSION_KEY_INT: i64;
    const EXTENSION_KEY: Name = Name::Int(Self::EXTENSION_KEY_INT);
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
#[serde(untagged)]
pub enum Name {
    Int(i64),
    Str(Tstr),
}

impl MimiContentAsRef for Name {
    type Target<'a> = NameRef<'a>;

    fn as_ref(&self) -> Self::Target<'_> {
        match self {
            Self::Int(int) => NameRef::Int(int),
            Self::Str(tstr) => NameRef::Str(tstr.as_ref()),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Serialize)]
#[repr(u8)]
#[serde(untagged)]
pub enum NameRef<'a> {
    Int(&'a i64),
    Str(TstrRef<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Value(ciborium::value::CanonicalValue);

impl From<ciborium::Value> for Value {
    fn from(value: ciborium::Value) -> Self {
        Self(ciborium::value::CanonicalValue::from(value))
    }
}

impl MimiContentAsRef for Value {
    type Target<'a> = ValueRef<'a>;

    fn as_ref(&self) -> Self::Target<'_> {
        ValueRef(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct ValueRef<'a>(&'a Value);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Timestamp {
    MsecsSinceEpoch(u64),
    ExtendedTime(ciborium::tag::Required<IndexMap<Name, Value>, 1001>), // TODO implement RFC9581
}

pub type MimiContentSalt = [u8; 16];

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_tuple::Serialize_tuple,
    serde_tuple::Deserialize_tuple,
)]
pub struct Expiration {
    pub relative: bool,
    pub time: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SenderUriExtension(Tstr);

impl MimiContentStandardExtension for SenderUriExtension {
    const EXTENSION_KEY_INT: i64 = 1;
}

impl std::ops::Deref for SenderUriExtension {
    type Target = Tstr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RoomUriExtension(Tstr);

impl MimiContentStandardExtension for RoomUriExtension {
    const EXTENSION_KEY_INT: i64 = 2;
}

impl std::ops::Deref for RoomUriExtension {
    type Target = Tstr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    serde_tuple::Serialize_tuple,
    serde_tuple::Deserialize_tuple,
)]
pub struct MimiContent {
    #[serde(with = "serde_bytes")]
    salt: MimiContentSalt,
    pub replaces: Option<MessageId>,
    pub topic_id: Bstr,
    pub expires: Option<Expiration>,
    pub in_reply_to: Option<MessageId>,
    pub extensions: IndexMap<Name, Value>,
    pub nested_part: NestedPart,
}

#[bon::bon]
impl MimiContent {
    #[builder]
    pub fn new(
        #[builder(field)] extensions: IndexMap<Name, Value>,
        #[builder(field)] salt: MimiContentSalt,
        topic_id: Bstr,
        expires: Option<Expiration>,
        nested_part: NestedPart,
        replaces: Option<MessageId>,
        in_reply_to: Option<MessageId>,
    ) -> Self {
        Self {
            salt,
            replaces,
            topic_id,
            expires,
            in_reply_to,
            extensions,
            nested_part,
        }
    }
}

#[allow(dead_code)]
impl<S: mimi_content_builder::State> MimiContentBuilder<S> {
    pub fn with_extension(mut self, name: Name, value: Value) -> Self {
        self.extensions.insert(name, value);
        self
    }

    pub fn with_sender_uri(self, sender_uri: String) -> Self {
        self.with_extension(
            SenderUriExtension::EXTENSION_KEY,
            ciborium::Value::Text(sender_uri).into(),
        )
    }

    pub fn with_room_uri(self, room_uri: String) -> Self {
        self.with_extension(
            RoomUriExtension::EXTENSION_KEY,
            ciborium::Value::Text(room_uri).into(),
        )
    }

    pub fn salt_with_rng(mut self, salt_csprng: &mut dyn rand_core::CryptoRngCore) -> Self {
        self.salt.fill(0); // Erase the salt for good measure
        salt_csprng.fill_bytes(&mut self.salt);
        self
    }

    pub fn salt_from_outside_entropy(mut self, salt: MimiContentSalt) -> Self {
        self.salt = salt;
        self
    }
}

impl MimiContent {
    /// Hashes the CBOR bytes of `self`
    pub fn hash<H: digest::Digest>(&self) -> Result<Bstr, MimiContentError> {
        let bytes = self.to_cbor_bytes()?;
        let hash = H::digest(&bytes);
        Ok(hash.to_vec().into())
    }

    pub fn get_extension<T: for<'de> serde::de::Deserialize<'de>>(&self, name: &Name) -> Option<T> {
        self.extensions.get(name).and_then(|ext_value| {
            ciborium::Value::from(ext_value.0.clone())
                .deserialized()
                .ok()
        })
    }

    #[inline]
    pub fn get_sender_uri(&self) -> Option<SenderUriExtension> {
        self.get_extension(&SenderUriExtension::EXTENSION_KEY)
    }

    #[inline]
    pub fn get_room_uri(&self) -> Option<RoomUriExtension> {
        self.get_extension(&RoomUriExtension::EXTENSION_KEY)
    }

    #[cfg(feature = "franking-tag")]
    /// Calculation of the franking_tag as described in mimi-protocol <https://www.ietf.org/archive/id/draft-ietf-mimi-protocol-03.html#name-client-creation-and-sending>
    /// This should belong in mimi-content as noted in this issue <https://github.com/ietf-wg-mimi/mimi-protocol/issues/91> so it is implemented here under a feature flag
    pub fn calculate_franking_tag(&self) -> Result<FrankingTag, MimiContentError> {
        use hmac::Mac as _;
        let mut hmac = hmac::SimpleHmac::<sha2::Sha256>::new_from_slice(&self.salt).unwrap(); // SAFETY: HMAC can take keys of any size because of OPAD

        hmac.update(&self.to_cbor_bytes()?);

        let franking_tag = hmac.finalize().into_bytes();
        Ok(FrankingTag(franking_tag.into()))
    }

    /// Fetch the topic id, returning `None` if the
    /// field is empty
    pub fn topic_id(&self) -> Option<&Bstr> {
        if self.topic_id.is_empty() {
            None
        } else {
            Some(&self.topic_id)
        }
    }
}

#[cfg(feature = "franking-tag")]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FrankingTag(pub [u8; 32]);

#[cfg(feature = "franking-tag")]
impl FrankingTag {
    #[inline]
    pub fn into_inner(self) -> [u8; 32] {
        self.0
    }

    #[inline]
    pub fn to_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde_tuple::Serialize_tuple)]
pub struct MimiContentRef<'a> {
    #[serde(with = "serde_bytes")]
    pub salt: &'a MimiContentSalt,
    pub replaces: Option<MessageIdRef<'a>>,
    pub topic_id: BstrRef<'a>,
    pub expires: Option<&'a Expiration>,
    pub in_reply_to: Option<MessageIdRef<'a>>,
    pub extensions: IndexMap<NameRef<'a>, ValueRef<'a>>,
    pub nested_part: NestedPartRef<'a>,
}

impl MimiContentAsRef for MimiContent {
    type Target<'a> = MimiContentRef<'a>;
    fn as_ref(&self) -> Self::Target<'_> {
        MimiContentRef {
            salt: &self.salt,
            replaces: self.replaces.as_ref().map(MessageId::as_ref),
            topic_id: self.topic_id.as_ref(),
            expires: self.expires.as_ref(),
            in_reply_to: self.in_reply_to.as_ref().map(MessageId::as_ref),
            extensions: self
                .extensions
                .iter()
                .map(|(k, v)| (k.as_ref(), v.as_ref()))
                .collect(),
            nested_part: self.nested_part.as_ref(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MimiContentError {
    #[error(transparent)]
    SerializeError(#[from] ciborium::ser::Error<std::io::Error>),
    #[error(transparent)]
    DeserializeError(#[from] ciborium::de::Error<std::io::Error>),
    #[error("The provided Hash Algorithm ({0:2X}) is not supported")]
    UnsupportedMessageIdHashAlg(u8),
    #[error("The provided Hash Algorithm ({0:?}) is unknown")]
    UnknownMessageIdHashAlg(Option<u8>),
    #[error("The custom Hash Algorithm is out of the custom range (64..u8::MAX)")]
    CustomMessageIdHashAlgOutOfRange(u8),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub trait MimiContentAsRef {
    type Target<'a>
    where
        Self: 'a;

    fn as_ref(&self) -> Self::Target<'_>;
}

pub trait MimiContentSerialize: serde::Serialize {
    fn to_cbor_bytes(&self) -> Result<Vec<u8>, MimiContentError> {
        let mut buf = vec![];
        self.to_cbor_bytes_into(&mut buf)?;
        Ok(buf)
    }

    fn to_cbor_bytes_into(&self, buf: &mut Vec<u8>) -> Result<(), MimiContentError> {
        ciborium::into_writer(self, buf)?;
        Ok(())
    }
}

pub trait MimiContentDeserialize: serde::de::DeserializeOwned {
    fn from_cbor_bytes(bytes: &[u8]) -> Result<Self, MimiContentError>
    where
        Self: Sized,
    {
        Ok(ciborium::from_reader(bytes)?)
    }
}

impl<T> MimiContentSerialize for T where T: serde::Serialize {}
impl<T> MimiContentDeserialize for T where T: serde::de::DeserializeOwned {}
