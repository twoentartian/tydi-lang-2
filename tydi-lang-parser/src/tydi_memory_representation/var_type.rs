use std::sync::{Arc, RwLock};

use serde::{Serialize};
use serde::ser::SerializeStruct;

use crate::{tydi_memory_representation::{Package, LogicType}, trait_common::GetName};

use crate::tydi_memory_representation::{Variable, Streamlet, Port, Implementation, Instance, Net, If, For};

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
    AnyPort,

    AnyImplementation,
    AnyInstance,
    AnyNet,

    PackageReference,

    Array(Box<TypeIndication>),
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
            
            TypedValue::Streamlet(_) => TypeIndication::AnyStreamlet,
            TypedValue::Port(_) => TypeIndication::AnyPort,

            TypedValue::Implementation(_) => TypeIndication::AnyImplementation,
            TypedValue::Instance(_) => TypeIndication::AnyInstance,
            TypedValue::Net(_) => TypeIndication::AnyNet,

            TypedValue::If(_) => TypeIndication::ComplierBuiltin,
            TypedValue::For(_) => TypeIndication::ComplierBuiltin,

            TypedValue::Array(v) => {
                if v.len() == 0 {
                    TypeIndication::Array(Box::new(TypeIndication::Any))
                }
                else {
                    TypeIndication::Array(Box::new(Self::infer_from_typed_value(&v[0]))) //maybe we should have TypeIndication::Array?
                }
            },
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
            TypeIndication::AnyStreamlet => match value {
                TypedValue::Streamlet(_) => true,
                _ => false,
            },
            TypeIndication::AnyPort => match value {
                TypedValue::Port(_) => true,
                _ => false,
            },
            TypeIndication::AnyImplementation => match value {
                TypedValue::Implementation(_) => true,
                _ => false,
            },
            TypeIndication::AnyInstance => match value {
                TypedValue::Instance(_) => true,
                _ => false,
            },
            TypeIndication::AnyNet => match value {
                TypedValue::Net(_) => true,
                _ => false,
            },

            _ => todo!()
        }
    }
}

#[derive(Clone, Debug, strum::IntoStaticStr)]
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
    Port(Arc<RwLock<Port>>),

    Implementation(Arc<RwLock<Implementation>>),
    Instance(Arc<RwLock<Instance>>),
    Net(Arc<RwLock<Net>>),

    If(Arc<RwLock<If>>),
    For(Arc<RwLock<For>>),

    Array(Vec<TypedValue>),
}

impl Serialize for TypedValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer 
    {
        let mut state = serializer.serialize_struct("TypedValue", 2)?;
        let enum_type_str: &'static str = self.into();
        state.serialize_field("type", enum_type_str)?;
        match self {
            TypedValue::UnknwonValue => {
                let v = format!("???");
                state.serialize_field("value", &v)?;
            },
            TypedValue::PackageReferenceValue(package_ref) => {
                let package = package_ref.read().unwrap();
                state.serialize_field("value", &*package)?;
            },
            TypedValue::IntValue(v) => state.serialize_field("value", v)?,
            TypedValue::StringValue(v) => state.serialize_field("value", v)?,
            TypedValue::BoolValue(v) => state.serialize_field("value", v)?,
            TypedValue::FloatValue(v) => state.serialize_field("value", v)?,
            TypedValue::ClockDomainValue(v) => {
                let v = format!("!CLOCK_{}", v);
                state.serialize_field("value", &v)?;
            },
            TypedValue::LogicTypeValue(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            },
            TypedValue::Streamlet(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            },
            TypedValue::Port(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            },
            TypedValue::Implementation(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            }
            TypedValue::Instance(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            }
            TypedValue::Net(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            }
            TypedValue::If(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            }
            TypedValue::For(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            }
            TypedValue::Array(v) => {
                state.serialize_field("value", &*v)?;
            }

        };
        state.end()
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
            (Self::Array(v0), Self::Array(v1)) => {
                if v0.len() != v1.len() {
                    return false;
                }
                for i in 0..v0.len() {
                    if v0[i] != v1[i] {
                        return false;
                    }
                }
                return true;
            },
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

