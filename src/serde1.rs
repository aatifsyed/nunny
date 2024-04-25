use serde1::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

use crate::Slice;
#[cfg(feature = "alloc")]
use {
    crate::{Error, Vec},
    alloc::boxed::Box,
};

impl<'de: 'a, 'a> Deserialize<'de> for &'a Slice<u8> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <&Slice<u8>>::try_from(<&[u8]>::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

impl<T> Serialize for Slice<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_slice().serialize(serializer)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(do_doc_cfg, doc(cfg(feature = "alloc")))]
impl<'de, T> Deserialize<'de> for Box<Slice<T>>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <Box<Slice<T>>>::try_from(<Box<[T]>>::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(do_doc_cfg, doc(cfg(feature = "alloc")))]
impl<'de, T> Deserialize<'de> for Vec<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::new(alloc::vec::Vec::deserialize(deserializer)?)
            .map_err(|_| D::Error::custom(Error(())))
    }
}
#[cfg(feature = "alloc")]
#[cfg_attr(do_doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_vec().serialize(serializer)
    }
}
