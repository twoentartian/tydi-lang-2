use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::deep_clone::DeepClone;
use crate::tydi_memory_representation::{CodeLocation, Attribute, TraitCodeLocationAccess, Variable, TypeIndication, Implementation, Scope, GlobalIdentifier, Port};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_access_pub, generate_get_pub, generate_set_pub, generate_name};

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum InstanceType {
    Unknown,
    SelfInst,
    ExternalInst,
}

impl DeepClone for InstanceType {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
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

    #[serde(with = "crate::serde_serialization::use_name_for_optional_arc_rwlock")]
    parent_scope: Option<Arc<RwLock<Scope>>>,
    id_in_scope: Option<String>,
}

impl GetName for Instance {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl DeepClone for Instance {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            derived_impl_var: self.derived_impl_var.clone(),    //shallow clone should be enough, avoid stack overflow
            derived_impl: self.derived_impl.clone(),            //shallow clone should be enough, avoid stack overflow
            inst_type: self.inst_type.deep_clone(),
            location_define: self.location_define.deep_clone(),
            document: self.document.deep_clone(),
            attributes: self.attributes.deep_clone(),
            parent_scope: self.parent_scope.clone(),
            id_in_scope: self.id_in_scope.deep_clone(),
        };
        return output;
    }
}

impl HasDocument for Instance {
    generate_access!(document, Option<String>, get_document, set_document);
}

impl TraitCodeLocationAccess for Instance {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl GlobalIdentifier for Instance {
    generate_access!(parent_scope, Option<Arc<RwLock<Scope>>>, get_parent_scope, set_parent_scope);
    generate_access!(id_in_scope, Option<String>, get_id_in_scope, set_id_in_scope);
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
            parent_scope: None,
            id_in_scope: None,
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
            parent_scope: None,
            id_in_scope: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_with_derived_implementation(name: String, derived_implementation: Arc<RwLock<Implementation>>) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name.clone(),
            derived_impl_var: Variable::new_place_holder(),
            derived_impl: Some(derived_implementation),
            inst_type: InstanceType::ExternalInst,
            location_define: CodeLocation::new_unknown(),
            document: None,
            attributes: vec![],
            parent_scope: None,
            id_in_scope: None,
        };
        return Arc::new(RwLock::new(output));
    }

    generate_set_pub!(name, String, set_name);
    generate_access_pub!(attributes, Vec<Attribute>, get_attributes, set_attributes);
    generate_access_pub!(derived_impl_var, Arc<RwLock<Variable>>, get_derived_impl_var, set_derived_impl_var);
    generate_access_pub!(derived_impl, Option<Arc<RwLock<Implementation>>>, get_derived_impl, set_derived_impl);
    generate_access_pub!(inst_type, InstanceType, get_inst_type, set_inst_type);
    generate_access_pub!(id_in_scope, Option<String>, get_id_in_scope, set_id_in_scope);

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

//interfaces for quick access
impl Instance {
    pub fn get_all_ports(&self) -> Vec<Arc<RwLock<Port>>> {
        let derived_miplementation = self.derived_impl.clone();
        assert!(derived_miplementation.is_some(), "instance ({}) is not evaluated when getting ports", self.get_name());
        let derived_miplementation = derived_miplementation.unwrap();
        return derived_miplementation.read().unwrap().get_all_ports();
    }
}