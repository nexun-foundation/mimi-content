use serde::ser::{SerializeMap as _, SerializeSeq};

pub type ExtendedTime = ciborium::tag::Required<ExtendedTimeDetailed, 1001>;
pub type Duration = ciborium::tag::Required<ExtendedTimeDetailed, 1002>;
pub type Period = ciborium::tag::Required<PeriodInner, 1003>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedTimeDetailed {
    pub base_time: std::time::SystemTime, // TODO: switch to a WASM-friendly time
}

impl<'de> serde::Deserialize<'de> for ExtendedTimeDetailed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EtdVisitor;
        impl<'de> serde::de::Visitor<'de> for EtdVisitor {
            type Value = ExtendedTimeDetailed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A RFC9581-compliant ExtendedTimeDetailed map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                while let Some((k, v)) = map.next_entry()? {}

                todo!()
            }
        }

        deserializer.deserialize_map(EtdVisitor)
    }
}

impl serde::Serialize for ExtendedTimeDetailed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = 1;
        let mut map = serializer.serialize_map(Some(len))?;

        // TODO: Finish this
        map.end()
    }
}

pub enum PeriodInner {
    Span {
        start: ExtendedTime,
        end: ExtendedTime,
    },
    DurationStartsAt {
        duration: Duration,
        start: ExtendedTime,
    },
    DurationEndsAt {
        duration: Duration,
        end: ExtendedTime,
    },
}

impl PeriodInner {
    const fn field_count(&self) -> usize {
        match self {
            PeriodInner::Span { .. } => 2,
            PeriodInner::DurationStartsAt { .. } => 3,
            PeriodInner::DurationEndsAt { .. } => 3,
        }
    }
}

impl serde::Serialize for PeriodInner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let field_count = self.field_count();
        let mut seq = serializer.serialize_seq(Some(field_count))?;
        match self {
            PeriodInner::Span { start, end } => {
                debug_assert_eq!(field_count, 2);
                seq.serialize_element(start)?;
                seq.serialize_element(end)?;
            }
            PeriodInner::DurationStartsAt { duration, start } => {
                debug_assert_eq!(field_count, 3);
                seq.serialize_element(start)?;
                seq.serialize_element(&Option::<ExtendedTime>::None)?;
                seq.serialize_element(duration)?;
            }
            PeriodInner::DurationEndsAt { duration, end } => {
                debug_assert_eq!(field_count, 3);
                seq.serialize_element(&Option::<ExtendedTime>::None)?;
                seq.serialize_element(end)?;
                seq.serialize_element(duration)?;
            }
        }

        seq.end()
    }
}

impl<'de> serde::Deserialize<'de> for PeriodInner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PeriodInnerVisitor;
        impl<'de> serde::de::Visitor<'de> for PeriodInnerVisitor {
            type Value = PeriodInner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A RFC9581-compliant Period object")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                use serde::de::Error as _;
                let Some(maybe_start) = seq.next_element::<Option<ExtendedTime>>()? else {
                    return Err(A::Error::missing_field("start"));
                };
                let Some(maybe_end) = seq.next_element::<Option<ExtendedTime>>()? else {
                    return Err(A::Error::missing_field("end"));
                };

                Ok(match (maybe_start, maybe_end) {
                    (Some(start), Some(end)) => PeriodInner::Span { start, end },
                    (Some(start), None) => PeriodInner::DurationStartsAt {
                        duration: seq
                            .next_element()?
                            .ok_or_else(|| A::Error::missing_field("duration"))?,
                        start,
                    },
                    (None, Some(end)) => PeriodInner::DurationEndsAt {
                        duration: seq
                            .next_element()?
                            .ok_or_else(|| A::Error::missing_field("duration"))?,
                        end,
                    },
                    (None, None) => {
                        return Err(A::Error::custom("Both `start` and `end` are missing"));
                    }
                })
            }
        }

        deserializer.deserialize_seq(PeriodInnerVisitor)
    }
}
