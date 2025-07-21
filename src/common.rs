use crate::MimiContentAsRef;

#[derive(
    Debug,
    Default,
    Clone,
    Hash,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Tstr(String);

impl std::ops::Deref for Tstr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Tstr {
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<Tstr> for String {
    fn from(value: Tstr) -> Self {
        value.into_inner()
    }
}

impl From<String> for Tstr {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Tstr {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, serde::Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct TstrRef<'a>(&'a str);

impl std::ops::Deref for TstrRef<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl MimiContentAsRef for Tstr {
    type Target<'a> = TstrRef<'a>;
    fn as_ref(&self) -> TstrRef {
        TstrRef(self.0.as_str())
    }
}

impl<'a> From<&'a str> for TstrRef<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Bstr(#[serde(with = "serde_bytes")] Vec<u8>);

impl Bstr {
    #[inline]
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct BstrRef<'a>(#[serde(with = "serde_bytes")] &'a [u8]);

impl MimiContentAsRef for Bstr {
    type Target<'a> = BstrRef<'a>;
    fn as_ref(&self) -> BstrRef {
        BstrRef(&self.0)
    }
}

impl From<Vec<u8>> for Bstr {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<Bstr> for Vec<u8> {
    fn from(value: Bstr) -> Self {
        value.into_inner()
    }
}

impl<'a> From<&'a [u8]> for BstrRef<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for Bstr {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for BstrRef<'_> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
