use std::sync::{RwLock, Arc};

use pest::pratt_parser::PrattParser;

use crate::evaluation::evaluate_LogicalType;
use crate::tydi_lang_src_to_memory_representation;
use crate::tydi_memory_representation::{Scope, TypedValue, TypeIndication, LogicType};
use crate::tydi_parser::*;
use crate::error::TydiLangError;

use super::{Evaluator, evaluate_Term, evaluate_BinaryOperation};

#[derive(Clone, Debug)]
pub enum Expression {
    Error(TydiLangError),
    Term(TypedValue),
    BinOp {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
}

impl Expression {
    pub fn evaluate_TypedValue(&self, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
        match self {
            Expression::Error(err) => return Err(err.clone()),
            Expression::Term(v) => return Ok(v.clone()),
            Expression::BinOp { lhs, op, rhs } => return evaluate_BinaryOperation(lhs, op, rhs, scope.clone(), evaluator.clone()),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Operator {
    Unknown,

    AccessInner,
    AccessProperty,

    LeftShift,
    RightShift,
    LogicalAnd,
    LogicalOr,
    LogicalEq,
    LogicalNotEq,
    GreaterEq,
    LessEq,

    Greater,
    Less,
    Add,
    Minus,
    Multiply,
    Divide,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
}

#[derive(Clone, PartialEq, Debug)]
pub enum UnaryOperator {
    Unknown,

    OP_UnaryMinus,
    OP_UnaryNot,
}

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};

        // Precedence is defined lowest to highest
        PrattParser::new()
            //https://en.cppreference.com/w/cpp/language/operator_precedence
            .op(Op::infix(Rule::OP_LogicalOr, Left))   // 15
            .op(Op::infix(Rule::OP_LogicalAnd, Left))   // 14
            .op(Op::infix(Rule::OP_BitOr, Left))   // 13
            .op(Op::infix(Rule::OP_BitXor, Left))   // 12
            .op(Op::infix(Rule::OP_BitAnd, Left))   // 11
            .op(Op::infix(Rule::OP_LogicalEq, Left) | Op::infix(Rule::OP_LogicalNotEq, Left)) // 10
            .op(Op::infix(Rule::OP_Greater, Left) | Op::infix(Rule::OP_Less, Left) | Op::infix(Rule::OP_GreaterEq, Left) | Op::infix(Rule::OP_LessEq, Left))    // 9
            .op(Op::infix(Rule::OP_LeftShift, Left) | Op::infix(Rule::OP_RightShift, Left)) // 7
            .op(Op::infix(Rule::OP_Add, Left) | Op::infix(Rule::OP_Minus, Left))            // 6
            .op(Op::infix(Rule::OP_Multiply, Left) | Op::infix(Rule::OP_Divide, Left) | Op::infix(Rule::OP_Mod, Left))  // 5
            .op(Op::infix(Rule::OP_AccessInner, Left) | Op::infix(Rule::OP_AccessProperty, Left))   // 2
    };
}

pub fn evaluate_expression_pest(exp: Pair<Rule>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<Expression, TydiLangError> {
    let result = PRATT_PARSER
    .map_primary(|primary| match primary.as_rule() {
        Rule::Term => {
            let result = evaluate_Term(primary.clone(), scope.clone(), evaluator.clone());
            if result.is_err() {
                return Expression::Error(result.err().unwrap());
            }
            let result = result.ok().unwrap();
            return Expression::Term(result);
        },
        Rule::InfixOp => {
            let result = evaluate_expression_pest(primary, scope.clone(), evaluator.clone());
            return result.ok().unwrap()
        },
        Rule::LogicalType => {
            let logic_type = evaluate_LogicalType(primary, scope.clone(), evaluator.clone());
            if logic_type.is_err() {
                return Expression::Error(logic_type.err().unwrap());
            }
            let logic_type = logic_type.ok().unwrap();
            return Expression::Term(logic_type);
        }
        rule => todo!("Unknown rule: {:?}", rule),
    })
    .map_infix(|lhs, op, rhs| {
        let op = match op.as_rule() {
            Rule::OP_AccessInner => Operator::AccessInner, // AccessInner,
            Rule::OP_AccessProperty => Operator::AccessProperty, //AccessProperty,
        
            Rule::OP_LeftShift => Operator::LeftShift, //LeftShift,
            Rule::OP_RightShift => Operator::RightShift, //RightShift,
            Rule::OP_LogicalAnd => Operator::LogicalAnd, //LogicalAnd,
            Rule::OP_LogicalOr => Operator::LogicalOr, //LogicalOr,
            Rule::OP_LogicalEq => Operator::LogicalEq, //LogicalEq,
            Rule::OP_LogicalNotEq => Operator::LogicalNotEq, //LogicalNotEq,
            Rule::OP_GreaterEq => Operator::GreaterEq, //GreaterEq,
            Rule::OP_LessEq => Operator::LessEq, //LessEq,
        
            Rule::OP_Greater => Operator::Greater, //Greater,
            Rule::OP_Less => Operator::Less, //Less,
            Rule::OP_Add => Operator::Add, //Add,
            Rule::OP_Minus => Operator::Minus, //Minus,
            Rule::OP_Multiply => Operator::Multiply, //Multiply,
            Rule::OP_Divide => Operator::Divide, //Divide,
            Rule::OP_Mod => Operator::Mod, //Mod,
            Rule::OP_BitAnd => Operator::BitAnd, //BitAnd,
            Rule::OP_BitOr => Operator::BitOr, //BitOr,
            Rule::OP_BitXor => Operator::BitXor, //BitXor,
            rule => todo!("Unknown operator {:?}", rule),
        };
        Expression::BinOp {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    })
    .parse(exp.into_inner());

    return Ok(result);
}

pub fn evaluate_expression(exp: String, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let parse_result = TydiLangSrc::parse(Rule::Exp,&exp);
    if parse_result.is_err() {
        unreachable!("because the exp should have already been parsed before, we should never reach here");
    }
    let mut parse_result = parse_result.ok().unwrap();
    let expresssion = evaluate_expression_pest(parse_result.next().unwrap(), scope.clone(), evaluator.clone())?;
    return expresssion.evaluate_TypedValue(scope.clone(), evaluator.clone());
}




