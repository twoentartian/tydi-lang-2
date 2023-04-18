use serde::{Deserialize, Serialize};
use serde::de::Deserializer;
use serde::ser::Serializer;
use std::sync::{Arc, RwLock};

use crate::tydi_memory_representation::GetName;

pub fn serialize<S, T>(val: &Arc<RwLock<T>>, s: S) -> Result<S::Ok, S::Error>
        where S: Serializer, T: Serialize + GetName,
{
    String::serialize(&val.read().unwrap().get_name(), s)
}

pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<RwLock<T>>, D::Error>
        where D: Deserializer<'de>, T: Deserialize<'de>,
{
    todo!()
}