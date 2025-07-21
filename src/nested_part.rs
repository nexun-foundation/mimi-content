use crate::{dispositions::Disposition, Bstr, BstrRef, MimiContentAsRef, Tstr, TstrRef};

mod codec;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(u8)]
#[serde(rename_all = "camelCase")]
pub enum PartSemantics {
    /// A single part of the content multipart must be picked and processed.
    ///
    /// Examples:
    /// - A multipart with various versions of the same text, in different languages
    /// - A multipart reaction with various icon size variations
    ChooseOne = 0,
    /// All parts *must* be processed
    SingleUnit = 1,
    /// As many parts as possible must be processed
    ProcessAll = 2,
}

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct SinglePart {
    pub content_type: Tstr,
    pub content: Bstr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SinglePartRef<'a> {
    pub content_type: TstrRef<'a>,
    pub content: BstrRef<'a>,
}

impl SinglePartRef<'_> {
    pub const fn field_count() -> usize {
        2
    }
}

impl MimiContentAsRef for SinglePart {
    type Target<'a> = SinglePartRef<'a>;
    fn as_ref(&self) -> SinglePartRef {
        SinglePartRef {
            content_type: self.content_type.as_ref(),
            content: self.content.as_ref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct ExternalPart {
    pub content_type: Tstr,
    pub url: Tstr,
    pub expires: u32,
    pub size: u64,
    pub enc_alg: u16,
    pub key: Bstr,
    pub nonce: Bstr,
    pub aad: Bstr,
    pub hash_alg: u8,
    pub content_hash: Bstr,
    pub description: Tstr,
    pub filename: Tstr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalPartRef<'a> {
    pub content_type: TstrRef<'a>,
    pub url: TstrRef<'a>,
    pub expires: &'a u32,
    pub size: &'a u64,
    pub enc_alg: &'a u16,
    pub key: BstrRef<'a>,
    pub nonce: BstrRef<'a>,
    pub aad: BstrRef<'a>,
    pub hash_alg: &'a u8,
    pub content_hash: BstrRef<'a>,
    pub description: TstrRef<'a>,
    pub filename: TstrRef<'a>,
}

impl ExternalPartRef<'_> {
    pub const fn field_count() -> usize {
        12
    }
}

impl MimiContentAsRef for ExternalPart {
    type Target<'a> = ExternalPartRef<'a>;
    fn as_ref(&self) -> ExternalPartRef {
        ExternalPartRef {
            content_type: self.content_type.as_ref(),
            url: self.url.as_ref(),
            expires: &self.expires,
            size: &self.size,
            enc_alg: &self.enc_alg,
            key: self.key.as_ref(),
            nonce: self.nonce.as_ref(),
            aad: self.aad.as_ref(),
            hash_alg: &self.hash_alg,
            content_hash: self.content_hash.as_ref(),
            description: self.description.as_ref(),
            filename: self.filename.as_ref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct MultiPart {
    pub part_semantics: PartSemantics,
    pub parts: Vec<NestedPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiPartRef<'a> {
    pub part_semantics: &'a PartSemantics,
    pub parts: Vec<NestedPartRef<'a>>,
}

impl MultiPartRef<'_> {
    pub const fn field_count() -> usize {
        2
    }
}

impl MimiContentAsRef for MultiPart {
    type Target<'a> = MultiPartRef<'a>;
    fn as_ref(&self) -> MultiPartRef {
        MultiPartRef {
            part_semantics: &self.part_semantics,
            parts: self.parts.iter().map(NestedPart::as_ref).collect(),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(u8)]
#[serde(rename_all = "camelCase")]
pub enum NestedPartContentCardinality {
    NullPart = 0,
    SinglePart = 1,
    ExternalPart = 2,
    MultiPart = 3,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum NestedPartContent {
    #[default]
    NullPart = NestedPartContentCardinality::NullPart as u8,
    SinglePart(SinglePart) = NestedPartContentCardinality::SinglePart as u8,
    ExternalPart(ExternalPart) = NestedPartContentCardinality::ExternalPart as u8,
    MultiPart(MultiPart) = NestedPartContentCardinality::MultiPart as u8,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum NestedPartContentRef<'a> {
    #[default]
    NullPart = NestedPartContentCardinality::NullPart as u8,
    SinglePart(SinglePartRef<'a>) = NestedPartContentCardinality::SinglePart as u8,
    ExternalPart(ExternalPartRef<'a>) = NestedPartContentCardinality::ExternalPart as u8,
    MultiPart(MultiPartRef<'a>) = NestedPartContentCardinality::MultiPart as u8,
}

impl MimiContentAsRef for NestedPartContent {
    type Target<'a> = NestedPartContentRef<'a>;
    fn as_ref(&self) -> Self::Target<'_> {
        match self {
            Self::NullPart => NestedPartContentRef::NullPart,
            Self::SinglePart(single) => NestedPartContentRef::SinglePart(single.as_ref()),
            Self::ExternalPart(external) => NestedPartContentRef::ExternalPart(external.as_ref()),
            Self::MultiPart(multi) => NestedPartContentRef::MultiPart(multi.as_ref()),
        }
    }
}

impl NestedPartContent {
    pub fn cardinality(&self) -> NestedPartContentCardinality {
        match self {
            Self::NullPart => NestedPartContentCardinality::NullPart,
            Self::SinglePart(_) => NestedPartContentCardinality::SinglePart,
            Self::ExternalPart(_) => NestedPartContentCardinality::ExternalPart,
            Self::MultiPart(_) => NestedPartContentCardinality::MultiPart,
        }
    }
}

impl NestedPartContentRef<'_> {
    pub fn cardinality(&self) -> NestedPartContentCardinality {
        match self {
            Self::NullPart => NestedPartContentCardinality::NullPart,
            Self::SinglePart(_) => NestedPartContentCardinality::SinglePart,
            Self::ExternalPart(_) => NestedPartContentCardinality::ExternalPart,
            Self::MultiPart(_) => NestedPartContentCardinality::MultiPart,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct NestedPart {
    #[builder(default)]
    pub disposition: Disposition,
    #[builder(default)]
    pub language: Tstr,
    pub part_content: NestedPartContent,
}

impl Default for NestedPart {
    fn default() -> Self {
        Self {
            disposition: Default::default(),
            language: Tstr::from("en".to_owned()),
            part_content: Default::default(),
        }
    }
}

impl MimiContentAsRef for NestedPart {
    type Target<'a> = NestedPartRef<'a>;
    fn as_ref(&self) -> NestedPartRef {
        NestedPartRef {
            disposition: &self.disposition,
            language: TstrRef::from(&*self.language),
            part_content: self.part_content.as_ref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedPartRef<'a> {
    pub disposition: &'a Disposition,
    pub language: TstrRef<'a>,
    pub part_content: NestedPartContentRef<'a>,
}

impl NestedPartRef<'_> {
    pub const fn field_count() -> usize {
        3
    }
}
