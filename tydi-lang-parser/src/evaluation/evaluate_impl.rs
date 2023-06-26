use std::sync::{Arc, RwLock};

use crate::trait_common::{GetName, HasDocument};
use crate::tydi_memory_representation::{InstanceType, Scope, TypedValue, GetScope, Implementation, TraitCodeLocationAccess, Variable, Instance, Net, CodeLocation, ScopeType};

use crate::error::TydiLangError;

use super::{Evaluator, evaluate_var, evaluate_scope, ScopeOwner};


pub fn evaluate_impl(target: Arc<RwLock<Implementation>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
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

    let rhs_port_var = target.read().unwrap().get_sink();
    let rhs_port_value = evaluate_var(rhs_port_var.clone(), scope.clone(), evaluator.clone())?;
    let rhs_port = match &rhs_port_value {
        TypedValue::Port(port) => port.clone(),
        _ => return Err(TydiLangError::new(format!("{} is not a port, but used in defining net {}", lhs_port_var.read().unwrap().get_name(), target.read().unwrap().get_name()), target.read().unwrap().get_code_location())),
    };

    {
        let mut target_write = target.write().unwrap();
        target_write.set_source_port(Some(lhs_port.clone()));
        target_write.set_sink_port(Some(rhs_port.clone()));
    }

    return Ok(TypedValue::Net(target.clone()));
}