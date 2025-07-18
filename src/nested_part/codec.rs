use serde::ser::SerializeSeq as _;

use crate::{
    ExternalPart, ExternalPartRef, MultiPart, MultiPartRef, NestedPart, NestedPartContent,
    NestedPartContentCardinality, NestedPartContentRef, NestedPartRef, SinglePart, SinglePartRef,
};

fn map_len_for_nestedpartref(nested_part: &NestedPartRef<'_>) -> usize {
    let extra_fields = match nested_part.part_content {
        NestedPartContentRef::NullPart => 0,
        NestedPartContentRef::SinglePart(_) => SinglePartRef::field_count(),
        NestedPartContentRef::MultiPart(_) => MultiPartRef::field_count(),
        NestedPartContentRef::ExternalPart(_) => ExternalPartRef::field_count(),
    };
    NestedPartRef::field_count() + extra_fields
}

impl serde::Serialize for NestedPartRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(map_len_for_nestedpartref(self)))?;
        seq.serialize_element(&self.disposition)?;
        seq.serialize_element(&self.language)?;
        seq.serialize_element(&self.part_content.cardinality())?;
        match &self.part_content {
            NestedPartContentRef::NullPart => {}
            NestedPartContentRef::SinglePart(single) => {
                seq.serialize_element(&single.content_type)?;
                seq.serialize_element(&single.content)?;
            }
            NestedPartContentRef::ExternalPart(external) => {
                seq.serialize_element(&external.content_type)?;
                seq.serialize_element(&external.url)?;
                seq.serialize_element(&external.expires)?;
                seq.serialize_element(&external.size)?;
                seq.serialize_element(&external.enc_alg)?;
                seq.serialize_element(&external.key)?;
                seq.serialize_element(&external.nonce)?;
                seq.serialize_element(&external.aad)?;
                seq.serialize_element(&external.hash_alg)?;
                seq.serialize_element(&external.content_hash)?;
                seq.serialize_element(&external.description)?;
                seq.serialize_element(&external.filename)?;
            }
            NestedPartContentRef::MultiPart(multi) => {
                seq.serialize_element(&multi.part_semantics)?;
                seq.serialize_element(&multi.parts)?;
            }
        }

        seq.end()
    }
}

impl serde::Serialize for NestedPart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use crate::MimiContentAsRef as _;
        self.as_ref().serialize(serializer)
    }
}

struct NestedPartVisitor;
impl<'de> serde::de::Visitor<'de> for NestedPartVisitor {
    type Value = NestedPart;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a NestedPart struct formatted as a tuple value")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<NestedPart, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        let mut counter = 0;
        let disposition = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
        counter += 1;
        let language = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
        counter += 1;
        let cardinality: NestedPartContentCardinality = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
        counter += 1;

        let part_content = match cardinality {
            NestedPartContentCardinality::NullPart => NestedPartContent::NullPart,
            NestedPartContentCardinality::SinglePart => {
                let content_type = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;
                let content = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;

                NestedPartContent::SinglePart(SinglePart {
                    content_type,
                    content,
                })
            }
            NestedPartContentCardinality::ExternalPart => {
                let content_type = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;
                let url = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;
                let expires = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;
                let size = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;
                let enc_alg = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;
                let key = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;
                let nonce = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;
                let aad = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;
                let hash_alg = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;
                let content_hash = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;
                let description = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                let filename = seq.next_element()?.unwrap_or_default();

                NestedPartContent::ExternalPart(ExternalPart {
                    content_type,
                    url,
                    expires,
                    size,
                    enc_alg,
                    key,
                    nonce,
                    aad,
                    hash_alg,
                    content_hash,
                    description,
                    filename,
                })
            }
            NestedPartContentCardinality::MultiPart => {
                let part_semantics = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;
                counter += 1;

                let parts = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(counter, &self))?;

                NestedPartContent::MultiPart(MultiPart {
                    part_semantics,
                    parts,
                })
            }
        };

        Ok(NestedPart {
            disposition,
            language,
            part_content,
        })
    }
}

impl<'de> serde::Deserialize<'de> for NestedPart {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(NestedPartVisitor)
    }
}
