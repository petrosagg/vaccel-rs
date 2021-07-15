use std::convert::TryFrom;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Resource {
    Bytes(Vec<u8>),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceRef<T> {
    pub(crate) id: u64,
    pub(crate) marker: PhantomData<T>,
}

// TODO: make this a macro
impl From<Vec<u8>> for Resource {
    fn from(inner: Vec<u8>) -> Self {
        Resource::Bytes(inner)
    }
}

impl<'a> TryFrom<&'a Resource> for &'a Vec<u8> {
    type Error = ();

    fn try_from(inner: &'a Resource) -> Result<Self, Self::Error> {
        match inner {
            Resource::Bytes(ref inner) => Ok(inner),
            _ => Err(()),
        }
    }
}

impl From<String> for Resource {
    fn from(inner: String) -> Self {
        Resource::String(inner)
    }
}

impl<'a> TryFrom<&'a Resource> for &'a String {
    type Error = ();

    fn try_from(inner: &'a Resource) -> Result<Self, Self::Error> {
        match inner {
            Resource::String(ref inner) => Ok(inner),
            _ => Err(()),
        }
    }
}
