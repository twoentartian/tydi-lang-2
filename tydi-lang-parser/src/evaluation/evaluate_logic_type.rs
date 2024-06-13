use std::sync::{RwLock, Arc};
use crate::generate_name::generate_init_value;
use crate::tydi_lang_src_to_memory_representation;
use crate::tydi_memory_representation::{GetScope, LogicBit, LogicGroup, LogicStream, LogicType, LogicUnion, Scope, ScopeType, SrcInfo, TraitCodeLocationAccess, TypeIndication, TypedValue};
use crate::tydi_parser::*;
use crate::error::TydiLangError;

use super::{Evaluator, evaluate_var, evaluate_scope, ScopeOwner};

#[allow(non_snake_case)]
pub fn evaluate_LogicalType(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    evaluator.write().unwrap().increase_deepth();
    
    let mut output = TypedValue::UnknwonValue;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::LogicalType_Basic => {
                let logic_type = tydi_lang_src_to_memory_representation::parse_LogicalType_Basic(element, scope.clone(), SrcInfo::new_init())?;
                match &logic_type {
                    TypeIndication::LogicNull => {
                        output = TypedValue::LogicTypeValue(Arc::new(RwLock::new(LogicType::LogicNullType)));
                    },
                    TypeIndication::LogicStream(var) => {
                        evaluate_var(var.clone(), scope.clone(), evaluator.clone())?;
                        output = TypedValue::RefToVar(var.clone());
                    },
                    TypeIndication::LogicBit(var) => {
                        evaluate_var(var.clone(), scope.clone(), evaluator.clone())?;
                        output = TypedValue::RefToVar(var.clone());
                    },
                    TypeIndication::LogicGroup(_) => {
                        unreachable!()
                    },
                    TypeIndication::LogicUnion(_) => {
                        unreachable!()
                    },
                    _ => unreachable!(),
                }
            }
            _ => unreachable!()
        }
    }

    evaluator.write().unwrap().decrease_deepth();
    return Ok(output);
}

#[allow(non_snake_case)]
pub fn evaluate_LogicStream(target: Arc<RwLock<LogicStream>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    evaluator.write().unwrap().increase_deepth();
    
    //evaluate stream_type
    {
        let stream_type = target.read().unwrap().get_stream_type();
        let value = evaluate_var(stream_type.clone(), scope.clone(), evaluator.clone())?;
        match &value {
            TypedValue::LogicTypeValue(x) => {
                match *x.read().unwrap() {
                    LogicType::LogicNullType => return Err(TydiLangError::new(format!("the stream_type {:?} cannot be logic Null", value), target.read().unwrap().get_code_location())),
                    LogicType::LogicBitType(_) => (),
                    LogicType::LogicGroupType(_) => (),
                    LogicType::LogicUnionType(_) => (),
                    LogicType::LogicStreamType(_) => return Err(TydiLangError::new(format!("the stream_type {:?} cannot be a nested stream type", value), target.read().unwrap().get_code_location())),
                }
            },
            TypedValue::RefToVar(ref_var) => {
                evaluate_var(ref_var.clone(), scope.clone(), evaluator.clone())?;
            }
            _ => return Err(TydiLangError::new(format!("the stream_type {:?} must be a logic type", value), target.read().unwrap().get_code_location()))
        }
    }

    //evaluate dimension
    {
        let dimension = target.read().unwrap().get_dimension();
        let value = evaluate_var(dimension, scope.clone(), evaluator.clone())?;
        match &value {
            TypedValue::IntValue(x) => {
                if *x <= 0 {
                    return Err(TydiLangError::new(format!("the dimension {:?} must be larger than 0", value), target.read().unwrap().get_code_location()));
                }
            },
            _ => return Err(TydiLangError::new(format!("the dimension {:?} must be an integer", value), target.read().unwrap().get_code_location()))
        }
    }

    //evaluate user_type
    {
        let user_type = target.read().unwrap().get_user_type();
        let value = evaluate_var(user_type, scope.clone(), evaluator.clone())?;
        match &value {
            TypedValue::LogicTypeValue(x) => {
                match *x.read().unwrap() {
                    LogicType::LogicNullType => (),
                    LogicType::LogicBitType(_) => (),
                    LogicType::LogicGroupType(_) => (),
                    LogicType::LogicUnionType(_) => (),
                    LogicType::LogicStreamType(_) => return Err(TydiLangError::new(format!("the user_type {:?} cannot be a logic stream type", value), target.read().unwrap().get_code_location())),
                }
            },
            TypedValue::RefToVar(ref_var) => {
                evaluate_var(ref_var.clone(), scope.clone(), evaluator.clone())?;
            }
            _ => return Err(TydiLangError::new(format!("the user_type {:?} must be a logic type", value), target.read().unwrap().get_code_location()))
        }
    }

    //evaluate throughput
    {
        let throughput = target.read().unwrap().get_throughput();
        let value = evaluate_var(throughput, scope.clone(), evaluator.clone())?;
        match &value {
            TypedValue::FloatValue(x) => {
                if *x <= 0.0 {
                    return Err(TydiLangError::new(format!("the throughput {:?} must be positive", value), target.read().unwrap().get_code_location()));
                }
            },
            _ => return Err(TydiLangError::new(format!("the throughput {:?} must be a float number", value), target.read().unwrap().get_code_location()))
        }
    }

    //evaluate synchronicity
    {
        let synchronicity = target.read().unwrap().get_synchronicity();
        let value = evaluate_var(synchronicity, scope.clone(), evaluator.clone())?;
        match &value {
            TypedValue::StringValue(x) => {
                if x != "Sync" && x != "Flatten" && x != "Desync" && x != "FlatDesync" {
                    return Err(TydiLangError::new(format!("the synchronicity {:?} must be one of Sync(default)/Flatten/Desync/FlatDesync", value), target.read().unwrap().get_code_location()));
                }
            },
            _ => return Err(TydiLangError::new(format!("the synchronicity {:?} must be a string", value), target.read().unwrap().get_code_location()))
        }
    }

    //evaluate complexity
    {
        let complexity = target.read().unwrap().get_complexity();
        let value = evaluate_var(complexity, scope.clone(), evaluator.clone())?;
        match &value {
            TypedValue::IntValue(x) => {
                if !(*x >= 1 && *x <= 8) {
                    return Err(TydiLangError::new(format!("the complexity {:?} must be in the range 1~8", value), target.read().unwrap().get_code_location()))
                }
            },
            _ => return Err(TydiLangError::new(format!("the complexity {:?} must be an integer", value), target.read().unwrap().get_code_location()))
        }
    }

    //evaluate direction
    {
        let direction = target.read().unwrap().get_direction();
        let value = evaluate_var(direction, scope.clone(), evaluator.clone())?;
        match &value {
            TypedValue::StringValue(x) => {
                if x != "Forward" && x != "Reverse" {
                    return Err(TydiLangError::new(format!("the direction {:?} must be one of Forward(default)/Reverse", value), target.read().unwrap().get_code_location()));
                }
            },
            _ => return Err(TydiLangError::new(format!("the direction {:?} must be a string", value), target.read().unwrap().get_code_location()))
        }
    }

    //evalute keep
    {
        let keep = target.read().unwrap().get_keep();
        let value = evaluate_var(keep, scope.clone(), evaluator.clone())?;
        match &value {
            TypedValue::BoolValue(_) => (),
            _ => return Err(TydiLangError::new(format!("the keep {:?} must be a bool", value), target.read().unwrap().get_code_location()))
        }
    }

    let output = TypedValue::LogicTypeValue(Arc::new(RwLock::new(LogicType::LogicStreamType(target.clone()))));


    evaluator.write().unwrap().decrease_deepth();
    return Ok(output);
}

