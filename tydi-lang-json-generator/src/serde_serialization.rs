pub mod use_inner_for_arc_rwlock {
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
}

pub mod use_name_for_arc_rwlock {
    use serde::{Deserialize, Serialize};
    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use std::sync::{Arc, RwLock};

    use crate::util::GetName;

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
}

pub mod use_inner_for_optional_arc_rwlock {
    use serde::{Deserialize, Serialize};
    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use std::sync::{Arc, RwLock};
    
    pub fn serialize<S, T>(val: &Option<Arc<RwLock<T>>>, s: S) -> Result<S::Ok, S::Error>
        where S: Serializer, T: Serialize,
    {
        match val {
            Some(v) => T::serialize(&*v.read().unwrap(), s),
            None => s.serialize_none(),
        }
    }
    
    pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<RwLock<T>>, D::Error>
            where D: Deserializer<'de>, T: Deserialize<'de>,
    {
        todo!()
    }
}

pub mod arc_rwlock_in_hash_map_value {
    use std::collections::HashMap;
    use serde::{Deserialize, Serialize};
    use serde::de::Deserializer;
    use serde::ser::{Serializer, SerializeMap};
    use std::sync::{Arc, RwLock};

    pub fn serialize<S, T, K>(val: &HashMap<K, Arc<RwLock<T>>>, s: S) -> Result<S::Ok, S::Error>
            where S: Serializer, T: Serialize, K: Serialize,
    {
        let mut variables_map = s.serialize_map(Some(val.len()))?;
        for (k, v) in val {
            let v = v.read().unwrap();
            variables_map.serialize_entry(k, &*v)?;
        }
        variables_map.end()
    }
    
    pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<RwLock<T>>, D::Error>
            where D: Deserializer<'de>, T: Deserialize<'de>,
    {
        todo!()
    }
}

pub mod arc_rwlock_in_btree_map_value {
    use std::collections::BTreeMap;
    use serde::{Deserialize, Serialize};
    use serde::de::Deserializer;
    use serde::ser::{Serializer, SerializeMap};
    use std::sync::{Arc, RwLock};

    pub fn serialize<S, T, K>(val: &BTreeMap<K, Arc<RwLock<T>>>, s: S) -> Result<S::Ok, S::Error>
            where S: Serializer, T: Serialize, K: Serialize,
    {
        let mut variables_map = s.serialize_map(Some(val.len()))?;
        for (k, v) in val {
            let v = v.read().unwrap();
            variables_map.serialize_entry(k, &*v)?;
        }
        variables_map.end()
    }
    
    pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<RwLock<T>>, D::Error>
            where D: Deserializer<'de>, T: Deserialize<'de>,
    {
        todo!()
    }
}

pub mod arc_rwlock_in_vec_value {
    use serde::{Deserialize, Serialize};
    use serde::de::Deserializer;
    use serde::ser::{Serializer, SerializeSeq};
    use std::sync::{Arc, RwLock};

    pub fn serialize<S, T>(val: &Vec<Arc<RwLock<T>>>, s: S) -> Result<S::Ok, S::Error>
            where S: Serializer, T: Serialize,
    {
        let mut variables_vec = s.serialize_seq(Some(val.len()))?;
        for v in val {
            let v = v.read().unwrap();
            variables_vec.serialize_element(&*v)?;
        }
        variables_vec.end()
    }
    
    pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<RwLock<T>>, D::Error>
            where D: Deserializer<'de>, T: Deserialize<'de>,
    {
        todo!()
    }
}
