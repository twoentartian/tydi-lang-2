

#[cfg(test)]
mod test_expression_parser {
    use crate::tydi_memory_representation::{Project, TypedValue, Scope};

    use super::super::*;

    fn check_exp(exp: String, val: TypedValue) {
        let scope = Scope::new_place_holder();
        let evaluator = Evaluator::new(Project::new(format!("test")));
        let output = evaluate_expression(exp, scope.clone(), evaluator.clone()).expect("evaluation fail");
        assert_eq!(output, val);
    }

    fn get_exp_value(exp: String) -> TypedValue {
        let scope = Scope::new_place_holder();
        let evaluator = Evaluator::new(Project::new(format!("test")));
        let output = evaluate_expression(exp, scope.clone(), evaluator.clone()).expect("evaluation fail");
        return output;
    }

    #[test]
    fn simple_operation() {
        check_exp(format!("1+2+3"), TypedValue::IntValue(6));
        check_exp(format!("1+2+3.0"), TypedValue::FloatValue(6.0));
        check_exp(format!("1+2+3-8"), TypedValue::IntValue(-2));
        check_exp(format!("1+2+3*3-8"), TypedValue::IntValue(4));
        check_exp(format!("1+2+3*3.0-8"), TypedValue::FloatValue(4.0));
        check_exp(format!("1/1"), TypedValue::IntValue(1));
        check_exp(format!("1*1"), TypedValue::IntValue(1));
        check_exp(format!("10%3"), TypedValue::IntValue(1));
        check_exp(format!("\"hello, \" + \"world\""), TypedValue::StringValue(format!("hello, world")));
        check_exp(format!("{{1,2,3}}"), TypedValue::Array(vec![TypedValue::IntValue(1),TypedValue::IntValue(2),TypedValue::IntValue(3)]));
        check_exp(format!("{{1,2,3.0}}"), TypedValue::Array(vec![TypedValue::IntValue(1),TypedValue::IntValue(2),TypedValue::FloatValue(3.0)]));
        check_exp(format!("{{1,2,\"3.0\"}}"), TypedValue::Array(vec![TypedValue::IntValue(1),TypedValue::IntValue(2),TypedValue::StringValue(String::from("3.0"))]));

    }

    #[test]
    fn simple_unary() {
        check_exp(format!("-1+2+3"), TypedValue::IntValue(4));
        check_exp(format!("1--1"), TypedValue::IntValue(2));
        check_exp(format!("--1"), TypedValue::IntValue(1));
    }

    #[test]
    fn bitwise_operation() {
        check_exp(format!("0b1111&0b0000"), TypedValue::IntValue(0b0000));
        check_exp(format!("0b1111|0b0000"), TypedValue::IntValue(0b1111));
        check_exp(format!("0b0011^0b0011"), TypedValue::IntValue(0b0000));
    }

    #[test]
    fn array_operation() {
        check_exp(format!("{{1,2}} + {{3,4}}"), TypedValue::Array(vec![TypedValue::IntValue(1),TypedValue::IntValue(2),TypedValue::IntValue(3),TypedValue::IntValue(4)]));
        check_exp(format!("1 + {{2,3,4}}"), TypedValue::Array(vec![TypedValue::IntValue(1),TypedValue::IntValue(2),TypedValue::IntValue(3),TypedValue::IntValue(4)]));
        check_exp(format!("{{1,2,3}} + 4"), TypedValue::Array(vec![TypedValue::IntValue(1),TypedValue::IntValue(2),TypedValue::IntValue(3),TypedValue::IntValue(4)]));
        check_exp(format!("{{1,2,3}} + 4.0"), TypedValue::Array(vec![TypedValue::IntValue(1),TypedValue::IntValue(2),TypedValue::IntValue(3),TypedValue::FloatValue(4.0)]));
        check_exp(format!("{{1,2,3}} + 4.0*5"), TypedValue::Array(vec![TypedValue::IntValue(1),TypedValue::IntValue(2),TypedValue::IntValue(3),TypedValue::FloatValue(20.0)]));
    }

    #[test]
    fn logic_operations() {
        check_exp(format!("1==1 && 2==2"), TypedValue::BoolValue(true));
        check_exp(format!("1 < 1 && 2==2"), TypedValue::BoolValue(false));
        check_exp(format!("1 < 1 || 2==2"), TypedValue::BoolValue(true));
        check_exp(format!("{{1,2,3}} == {{1,2,3}}"), TypedValue::BoolValue(true));
        check_exp(format!("{{1,2,3}} == {{1,2,3,4}}"), TypedValue::BoolValue(false));
        check_exp(format!("{{1,2,3}} == {{1,2,4}}"), TypedValue::BoolValue(false));
        check_exp(format!("{{1,2,3}} == {{1,2,4.0}}"), TypedValue::BoolValue(false));
        check_exp(format!("{{1.0,2.0,3.0,4.0}} == {{1.0,2.0}} + 3.0 + 4.0"), TypedValue::BoolValue(true));
        check_exp(format!("{{1.0,2.0,\"3.0\",4.0}} == {{1.0,2.0}} + \"3.0\" + 4.0"), TypedValue::BoolValue(true));
    }

    #[test]
    fn assert_eq_values() {
        assert!(get_exp_value(format!("1+2*9")) == get_exp_value(format!("19")));
        assert!(get_exp_value(format!("1+2*9")) != get_exp_value(format!("19.0")));
        assert!(get_exp_value(format!("(1+2)*9")) == get_exp_value(format!("27")));
        assert!(get_exp_value(format!("{{1,2+3, 4.0*9, {{1,2,3}} + 1}}")) == get_exp_value(format!("{{1,5,36.0,{{1,2,3,1}}}}")));
    }

}