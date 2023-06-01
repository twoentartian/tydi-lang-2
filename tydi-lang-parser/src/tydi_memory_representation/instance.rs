use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::tydi_memory_representation::{CodeLocation, Attribute, TraitCodeLocationAccess, Variable, TypeIndication, Implementation};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_access_pub, generate_get_pub, generate_set_pub, generate_name};

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum InstanceType {
    Unknown,
    SelfInst,
    ExternalInst,
}

#[derive(Clone, Debug, Serialize)]
pub struct Instance {
    name: String,

    #[serde(with = "crate::serde_serialization::use_name_for_arc_rwlock")]
    derived_impl_var: Arc<RwLock<Variable>>,

    #[serde(with = "crate::serde_serialization::use_name_for_optional_arc_rwlock")]
    derived_impl: Option<Arc<RwLock<Implementation>>>,

    inst_type: InstanceType,

    location_define: CodeLocation,

    document: Option<String>,

    attributes: Vec<Attribute>,
}

impl GetName for Instance {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl HasDocument for Instance {
    generate_access!(document, Option<String>, get_document, set_document);
}

impl TraitCodeLocationAccess for Instance {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl Instance {
    pub fn new(name: String, derived_implementation_exp: String) -> Arc<RwLock<Self>> {
        let mut output = Self {
            name: name.clone(),
            derived_impl_var: Variable::new_place_holder(),
            derived_impl: None,
            inst_type: InstanceType::Unknown,
            location_define: CodeLocation::new_unknown(),
            document: None,
            attributes: vec![],
        };
        output.set_derived_implementation_exp(derived_implementation_exp, CodeLocation::new_unknown());
        return Arc::new(RwLock::new(output));
    }

    pub fn new_place_holder() -> Arc<RwLock<Self>> {
        let output = Self {
            name: generate_name::generate_init_value(),
            derived_impl_var: Variable::new_place_holder(),
            derived_impl: None,
            inst_type: InstanceType::Unknown,
            location_define: CodeLocation::new_unknown(),
            document: None,
            attributes: vec![],
        };
        return Arc::new(RwLock::new(output));
    }

    generate_set_pub!(name, String, set_name);
    generate_access_pub!(attributes, Vec<Attribute>, get_attributes, set_attributes);
    generate_access_pub!(derived_impl_var, Arc<RwLock<Variable>>, get_derived_impl_var, set_derived_impl_var);
    generate_access_pub!(derived_impl, Option<Arc<RwLock<Implementation>>>, get_derived_impl, set_derived_impl);
    generate_access_pub!(inst_type, InstanceType, get_inst_type, set_inst_type);

    pub fn set_derived_implementation_exp(&mut self, derived_implementation_exp: String, code_location: CodeLocation) {
        let streamlet_var = Variable::new(format!("derived_implementation_exp_of_{}", self.name.clone()), Some(derived_implementation_exp));
        {
            let mut streamlet_var_write = streamlet_var.write().unwrap();
            streamlet_var_write.set_code_location(code_location);
            streamlet_var_write.set_type_indication(TypeIndication::AnyImplementation);
        }
        self.derived_impl_var = streamlet_var;
    }

}