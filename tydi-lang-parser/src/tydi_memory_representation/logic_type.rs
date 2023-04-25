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