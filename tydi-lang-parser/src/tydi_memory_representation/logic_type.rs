use std::sync::{Arc, RwLock};

use serde::{Serialize};

pub mod logic_bit;
pub(in crate) use logic_bit::*;

pub mod logic_group;
pub(in crate) use logic_group::*;

pub mod logic_union;
pub(in crate) use logic_union::*;

pub mod logic_stream;
pub(in crate) use logic_stream::*;

#[derive(Clone, Debug, Serialize)]
pub enum LogicType {
    LogicNullType,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    LogicBitType(Arc<RwLock<LogicBit>>),

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    LogicGroupType(Arc<RwLock<LogicGroup>>),

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    LogicUnionType(Arc<RwLock<LogicUnion>>),

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    LogicStreamType(Arc<RwLock<LogicStream>>),
}

impl PartialEq for LogicType {
    fn eq(&self, target: &LogicType) -> bool { 
        match (self, target) {
            (Self::LogicNullType, Self::LogicNullType) => return true,
            (Self::LogicBitType(v0), Self::LogicBitType(v1)) => {
                return Arc::ptr_eq(v0, v1);
            },
            (Self::LogicGroupType(v0), Self::LogicGroupType(v1)) => {
                return Arc::ptr_eq(v0, v1);
            },
            (Self::LogicUnionType(v0), Self::LogicUnionType(v1)) => {
                return Arc::ptr_eq(v0, v1);
            },
            (Self::LogicStreamType(v0), Self::LogicStreamType(v1)) => {
                return Arc::ptr_eq(v0, v1);
            },
            (_, _) => return false,
        }
    }
}