#[allow(non_snake_case)]
pub fn evaluate_LogicBit(target: Arc<RwLock<LogicBit>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    evaluator.write().unwrap().increase_deepth();

    let bit_width_var = target.read().unwrap().get_bit_width();
    let bit_width_typed_value = evaluate_var(bit_width_var, scope.clone(), evaluator.clone())?;
    match &bit_width_typed_value {
        TypedValue::IntValue(x) => {
            if *x <= 0 {
                return Err(TydiLangError::new(format!("the bitwidth {:?} must be larger than 0", bit_width_typed_value), target.read().unwrap().get_code_location()));
            }
        },
        _ => return Err(TydiLangError::new(format!("the bitwidth {:?} must be an integer", bit_width_typed_value), target.read().unwrap().get_code_location()))
    }
    let output = TypedValue::LogicTypeValue(Arc::new(RwLock::new(LogicType::LogicBitType(target.clone()))));

    evaluator.write().unwrap().decrease_deepth();
    return Ok(output);
}

#[allow(non_snake_case)]
pub fn evaluate_LogicGroup(target: Arc<RwLock<LogicGroup>>, _scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    evaluator.write().unwrap().increase_deepth();

    let logic_group_scope = target.read().unwrap().get_scope();
    evaluate_scope(logic_group_scope.clone(), &ScopeType::GroupScope, &ScopeOwner::LogicGroup(target.clone()), logic_group_scope.clone(), evaluator.clone())?;

    let output = TypedValue::LogicTypeValue(Arc::new(RwLock::new(LogicType::LogicGroupType(target.clone()))));
    
    evaluator.write().unwrap().decrease_deepth();
    return Ok(output);
}

#[allow(non_snake_case)]
pub fn evaluate_LogicUnion(target: Arc<RwLock<LogicUnion>>, _scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    evaluator.write().unwrap().increase_deepth();

    let logic_union_scope = target.read().unwrap().get_scope();
    evaluate_scope(logic_union_scope.clone(), &ScopeType::UnionScope, &ScopeOwner::LogicUnion(target.clone()), logic_union_scope.clone(), evaluator.clone())?;

    let output = TypedValue::LogicTypeValue(Arc::new(RwLock::new(LogicType::LogicUnionType(target.clone()))));

    evaluator.write().unwrap().decrease_deepth();
    return Ok(output);
}

