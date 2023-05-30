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

pub mod use_name_for_arc_rwlock {
    use serde::{Deserialize, Serialize};
    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use std::sync::{Arc, RwLock};

    use crate::trait_common::GetName;

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

pub mod use_name_for_optional_arc_rwlock {
    use serde::{Deserialize, Serialize};
    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use std::sync::{Arc, RwLock};

    use crate::trait_common::GetName;
    
    pub fn serialize<S, T>(val: &Option<Arc<RwLock<T>>>, s: S) -> Result<S::Ok, S::Error>
        where S: Serializer, T: Serialize + GetName,
    {
        match val {
            Some(v) => String::serialize(&v.read().unwrap().get_name(), s),
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

pub mod serialize_variable_detail {
    use serde::{Deserialize};
    use serde::de::Deserializer;
    use serde::ser::{Serializer, SerializeStruct};
    use std::sync::{Arc, RwLock};

    use crate::trait_common::GetName;
    use crate::tydi_memory_representation::{Variable, TraitCodeLocationAccess};

    pub fn serialize<S>(val: &Arc<RwLock<Variable>>, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer
    {
        let val = val.read().unwrap();
        let mut state = serializer.serialize_struct("Variable", 6)?;
        state.serialize_field("name", &val.get_name())?;
        state.serialize_field("exp", &val.get_exp())?;
        state.serialize_field("evaluated", &val.get_evaluated())?;
        state.serialize_field("type_indication", &val.get_type_indication())?;
        state.serialize_field("declare_location", &val.get_code_location())?;
        state.end()
    }
    
    pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<RwLock<T>>, D::Error>
            where D: Deserializer<'de>, T: Deserialize<'de>,
    {
        todo!()
    }
}

pub mod serialize_variable_value_only {
    use serde::{Deserialize, Serialize};
    use serde::de::Deserializer;
    use serde::ser::{Serializer, SerializeSeq};
    use std::sync::{Arc, RwLock};

    use crate::tydi_memory_representation::{Variable, EvaluationStatus, TypedValue};

    pub fn serialize<S>(val: &Arc<RwLock<Variable>>, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer
    {
        let val = val.read().unwrap();
        if val.get_evaluated() == EvaluationStatus::Evaluated || val.get_evaluated() == EvaluationStatus::Predefined {
            TypedValue::serialize(&val.get_value(), serializer)
            // if val.get_is_array() {
            //     let mut seq = serializer.serialize_seq(Some(val.get_value().len()))?;
            //     for value in val.get_value() {
            //         seq.serialize_element(&value)?;
            //     }
            //     seq.end()
            // }
            // else {
            //     let value = val.get_value();
            //     let value = value.first().unwrap();
            //     value.serialize(serializer)
            // }
        }
        else {
            let exp = val.get_exp();
            match exp {
                Some(exp) => serializer.serialize_str(&format!("{exp}")),
                None => serializer.serialize_str(&format!("???")),
            }
        }
    }
    
    pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<RwLock<T>>, D::Error>
            where D: Deserializer<'de>, T: Deserialize<'de>,
    {
        todo!()
    }
}
