use std::sync::{Arc, RwLock};

use evaluate_var::evaluate_id_in_typed_value;

use crate::generate_name::generate_init_value;
use crate::trait_common::{GetName, HasDocument};
use crate::tydi_memory_representation::{InstanceType, Scope, TypedValue, GetScope, Implementation, TraitCodeLocationAccess, Variable, Instance, Net, CodeLocation, ScopeType, PortOwner, ScopeRelationType};

use crate::error::TydiLangError;

use super::{Evaluator, evaluate_var, evaluate_scope, ScopeOwner, evaluate_expression};


pub fn evaluate_impl(target: Arc<RwLock<Implementation>>, _scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let impl_scope = target.read().unwrap().get_scope();

    //find derived streamlet
    let target_derived_streamlet = target.read().unwrap().get_derived_streamlet_var();
    let derived_streamlet_typed_value = evaluate_var(target_derived_streamlet.clone(), impl_scope.clone(), evaluator.clone())?;
    let derived_streamlet = match &derived_streamlet_typed_value {
        TypedValue::Streamlet(s) => s,
        _ => return Err(TydiLangError::new(format!("{} is not a streamlet, but used in defining impl({})", target_derived_streamlet.read().unwrap().get_name(), target.read().unwrap().get_name()), target_derived_streamlet.read().unwrap().get_code_location()))
    };
    {
        let mut target_write = target.write().unwrap();
        target_write.set_derived_streamlet(Some(derived_streamlet.clone()));
    }

    //add scope relationship
    {
        let derived_streamlet_scope = derived_streamlet.read().unwrap().get_scope();
        let mut impl_scope_write = impl_scope.write().unwrap();
        impl_scope_write.add_scope_relationship(derived_streamlet_scope, crate::tydi_memory_representation::ScopeRelationType::ImplToStreamletRela)?;
    }

    //create self instance
    let self_instance = Instance::new_place_holder();
    {
        let mut self_instance_write = self_instance.write().unwrap();
        self_instance_write.set_name(format!("self"));
        self_instance_write.set_derived_impl_var(Variable::new_predefined(format!("self_derived_implementation"), TypedValue::Implementation(target.clone())));
        self_instance_write.set_inst_type(InstanceType::SelfInst);
        self_instance_write.set_derived_impl(Some(target.clone()));
        self_instance_write.set_document(None);
        self_instance_write.set_code_location(CodeLocation::new_unknown());
        self_instance_write.set_attributes(vec![]);
    }

    //add self variable
    let self_var = Variable::new_builtin(format!("self"), TypedValue::Instance(self_instance));
    {
        let mut impl_scope_write = impl_scope.write().unwrap();
        impl_scope_write.add_var(self_var)?;
    }

    evaluate_scope(impl_scope.clone(), &ScopeType::ImplementationScope, &ScopeOwner::Implementation(target.clone()), impl_scope.clone(), evaluator.clone())?;

    return Ok(TypedValue::Implementation(target.clone()));
}


pub fn evaluate_instance(target: Arc<RwLock<Instance>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    //if this is a self instance
    if target.read().unwrap().get_inst_type() == InstanceType::SelfInst {
        return Ok(TypedValue::Instance(target.clone()));
    }
    
    let target_derived_impl = target.read().unwrap().get_derived_impl_var();
    let derived_impl = evaluate_var(target_derived_impl.clone(), scope.clone(), evaluator.clone())?;
    match &derived_impl {
        TypedValue::Implementation(derived_implementation) => {
            target.write().unwrap().set_derived_impl(Some(derived_implementation.clone()));
        },
        _ => return Err(TydiLangError::new(format!("{} is not an implementation, but used in defining instance({})", target_derived_impl.read().unwrap().get_name(), target.read().unwrap().get_name()), target_derived_impl.read().unwrap().get_code_location()))
    }

    {
        let mut target_write = target.write().unwrap();
        target_write.set_inst_type(InstanceType::ExternalInst);
    }

    return Ok(TypedValue::Instance(target.clone()));
}


