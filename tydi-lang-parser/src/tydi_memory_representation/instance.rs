use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::tydi_memory_representation::{CodeLocation, Attribute, TraitCodeLocationAccess, Variable, TypeIndication};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_access_pub, generate_get_pub, generate_set_pub, generate_name};


#[derive(Clone, Debug, Serialize)]
pub struct Instance {
    name: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    derived_implementation: Arc<RwLock<Variable>>,

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
            derived_implementation: Variable::new_place_holder(),
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
            derived_implementation: Variable::new_place_holder(),
            location_define: CodeLocation::new_unknown(),
            document: None,
            attributes: vec![],
        };
        return Arc::new(RwLock::new(output));
    }

    generate_set_pub!(name, String, set_name);
    generate_access_pub!(attributes, Vec<Attribute>, get_attributes, set_attributes);
    generate_access_pub!(derived_implementation, Arc<RwLock<Variable>>, get_derived_implementation, set_derived_implementation);

    pub fn set_derived_implementation_exp(&mut self, derived_implementation_exp: String, code_location: CodeLocation) {
        let streamlet_var = Variable::new(format!("derived_implementation_exp_of_{}", self.name.clone()), Some(derived_implementation_exp));
        {
            let mut streamlet_var_write = streamlet_var.write().unwrap();
            streamlet_var_write.set_code_location(code_location);
            streamlet_var_write.set_type_indication(TypeIndication::AnyImplementation);
        }
        self.derived_implementation = streamlet_var;
    }

}