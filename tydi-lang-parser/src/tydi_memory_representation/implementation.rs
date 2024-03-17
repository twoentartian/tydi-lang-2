use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::deep_clone::{DeepClone, DeepClone_ArcLock};
use crate::error::TydiLangError;
use crate::tydi_memory_representation::{Streamlet, Net, TemplateArg, CodeLocation, Scope, ScopeType, GetScope, Attribute, TraitCodeLocationAccess, Variable, TypeIndication, TypedValue, Port, GlobalIdentifier, Instance, EvaluationStatus};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_access_pub, generate_get_pub, generate_set_pub, generate_name};


#[derive(Clone, Debug, strum::IntoStaticStr)]
pub enum ImplementationType {
    Unknown,
    Normal,
    Template,
    TemplateInstance(Arc<RwLock<Implementation>>, BTreeMap<usize, TypedValue>),
}

impl DeepClone for ImplementationType {
    fn deep_clone(&self) -> Self {
        return self.clone();    //shallow clone should be enough
    }
}

impl Serialize for ImplementationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct("ImplementationType", 2)?;
        let enum_type_str: &'static str = self.into();
        state.serialize_field("type", enum_type_str)?;

        match self {
            ImplementationType::Unknown => {
                //skip
            },
            ImplementationType::Normal => {
                //skip
            },
            ImplementationType::Template => {
                //skip
            },
            ImplementationType::TemplateInstance(template, arg_map) => {
                state.serialize_field("template_name", &template.read().unwrap().get_name())?;
                state.serialize_field("args", arg_map)?;
            },
        }
        state.end()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Implementation {
    name: String,

    impl_type: ImplementationType,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    scope: Arc<RwLock<Scope>>,

    #[serde(with = "crate::serde_serialization::use_name_for_arc_rwlock")]
    derived_streamlet_var: Arc<RwLock<Variable>>,

    #[serde(with = "crate::serde_serialization::use_name_for_optional_arc_rwlock")]
    derived_streamlet: Option<Arc<RwLock<Streamlet>>>,

    location_define: CodeLocation,

    #[serde(with = "crate::serde_serialization::use_name_for_optional_arc_rwlock")]
    parent_scope: Option<Arc<RwLock<Scope>>>,
    id_in_scope: Option<String>,

    document: Option<String>,

    template_args: Option<BTreeMap<usize, TemplateArg>>,

    attributes: Vec<Attribute>,
}

impl GetName for Implementation {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl DeepClone for Implementation {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            impl_type: self.impl_type.deep_clone(),
            scope: self.scope.read().unwrap().deep_clone_arclock(),
            derived_streamlet_var: self.derived_streamlet_var.deep_clone(),
            derived_streamlet: self.derived_streamlet.deep_clone(),
            location_define: self.location_define.deep_clone(),
            parent_scope: self.parent_scope.clone(),
            id_in_scope: self.id_in_scope.deep_clone(),
            document: self.document.deep_clone(),
            template_args: self.template_args.deep_clone(),
            attributes: self.attributes.deep_clone(),
        };
        return output;
    }
}

impl HasDocument for Implementation {
    generate_access!(document, Option<String>, get_document, set_document);
}

impl TraitCodeLocationAccess for Implementation {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl GetScope for Implementation {
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);
}

impl GlobalIdentifier for Implementation {
    generate_access!(parent_scope, Option<Arc<RwLock<Scope>>>, get_parent_scope, set_parent_scope);
    generate_access!(id_in_scope, Option<String>, get_id_in_scope, set_id_in_scope);
}

impl Implementation {
    pub fn new(name: String, streamlet_exp: String, impl_type: ImplementationType, parent_scope: Arc<RwLock<Scope>>) -> Arc<RwLock<Self>> {
        let mut output = Self {
            name: name.clone(),
            impl_type: impl_type,
            scope: Scope::new(format!("implementation_{}", name.clone()), ScopeType::ImplementationScope, parent_scope.clone()),
            derived_streamlet_var: Variable::new_place_holder(),
            derived_streamlet: None,
            location_define: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            document: None,
            template_args: None,
            attributes: vec![],
        };
        output.set_derived_streamlet_exp(streamlet_exp, CodeLocation::new_unknown());
        return Arc::new(RwLock::new(output));
    }

