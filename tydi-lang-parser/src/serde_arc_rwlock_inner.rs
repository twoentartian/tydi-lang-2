use serde::{Deserialize, Serialize};
use serde::de::Deserializer;
use serde::ser::Serializer;
use std::sync::{Arc, RwLock};

pub fn serialize<S, T>(val: &Arc<RwLock<T>>, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer, T: Serialize,
{
    T::serialize(&*val.read().unwrap(), s)
}

pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<RwLock<T>>, D::Error>
        where D: Deserializer<'de>, T: Deserialize<'de>,
{
    todo!()
}