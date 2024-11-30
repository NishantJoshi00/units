use core::marker::PhantomData;

use super::RType;

pub struct Json;

pub trait Formatter {
    type SType;

    fn serialize<T: serde::Serialize>(data: T) -> crate::Result<Self::SType>;
    fn deserialize<T>(data: Self::SType) -> crate::Result<T>
    where
        for<'a> T: serde::Deserialize<'a>;
}

impl Formatter for Json {
    type SType = serde_json::Value;

    fn serialize<T: serde::Serialize>(data: T) -> crate::Result<Self::SType> {
        serde_json::to_value(data).map_err(|e| e.into())
    }

    fn deserialize<T>(data: Self::SType) -> crate::Result<T>
    where
        for<'a> T: serde::Deserialize<'a>,
    {
        serde_json::from_value(data).map_err(|e| e.into())
    }
}

impl<Format: Formatter> RType<Format> {
    pub fn new<T: serde::Serialize>(data: T) -> crate::Result<Self> {
        let data = Format::serialize(data)?;
        Ok(Self(data, PhantomData))
    }

    pub fn into_inner<T>(self) -> crate::Result<T>
    where
        for<'a> T: serde::Deserialize<'a>,
    {
        Format::deserialize(self.0)
    }
}
