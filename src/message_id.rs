use crate::{MimiContent, MimiContentAsRef, MimiContentError, MimiContentSerialize, TstrRef};

const MESSAGE_ID_SIZE: usize = 32;

/// See https://www.ietf.org/archive/id/draft-ietf-mimi-content-04.html#name-message-id-and-accepted-tim
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct MessageId(serde_bytes::ByteArray<MESSAGE_ID_SIZE>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct MessageIdRef<'a>(#[serde(with = "serde_bytes")] &'a [u8; MESSAGE_ID_SIZE]);

impl MimiContentAsRef for MessageId {
    type Target<'a> = MessageIdRef<'a>;
    fn as_ref(&self) -> Self::Target<'_> {
        MessageIdRef(self.0.as_ref())
    }
}

impl std::ops::Deref for MessageId {
    type Target = [u8; MESSAGE_ID_SIZE];
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl std::ops::Deref for MessageIdRef<'_> {
    type Target = [u8; MESSAGE_ID_SIZE];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl MessageIdRef<'_> {
    /// Returns the hash algorithm (`0x01` for SHA256)
    pub fn hash_alg(&self) -> u8 {
        self.0[0]
    }
}

impl MessageId {
    /// Construct a MessageId using [`sha2::Sha256`].
    ///
    /// # Arguments
    ///
    /// * `sender_uri` - The sender's MIMI URI as a valid UTF-8 string
    /// * `room_uri` - The room's MIMI URI as a valid UTF-8 string
    /// * `mimi_content` - The MIMI-content message serialized as CBOR
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mimi_content = mimi_content::MimiContent::builder()
    ///     .salt_from_outside_entropy(Default::default())
    ///     .topic_id(Default::default())
    ///     .nested_part(mimi_content::NestedPart::default())
    ///     .build();
    ///
    /// mimi_content::MessageId::construct(
    ///     "mimi://example.domain/u/alice.smith".into(),
    ///     "mimi://example.domain/r/my.super.room".into(),
    ///     &mimi_content,
    /// ).unwrap();
    /// ```
    pub fn construct(
        sender_uri: TstrRef,
        room_uri: TstrRef,
        mimi_content: &MimiContent,
    ) -> Result<Self, MimiContentError> {
        Self::construct_with_custom_alg::<sha2::Sha256>(0x01, sender_uri, room_uri, mimi_content)
    }

    /// Construct a MessageId with a custom algorithm (for example one that doesn't have a well-known OID)
    ///
    /// ❗ Warning: ❗
    ///
    /// ## Directly using this is discouraged, as you might do absolutely random things that will break, for example mistmatching your digest algorithm and its identifier
    ///
    /// # Arguments
    ///
    /// * `hash_alg` - A MessageID hash algorithm identifier
    ///
    /// For the other arguments, see [`Self::construct`]
    fn construct_with_custom_alg<H: digest::Digest>(
        hash_alg: u8,
        sender_uri: TstrRef,
        room_uri: TstrRef,
        mimi_content: &MimiContent,
    ) -> Result<Self, MimiContentError> {
        let mimi_content_bytes = mimi_content.to_cbor_bytes()?;
        let digest = H::new()
            .chain_update(sender_uri.as_bytes())
            .chain_update(room_uri.as_bytes())
            .chain_update(mimi_content_bytes)
            .chain_update(mimi_content.salt)
            .finalize();

        let digest_read_len = std::cmp::min(MESSAGE_ID_SIZE - 1, digest.len());
        debug_assert!(digest_read_len <= MESSAGE_ID_SIZE - 1);
        let mut message_id = [0u8; MESSAGE_ID_SIZE];
        let (message_id_hash_alg, message_id_digest) = message_id.split_at_mut(1);
        // Set HashAlg
        message_id_hash_alg[0] = hash_alg;
        // Write the truncated hash output
        message_id_digest[..digest_read_len].copy_from_slice(&digest[..digest_read_len]);

        Ok(Self::from_raw_unchecked(message_id))
    }

    pub fn from_raw_unchecked(raw_message_id: [u8; MESSAGE_ID_SIZE]) -> Self {
        Self(serde_bytes::ByteArray::from(raw_message_id))
    }
}

#[cfg(test)]
mod tests {
    use crate::{MimiContent, MimiContentAsRef, Tstr};

    use super::MessageId;

    fn build_message_id(mimi_content: &MimiContent) {
        let _message_id = MessageId::construct(
            Tstr::from("mimi://u/alice.smith").as_ref(),
            Tstr::from("mimi://r/conversation").as_ref(),
            mimi_content,
        )
        .unwrap();
    }

    fn build_message_id_custom<H: digest::Digest>(mimi_content: &MimiContent, hash_alg: u8) {
        let _message_id = MessageId::construct_with_custom_alg::<H>(
            hash_alg,
            Tstr::from("mimi://u/alice.smith").as_ref(),
            Tstr::from("mimi://r/conversation").as_ref(),
            mimi_content,
        )
        .unwrap();
    }

    #[test]
    fn message_id_constructs_correctly() {
        let mimi_content = MimiContent {
            salt: [0; 16],
            replaces: None,
            topic_id: vec![].into(),
            expires: None,
            in_reply_to: None,
            extensions: Default::default(),
            nested_part: Default::default(),
        };

        // Sha256
        build_message_id(&mimi_content);

        // custom algs in the custom range
        build_message_id_custom::<sha2::Sha384>(&mimi_content, 0x07);
        build_message_id_custom::<sha2::Sha512>(&mimi_content, 0x08);
        build_message_id_custom::<sha3::Sha3_224>(&mimi_content, 0x09);
        build_message_id_custom::<sha3::Sha3_256>(&mimi_content, 0x0A);
        build_message_id_custom::<sha3::Sha3_384>(&mimi_content, 0x0B);
        build_message_id_custom::<sha3::Sha3_512>(&mimi_content, 0x0C);

        build_message_id_custom::<sha2::Sha224>(&mimi_content, 80);
        build_message_id_custom::<sha2::Sha512_224>(&mimi_content, 81);
        build_message_id_custom::<sha2::Sha512_256>(&mimi_content, 82);
    }
}
