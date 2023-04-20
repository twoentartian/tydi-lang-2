use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::tydi_memory_representation::{Package};

#[derive(Clone, Debug, Serialize)]
pub enum TypedValue {
    UnknwonValue,
    
    IntValue(i128),
    StringValue(String),
    BoolValue(bool),
    FloatValue(f64),
    ClockDomainValue(String),

    #[serde(with = "crate::serde_arc_rwlock_name")]
    PackageReferenceValue(Arc<RwLock<Package>>),


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