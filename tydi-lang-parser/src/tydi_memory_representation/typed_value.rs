use std::sync::{Arc, RwLock};

use serde::Serialize;
use serde::ser::SerializeStruct;

use crate::deep_clone::DeepClone;
use crate::{tydi_memory_representation::{Package, LogicType}, trait_common::GetName};

use crate::tydi_memory_representation::{Variable, Streamlet, Port, Implementation, Instance, Net, If, For, Identifier, GetScope, Function};

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

    LogicTypeRef(String),
    /// region end

    AnyStreamlet,
    AnyPort,

    AnyImplementation,
    AnyInstance,
    AnyNet,

    PackageReference,

    Function,

    Array(Box<TypeIndication>),
}

impl DeepClone for TypeIndication {
    fn deep_clone(&self) -> Self {
        let output = match self {
            TypeIndication::Any => self.clone(),
            TypeIndication::Unknown => self.clone(),
            TypeIndication::ComplierBuiltin => self.clone(),
            TypeIndication::Int => self.clone(),
            TypeIndication::String => self.clone(),
            TypeIndication::Bool => self.clone(),
            TypeIndication::Float => self.clone(),
            TypeIndication::Clockdomain => self.clone(),
            TypeIndication::AnyLogicType => self.clone(),
            TypeIndication::LogicNull => self.clone(),
            TypeIndication::LogicStream(v) => TypeIndication::LogicStream(v.deep_clone()),
            TypeIndication::LogicBit(v) => TypeIndication::LogicBit(v.deep_clone()),
            TypeIndication::LogicGroup(v) => TypeIndication::LogicGroup(v.deep_clone()),
            TypeIndication::LogicUnion(v) => TypeIndication::LogicUnion(v.deep_clone()),
            TypeIndication::LogicTypeRef(v) => TypeIndication::LogicTypeRef(v.deep_clone()),
            TypeIndication::AnyStreamlet => self.clone(),
            TypeIndication::AnyPort => self.clone(),
            TypeIndication::AnyImplementation => self.clone(),
            TypeIndication::AnyInstance => self.clone(),
            TypeIndication::AnyNet => self.clone(),
            TypeIndication::PackageReference => self.clone(),
            TypeIndication::Function => self.clone(),
            TypeIndication::Array(v) => TypeIndication::Array(Box::new(v.deep_clone())),
        };
        return output;
    }
}

impl std::string::ToString for TypeIndication {
    fn to_string(&self) -> String {
        match self {
            TypeIndication::Any => format!("any"),
            TypeIndication::Unknown => format!("unknown"),
            TypeIndication::ComplierBuiltin => format!("complier_builtin"),
            TypeIndication::Int => format!("int"),
            TypeIndication::String => format!("string"),
            TypeIndication::Bool => format!("bool"),
            TypeIndication::Float => format!("float"),
            TypeIndication::Clockdomain => format!("clock_domain"),
            TypeIndication::AnyLogicType => format!("any_logic_type"),
            TypeIndication::LogicNull => format!("logic_null"),
            TypeIndication::LogicStream(v) => format!("logic_stream({})", v.read().unwrap().get_value().get_brief_info()),
            TypeIndication::LogicBit(v) => format!("logic_bit({})", v.read().unwrap().get_value().get_brief_info()),
            TypeIndication::LogicGroup(v) => format!("logic_group({})", v.read().unwrap().get_value().get_brief_info()),
            TypeIndication::LogicUnion(v) => format!("logic_union({})", v.read().unwrap().get_value().get_brief_info()),
            TypeIndication::LogicTypeRef(v) => format!("logic_type_ref({})", v),
            TypeIndication::AnyStreamlet => format!("any_streamlet"),
            TypeIndication::AnyPort => format!("any_port"),
            TypeIndication::AnyImplementation => format!("any_implementation"),
            TypeIndication::AnyInstance => format!("any_instance"),
            TypeIndication::AnyNet => format!("any_net"),
            TypeIndication::PackageReference => format!("package_reference"),
            TypeIndication::Function => format!("function"),
            TypeIndication::Array(v) => format!("array({})", v.to_string()),
        }
    }
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
            TypedValue::Null => TypeIndication::Any,
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

            TypedValue::Function(_) => TypeIndication::Function,

            //TypedValue during evaluation phase only
            TypedValue::RefToVar(var) => Self::infer_from_typed_value(&var.read().unwrap().get_value()),
            TypedValue::Identifier(_) => unreachable!(),
            
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
    Null,

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

    Function(Arc<RwLock<Function>>),

    //special TypedValue during evaluation
    RefToVar(Arc<RwLock<Variable>>),
    Identifier(Arc<RwLock<Identifier>>),
}