pub fn evaluate_net(target: Arc<RwLock<Net>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    //evaluate lhs
    let lhs_port_var = target.read().unwrap().get_source();
    let lhs_port_value = evaluate_var(lhs_port_var.clone(), scope.clone(), evaluator.clone())?;
    let lhs_port = match &lhs_port_value {
        TypedValue::Port(port) => port.clone(),
        _ => return Err(TydiLangError::new(format!("{} is not a port, but used in defining net {}", lhs_port_var.read().unwrap().get_name(), target.read().unwrap().get_name()), target.read().unwrap().get_code_location())),
    };
    target.write().unwrap().set_source_port(Some(lhs_port.clone()));

    let rhs_port_var = target.read().unwrap().get_sink();
    let rhs_port_value = evaluate_var(rhs_port_var.clone(), scope.clone(), evaluator.clone())?;
    let rhs_port = match &rhs_port_value {
        TypedValue::Port(port) => port.clone(),
        _ => return Err(TydiLangError::new(format!("{} is not a port, but used in defining net {}", lhs_port_var.read().unwrap().get_name(), target.read().unwrap().get_name()), target.read().unwrap().get_code_location())),
    };
    target.write().unwrap().set_sink_port(Some(rhs_port.clone()));

    let get_port_owner_from_exp = |port_var: Arc<RwLock<Variable>>| -> Result<PortOwner, TydiLangError> {
        use crate::pest::Parser;
        use crate::tydi_parser::{Rule, TydiLangSrc};
        let port_owner_exp = port_var.read().unwrap().get_exp();
        let port_owner_exp = match port_owner_exp {
            Some(exp) => exp,
            None => return Err(TydiLangError::new(format!("{} of net {} has no port owner expression", port_var.read().unwrap().get_name(), target.read().unwrap().get_name()), target.read().unwrap().get_code_location())),
        };
        let port_owner_pest = TydiLangSrc::parse(Rule::Exp,&port_owner_exp).unwrap();
        let mut port_owner_name = generate_init_value();
        let mut counter = 0;
        for ele_exp in port_owner_pest.into_iter() {
            for element in ele_exp.into_inner().into_iter(){
                match element.as_rule() {
                    Rule::Term => {     //we only care about the first term because it's the port owner
                        if counter == 0 {
                            port_owner_name = element.as_str().to_string();
                        }
                        counter += 1;
                        break;
                    },
                    _ => (),    //ignore
                }
            }
        }
        assert!(port_owner_name != generate_init_value());
        let port_owner = if port_owner_name == String::from("self") || counter == 1 {
            PortOwner::ImplSelf
        }
        else {
            let port_owner_value = evaluate_expression(port_owner_name.clone(), scope.clone(), evaluator.clone())?;
            let port_owner_value = evaluate_id_in_typed_value(port_owner_value, ScopeRelationType::resolve_id_in_current_scope(), scope.clone(), evaluator.clone())?;
            
            let port_owner_inst = match port_owner_value {
                TypedValue::Instance(inst) => inst,
                TypedValue::RefToVar(var) => {
                    evaluate_var(var.clone(), scope.clone(), evaluator.clone())?;
                    let var_value = var.read().unwrap().get_value();
                    match var_value {
                        TypedValue::Instance(inst) => {
                            inst
                        },
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!()
            };
            PortOwner::ImplInstance(port_owner_inst)
        };
        return Ok(port_owner);
    };

    let lhs_port_owner = get_port_owner_from_exp(lhs_port_var.clone())?;
    target.write().unwrap().set_source_port_owner(lhs_port_owner);
    let rhs_port_owner = get_port_owner_from_exp(rhs_port_var.clone())?;
    target.write().unwrap().set_sink_port_owner(rhs_port_owner);

    return Ok(TypedValue::Net(target.clone()));
}
