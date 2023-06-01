use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize};

use crate::tydi_memory_representation::{Streamlet, TemplateArg, CodeLocation, Scope, ScopeType, GetScope, Attribute, TraitCodeLocationAccess, Variable, TypeIndication};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_access_pub, generate_get_pub, generate_set_pub, generate_name};

#[derive(Clone, Debug, Serialize)]
pub struct Implementation {
    name: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    scope: Arc<RwLock<Scope>>,

    #[serde(with = "crate::serde_serialization::use_name_for_arc_rwlock")]
    derived_streamlet_var: Arc<RwLock<Variable>>,

    #[serde(with = "crate::serde_serialization::use_name_for_optional_arc_rwlock")]
    derived_streamlet: Option<Arc<RwLock<Streamlet>>>,

    location_define: CodeLocation,

    document: Option<String>,

    template_args: Option<BTreeMap<usize, TemplateArg>>,

    attributes: Vec<Attribute>,
}

impl GetName for Implementation {
    fn get_name(&self) -> String {
        return self.name.clone();
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

impl Implementation {
    pub fn new(name: String, streamlet_exp: String, parent_scope: Arc<RwLock<Scope>>) -> Arc<RwLock<Self>> {
        let mut output = Self {
            name: name.clone(),
            scope: Scope::new(format!("implementation_{}", name.clone()), ScopeType::ImplementationScope, parent_scope.clone()),
            derived_streamlet_var: Variable::new_place_holder(),
            derived_streamlet: None,
            location_define: CodeLocation::new_unknown(),
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
            scope: Scope::new_place_holder(),
            derived_streamlet_var: Variable::new_place_holder(),
            derived_streamlet: None,
            location_define: CodeLocation::new_unknown(),
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