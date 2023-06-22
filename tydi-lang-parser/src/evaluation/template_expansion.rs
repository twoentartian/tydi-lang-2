use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use crate::deep_clone::DeepClone;
use crate::error::TydiLangError;
use crate::generate_name::{generate_init_value, generate_template_instance_name, generate_template_instance_name_based_on_old_name};
use crate::trait_common::GetName;
use crate::tydi_memory_representation::{Variable, TypedValue, Scope, TraitCodeLocationAccess, LogicType, GetScope, TypeIndication, EvaluationStatus};


pub fn try_template_expansion(template_var: Arc<RwLock<Variable>>, template_exps: &Option<BTreeMap<usize, TypedValue>>, scope: Arc<RwLock<Scope>>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let template_var_type = template_var.read().unwrap().get_value();

    let template_args;
    let new_instance_scope;
    let new_instance_var;

    match &template_var_type {
        TypedValue::LogicTypeValue(logic_type) =>{
            let logic_type = logic_type.read().unwrap().clone();
            match logic_type {
                LogicType::LogicGroupType(group_type) => {
                    //deep clone
                    let mut cloned_group_type = group_type.read().unwrap().deep_clone();
                    cloned_group_type.set_template_args(None);
                    new_instance_scope = cloned_group_type.get_scope();
                    template_args = group_type.read().unwrap().get_template_args();
                    let cloned_group_var = LogicType::LogicGroupType(Arc::new(RwLock::new(cloned_group_type)));
                    new_instance_var = Variable::new_builtin(generate_init_value(), TypedValue::LogicTypeValue(Arc::new(RwLock::new(cloned_group_var))));
                },
                LogicType::LogicUnionType(union_type) => {
                    todo!()
                },
                _ => return Ok(template_var.clone()),   //this type has no template
            }
        },
        TypedValue::Streamlet(streamlet) => todo!(),
        TypedValue::Implementation(implementation) => todo!(),
        _ => return Ok(template_var.clone()),   //this type has no template
    }

    //check template exps match args
    {
        if template_args.is_none() && template_exps.is_none() {
            return Ok(template_var);
        }
        if template_args.is_none() && !template_exps.is_none() {
            return Err(TydiLangError::new(format!("variable {} is not a template, but no template expression are given", template_var.read().unwrap().get_name()), template_var.read().unwrap().get_code_location()));
        }
        if !template_args.is_none() && template_exps.is_none() {
            return Err(TydiLangError::new(format!("variable {} is a template, but no template expression are given", template_var.read().unwrap().get_name()), template_var.read().unwrap().get_code_location()));
        }
        let template_args = template_args.as_ref().unwrap();
        let template_exps = template_exps.as_ref().unwrap();
        if template_args.len() != template_exps.len() {
            return Err(TydiLangError::new(format!("variable {} has {} template args, but provide {} expression(s)", template_var.read().unwrap().get_name(), template_args.len(), template_exps.len()), template_var.read().unwrap().get_code_location()));
        }
        for i in 0..template_args.len() {
            let template_arg_type = template_args.get(&i).expect("bug: template arg index not from 0 to n").get_type_indication();
            let template_arg_exp = template_exps.get(&i).expect("bug: template exp index not from 0 to n");
            if !template_arg_type.is_compatible_with_typed_value(template_arg_exp) {
                return Err(TydiLangError::new(format!("var: {}, template argument index {}, expected {}, get {}", template_var.read().unwrap().get_name(), i, template_arg_type.to_string(), template_arg_exp.get_brief_info()), template_var.read().unwrap().get_code_location()));
            }
        }

        //set new_instance_var
        {
            let mut new_instance_var_write = new_instance_var.write().unwrap();
            new_instance_var_write.set_name(generate_template_instance_name(template_var.clone(), template_exps));
            new_instance_var_write.set_code_location(template_var.read().unwrap().get_code_location());
        }

        //set new instance scope
        {
            let mut new_instance_scope_write = new_instance_scope.write().unwrap();
            let old_name = new_instance_scope_write.get_name();
            new_instance_scope_write.set_name(generate_template_instance_name_based_on_old_name(old_name, template_exps));
        }
    }

    //add template_args to the cloned_group_type_scope
    {
        let template_args = template_args.as_ref().unwrap();
        let template_exps = template_exps.as_ref().unwrap();
        for i in 0..template_args.len() {
            let exp = template_exps.get(&i).unwrap();
            let arg = template_args.get(&i).unwrap();
            let temp_var = Variable::new_builtin(arg.get_name(), exp.clone());
            {
                let mut temp_var_write = temp_var.write().unwrap();
                temp_var_write.set_evaluated(EvaluationStatus::Evaluated);
            }
            new_instance_scope.write().unwrap().add_var(temp_var)?;
        }
    }

    scope.write().unwrap().add_var(new_instance_var.clone())?;
    return Ok(new_instance_var);
}

