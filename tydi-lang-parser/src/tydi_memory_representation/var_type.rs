use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::{tydi_memory_representation::{Package, LogicType}, trait_common::GetName};

use crate::tydi_memory_representation::{Variable, Streamlet};

#[derive(Clone, Debug, Serialize)]
pub enum TypeIndication {
    Any,
    Unknown,
    ComplierBuiltin,

    Int,
    String,
    Bool,
    Float,
    Clockdomain,

    // represents any logic types
    AnyLogicType,

    /// region begin: these indications are only for indicator use - x : Bit(8), etc
    LogicNull,
    #[serde(with = "crate::serde_serialization::use_name_for_arc_rwlock")]
    LogicStream(Arc<RwLock<Variable>>),
    #[serde(with = "crate::serde_serialization::use_name_for_arc_rwlock")]
    LogicBit(Arc<RwLock<Variable>>),
    #[serde(with = "crate::serde_serialization::use_name_for_arc_rwlock")]
    LogicGroup(Arc<RwLock<Variable>>),
    #[serde(with = "crate::serde_serialization::use_name_for_arc_rwlock")]
    LogicUnion(Arc<RwLock<Variable>>),
    /// region end

    AnyStreamlet,

    AnyImplementation,

    PackageReference,
}

impl PartialEq for TypeIndication {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LogicStream(l0), Self::LogicStream(r0)) => l0.read().unwrap().get_name() == r0.read().unwrap().get_name(),
            (Self::LogicBit(l0), Self::LogicBit(r0)) => l0.read().unwrap().get_name() == r0.read().unwrap().get_name(),
            (Self::LogicGroup(l0), Self::LogicGroup(r0)) => l0.read().unwrap().get_name() == r0.read().unwrap().get_name(),
            (Self::LogicUnion(l0), Self::LogicUnion(r0)) => l0.read().unwrap().get_name() == r0.read().unwrap().get_name(),

            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl TypeIndication {
    pub fn infer_from_typed_value(value: &TypedValue) -> Self {
        return match value {
            TypedValue::UnknwonValue => TypeIndication::Any,
            TypedValue::PackageReferenceValue(_) => TypeIndication::ComplierBuiltin,
            TypedValue::IntValue(_) => TypeIndication::Int,
            TypedValue::StringValue(_) => TypeIndication::String,
            TypedValue::BoolValue(_) => TypeIndication::Bool,
            TypedValue::FloatValue(_) => TypeIndication::Float,
            TypedValue::ClockDomainValue(_) => TypeIndication::Clockdomain,
            TypedValue::LogicTypeValue(_) => TypeIndication::AnyLogicType,
            
            TypedValue::Streamlet(_) => unreachable!(),
        }
    }

    pub fn is_compatible_with_typed_value(&self, value: &TypedValue) -> bool {
        match self {
            TypeIndication::Any => { true },
            TypeIndication::Unknown => { false },   // we'd be striect here
            TypeIndication::ComplierBuiltin => { true },   
            TypeIndication::Int => match value {
                TypedValue::IntValue(_) => true,
                _ => false,
            },
            TypeIndication::String => match value {
                TypedValue::StringValue(_) => true,
                _ => false,
            },
            TypeIndication::Bool => match value {
                TypedValue::BoolValue(_) => true,
                _ => false,
            },
            TypeIndication::Float => match value {
                TypedValue::FloatValue(_) => true,
                _ => false,
            },
            TypeIndication::Clockdomain => match value {
                TypedValue::ClockDomainValue(_) => true,
                _ => false,
            },
            TypeIndication::AnyLogicType => match value {
                TypedValue::LogicTypeValue(_) => true,
                _ => false,
            },
            _ => todo!()
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypedValue {
    UnknwonValue,

    PackageReferenceValue(Arc<RwLock<Package>>),
    
    IntValue(i128),
    StringValue(String),
    BoolValue(bool),
    FloatValue(f64),
    ClockDomainValue(String),

    LogicTypeValue(Arc<RwLock<LogicType>>),

    Streamlet(Arc<RwLock<Streamlet>>),
}

impl Serialize for TypedValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer 
    {
        match self {
            TypedValue::UnknwonValue => {
                let v = format!("???");
                serializer.serialize_str(&v)
            },
            TypedValue::PackageReferenceValue(package_ref) => {
                crate::serde_serialization::use_name_for_arc_rwlock::serialize(package_ref, serializer)
            },
            TypedValue::IntValue(v) => serializer.serialize_i128(*v),
            TypedValue::StringValue(v) => serializer.serialize_str(v),
            TypedValue::BoolValue(v) => serializer.serialize_bool(*v),
            TypedValue::FloatValue(v) => serializer.serialize_f64(*v),
            TypedValue::ClockDomainValue(v) => {
                let v = format!("CLOCK_{}", v);
                serializer.serialize_str(&v)
            },
            TypedValue::LogicTypeValue(v) => {
                let v = v.read().unwrap();
                LogicType::serialize(&*v, serializer)
            },
            TypedValue::Streamlet(v) => {
                let v = v.read().unwrap();
                Streamlet::serialize(&*v, serializer)
            },
        }
    }
}

impl std::cmp::PartialEq for TypedValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UnknwonValue, Self::UnknwonValue) => false,      //unknown values are not equal to unknown values
            (Self::IntValue(l0), Self::IntValue(r0)) => l0 == r0,
            (Self::StringValue(l0), Self::StringValue(r0)) => l0 == r0,
            (Self::BoolValue(l0), Self::BoolValue(r0)) => l0 == r0,
            (Self::FloatValue(l0), Self::FloatValue(r0)) => l0 == r0,
            (Self::ClockDomainValue(l0), Self::ClockDomainValue(r0)) => l0 == r0,
            (Self::PackageReferenceValue(l0), Self::PackageReferenceValue(r0)) => {
                let l0_read = l0.read().unwrap();
                let r0_read = r0.read().unwrap();
                std::ptr::eq(&*l0_read, &*r0_read)
            }, //we compare the pointer address here
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}