impl DeepClone for TypedValue {
    fn deep_clone(&self) -> Self {
        let output = match self {
            // basic type starts
            TypedValue::UnknwonValue => self.clone(),
            TypedValue::Null => self.clone(),
            TypedValue::PackageReferenceValue(_) => self.clone(),
            TypedValue::IntValue(_) => self.clone(),
            TypedValue::StringValue(_) => self.clone(),
            TypedValue::BoolValue(_) => self.clone(),
            TypedValue::FloatValue(_) => self.clone(),
            TypedValue::ClockDomainValue(_) => self.clone(),
            // basic type ends

            TypedValue::LogicTypeValue(v) => TypedValue::LogicTypeValue(v.deep_clone()),
            TypedValue::Streamlet(v) => {
                //update the parent streamlet for ports
                let output = v.deep_clone();
                let streamlet_scope = output.read().unwrap().get_scope();
                let streamlet_variables = streamlet_scope.read().unwrap().get_variables();
                for (_, var) in streamlet_variables {
                    let var_value = var.read().unwrap().get_value();
                    match var_value {
                        TypedValue::Port(port) => {
                            port.write().unwrap().set_parent_streamlet(Some(output.clone()));
                        },
                        _ => (),    //ignore
                    }
                }
                TypedValue::Streamlet(output)
            },
            TypedValue::Port(v) => TypedValue::Port(v.deep_clone()),
            TypedValue::Implementation(v) => {
                //update the parent implementation for nets
                let output = v.deep_clone();
                let implementation_scope = output.read().unwrap().get_scope();
                let implementation_variables = implementation_scope.read().unwrap().get_variables();
                for (_, var) in implementation_variables {
                    let var_value = var.read().unwrap().get_value();
                    match var_value {
                        TypedValue::Net(net) => {
                            net.write().unwrap().set_parent_impl(Some(output.clone()));
                        },
                        _ => (),    //ignore
                    }
                }
                TypedValue::Implementation(output)
            },
            TypedValue::Instance(v) => TypedValue::Instance(v.deep_clone()),
            TypedValue::Net(v) => TypedValue::Net(v.deep_clone()),
            TypedValue::If(v) => TypedValue::If(v.deep_clone()),
            TypedValue::For(v) => TypedValue::For(v.deep_clone()),
            TypedValue::Array(v) => TypedValue::Array(v.deep_clone()),
            TypedValue::RefToVar(v) => TypedValue::RefToVar(v.deep_clone()),
            TypedValue::Identifier(v) => TypedValue::Identifier(v.deep_clone()),
            TypedValue::Function(v) => TypedValue::Function(v.deep_clone()),
        };
        return output;
    }
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
            TypedValue::Null => {
                let v = format!("Null");
                state.serialize_field("value", &v)?;
            },
            TypedValue::PackageReferenceValue(package_ref) => {
                let package = package_ref.read().unwrap();
                state.serialize_field("value", &package.get_name())?;
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
            },
            TypedValue::Instance(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            },
            TypedValue::Net(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            },
            TypedValue::If(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            },
            TypedValue::For(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            },
            TypedValue::Array(v) => {
                state.serialize_field("value", &*v)?;
            },
            TypedValue::Function(v) => {
                let v = v.read().unwrap();
                state.serialize_field("value", &*v)?;
            },

            //TypedValue during evaluation phase only
            TypedValue::RefToVar(v) => {
                state.serialize_field("value", &v.read().unwrap().get_name())?;
            },
            TypedValue::Identifier(iden) => unreachable!("identifier {:?} should be evaluated", iden),
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

impl TypedValue {
    pub fn get_brief_info(&self) -> String {
        match self {
            TypedValue::UnknwonValue => return String::from("???"),
            TypedValue::Null => return String::from("Null"),
            TypedValue::PackageReferenceValue(package_ref) => return format!("Package({})", package_ref.read().unwrap().get_name()),
            TypedValue::IntValue(v) => return format!("Int({})", v),
            TypedValue::StringValue(v) => return format!("String({})", v),
            TypedValue::BoolValue(v) => return format!("Bool({})", v),
            TypedValue::FloatValue(v) => return format!("Float({})", v),
            TypedValue::ClockDomainValue(v) => return format!("ClockDomain{})", v),
            TypedValue::LogicTypeValue(logic_type) => return logic_type.read().unwrap().get_brief_info(),
            TypedValue::Streamlet(streamlet) => return streamlet.read().unwrap().get_brief_info(),
            TypedValue::Port(port) => return port.read().unwrap().get_name(),
            TypedValue::Implementation(implementation) => return implementation.read().unwrap().get_brief_info(),
            TypedValue::Instance(inst) => {
                let parent_impl = inst.read().unwrap().get_derived_impl();
                let parent_impl_name = match parent_impl {
                    Some(parent_impl) => parent_impl.read().unwrap().get_name(),
                    None => format!("???"),
                };
                return format!("Instance {}({})", inst.read().unwrap().get_name(), parent_impl_name);
            },
            TypedValue::Net(_) => todo!(),
            TypedValue::If(_) => todo!(),
            TypedValue::For(_) => todo!(),
            TypedValue::Array(array) => return format!("Array({})", array.iter().map(|i| i.get_brief_info()).collect::<Vec<_>>().join(", ")),
            TypedValue::Function(v) => return format!("Fcuntion:{}({})", v.read().unwrap().get_function_id(), v.read().unwrap().get_function_arg_exps().iter().map(|(_key, value)| value.clone()).collect::<Vec<_>>().join(" ,")),
            TypedValue::RefToVar(v) => return format!("RefToVar({})", v.read().unwrap().get_name()),
            TypedValue::Identifier(v) => return format!("Identifier({})", v.read().unwrap().get_brief_info()),
        }
    }

    pub fn try_get_referenced_variable(&self) -> Option<Arc<RwLock<Variable>>> {
        match &self {
            TypedValue::RefToVar(var) => return Some(var.clone()),
            _ => return None,
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