    pub fn new_place_holder() -> Arc<RwLock<Self>> {
        let output = Self {
            name: generate_name::generate_init_value(),
            impl_type: ImplementationType::Unknown,
            scope: Scope::new_place_holder(),
            derived_streamlet_var: Variable::new_place_holder(),
            derived_streamlet: None,
            location_define: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            document: None,
            template_args: None,
            attributes: vec![],
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn get_brief_info(&self) -> String {
        let derived_streamlet_name = self.get_derived_streamlet_var().read().unwrap().get_name();
        return format!("Impl({})({})", &self.name, derived_streamlet_name);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name.clone();
        self.scope.write().unwrap().set_name(format!("implementation_{}", name.clone()));
    }

    generate_access_pub!(impl_type, ImplementationType, get_impl_type, set_impl_type);
    generate_access_pub!(template_args, Option<BTreeMap<usize, TemplateArg>>, get_template_args, set_template_args);
    generate_access_pub!(attributes, Vec<Attribute>, get_attributes, set_attributes);
    generate_access_pub!(derived_streamlet_var, Arc<RwLock<Variable>>, get_derived_streamlet_var, set_derived_streamlet_var);
    generate_access_pub!(derived_streamlet, Option<Arc<RwLock<Streamlet>>>, get_derived_streamlet, set_derived_streamlet);

    pub fn set_derived_streamlet_exp(&mut self, streamlet_exp: String, code_location: CodeLocation) {
        let streamlet_var = Variable::new(format!("streamlet_exp_of_{}", self.name.clone()), Some(streamlet_exp));
        {
            let mut streamlet_var_write = streamlet_var.write().unwrap();
            streamlet_var_write.set_type_indication(TypeIndication::AnyStreamlet);
            streamlet_var_write.set_code_location(code_location);
        }
        self.derived_streamlet_var = streamlet_var;
    }
}

//interfaces for quick access
impl Implementation {
    pub fn get_all_ports(&self) -> Vec<Arc<RwLock<Port>>> {
        let derived_streamlet = self.derived_streamlet.clone();
        assert!(derived_streamlet.is_some(), "implementation ({}) is not evaluated when getting ports", self.get_name());
        let derived_streamlet = derived_streamlet.unwrap();
        return derived_streamlet.read().unwrap().get_all_ports();
    }

    pub fn get_all_nets(&self) -> Vec<Arc<RwLock<Net>>> {
        let implementation_scope = self.get_scope();
        let all_vars_in_implementation = implementation_scope.read().unwrap().get_variables();

        let mut all_nets = vec![];
        for (var_name, each_var) in &all_vars_in_implementation {
            {
                let evaluation_status = each_var.read().unwrap().get_evaluated();
                assert!(evaluation_status == EvaluationStatus::Evaluated || evaluation_status == EvaluationStatus::Predefined || each_var.read().unwrap().get_name() == "self", "variable ({}) should be evaluated before sugaring", var_name);
            }
            let var_value = each_var.read().unwrap().get_value();
            match &var_value {
                crate::tydi_memory_representation::TypedValue::Net(net) => all_nets.push(net.clone()),
                crate::tydi_memory_representation::TypedValue::Array(array) => {
                    for var_value in array {
                        match &var_value {
                            crate::tydi_memory_representation::TypedValue::Net(net) => all_nets.push(net.clone()),
                            _ => (),
                        }
                    }
                },
                _ => (),
            }
        }

        return all_nets;
    }

    pub fn add_net(&self, parent_impl_arc: Arc<RwLock<Implementation>>, net: Arc<RwLock<Net>>) -> Result<(), TydiLangError> {
        let implementation_scope = self.get_scope();

        net.write().unwrap().set_parent_impl(Some(parent_impl_arc.clone()));
        let new_net_var = Variable::new_net(net.read().unwrap().get_name(), net.clone());
        new_net_var.write().unwrap().set_evaluated(EvaluationStatus::Evaluated);
        new_net_var.write().unwrap().set_parent_scope(Some(implementation_scope.clone()));
        implementation_scope.write().unwrap().add_var(new_net_var.clone())?;

        return Ok(());
    }

    pub fn add_instance(&self, parent_impl_arc: Arc<RwLock<Implementation>>, inst: Arc<RwLock<Instance>>) -> Result<(), TydiLangError> {
        let implementation_scope = self.get_scope();

        inst.write().unwrap().set_parent_scope(Some(implementation_scope.clone()));
        let new_voider_var = Variable::new_instance(inst.read().unwrap().get_name(), inst.clone());
        new_voider_var.write().unwrap().set_evaluated(EvaluationStatus::Evaluated);
        new_voider_var.write().unwrap().set_parent_scope(Some(implementation_scope.clone()));
        implementation_scope.write().unwrap().add_var(new_voider_var.clone())?;
        
        return Ok(());
    }

    pub fn get_all_instances(&self) -> Vec<Arc<RwLock<Instance>>> {
        let implementation_scope = self.get_scope();
        let all_vars_in_implementation = implementation_scope.read().unwrap().get_variables();

        let mut all_instances = vec![];
        for (var_name, each_var) in &all_vars_in_implementation {
            {
                let evaluation_status = each_var.read().unwrap().get_evaluated();
                assert!(evaluation_status == EvaluationStatus::Evaluated || evaluation_status == EvaluationStatus::Predefined || each_var.read().unwrap().get_name() == "self", "variable ({}) should be evaluated before sugaring", var_name);
            }
            let var_value = each_var.read().unwrap().get_value();
            match &var_value {
                crate::tydi_memory_representation::TypedValue::Instance(inst) => all_instances.push(inst.clone()),
                crate::tydi_memory_representation::TypedValue::Array(array) => {
                    for var_value in array {
                        match &var_value {
                            crate::tydi_memory_representation::TypedValue::Instance(inst) => all_instances.push(inst.clone()),
                            _ => (),
                        }
                    }
                },
                _ => (),
            }
        }

        return all_instances;
    }

}