#[cfg(test)]
mod test_var_type {
    use super::*;

    #[test]
    fn test_typed_value_eq() {
        let v0 = TypedValue::UnknwonValue;
        let v1 = TypedValue::UnknwonValue;
        assert!(v0 != v1);
    
        let v0 = TypedValue::IntValue(1);
        let v1 = TypedValue::IntValue(1);
        let v2 = TypedValue::IntValue(10);
        assert!(v0 == v1);
        assert!(v0 != v2);
    
        let v0 = TypedValue::StringValue(format!("1"));
        let v1 = TypedValue::StringValue(format!("1"));
        let v2 = TypedValue::StringValue(format!("10"));
        assert!(v0 == v1);
        assert!(v0 != v2);
    
        let v0 = TypedValue::BoolValue(true);
        let v1 = TypedValue::BoolValue(true);
        let v2 = TypedValue::BoolValue(false);
        assert!(v0 == v1);
        assert!(v0 != v2);
    
        let v0 = TypedValue::FloatValue(1.1);
        let v1 = TypedValue::FloatValue(1.1);
        let v2 = TypedValue::FloatValue(10.0);
        assert!(v0 == v1);
        assert!(v0 != v2);
    
        let v0 = TypedValue::ClockDomainValue(format!("1"));
        let v1 = TypedValue::ClockDomainValue(format!("1"));
        let v2 = TypedValue::ClockDomainValue(format!("10"));
        assert!(v0 == v1);
        assert!(v0 != v2);
    
        let p0 = Package::new(format!("package0"));
        let p1 = Package::new(format!("package1"));
        let v0 = TypedValue::PackageReferenceValue(p0.clone());
        let v1 = TypedValue::PackageReferenceValue(p0.clone());
        let v2 = TypedValue::PackageReferenceValue(p1.clone());
        assert!(v0 == v1);
        assert!(v0 != v2);   
    }
}

