use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize};

use crate::deep_clone::{DeepClone, DeepClone_ArcLock};
use crate::tydi_memory_representation::{Scope, GetScope, ScopeType, CodeLocation, TraitCodeLocationAccess, TemplateArg, GlobalIdentifier};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_name, generate_access_pub, generate_get_pub, generate_set_pub};

#[derive(Clone, Debug, Serialize)]
pub struct LogicGroup {
    name: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    scope: Arc<RwLock<Scope>>,

    location_define: CodeLocation,

    document: Option<String>,

    template_args: Option<BTreeMap<usize, TemplateArg>>,

    #[serde(with = "crate::serde_serialization::use_name_for_optional_arc_rwlock")]
    parent_scope: Option<Arc<RwLock<Scope>>>,
    id_in_scope: Option<String>,
}

impl GetName for LogicGroup {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl DeepClone for LogicGroup {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            scope: self.scope.read().unwrap().deep_clone_arclock(),
            location_define: self.location_define.deep_clone(),
            document: self.document.deep_clone(),
            template_args: self.template_args.deep_clone(),
            parent_scope: self.parent_scope.clone(),
            id_in_scope: self.id_in_scope.deep_clone(),
        };
        return output;
    }
}

impl HasDocument for LogicGroup {
    generate_access!(document, Option<String>, get_document, set_document);
}

impl TraitCodeLocationAccess for LogicGroup {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl GetScope for LogicGroup {
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);
}

impl GlobalIdentifier for LogicGroup {
    generate_access!(parent_scope, Option<Arc<RwLock<Scope>>>, get_parent_scope, set_parent_scope);
    generate_access!(id_in_scope, Option<String>, get_id_in_scope, set_id_in_scope);
}

impl LogicGroup {
    pub fn new(name: String, parent_scope: Arc<RwLock<Scope>>) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name.clone(),
            scope: Scope::new(format!("logic_group_{}", name.clone()), ScopeType::GroupScope, parent_scope.clone()),
            location_define: CodeLocation::new_unknown(),
            document: None,
            template_args: None,
            parent_scope: None,
            id_in_scope: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_place_holder() -> Arc<RwLock<Self>> {
        let output = Self {
            name: generate_name::generate_init_value(),
            scope: Scope::new_place_holder(),
            location_define: CodeLocation::new_unknown(),
            document: None,
            template_args: None,
            parent_scope: None,
            id_in_scope: None,
        };
        return Arc::new(RwLock::new(output));
    }

    generate_access_pub!(template_args, Option<BTreeMap<usize, TemplateArg>>, get_template_args, set_template_args);

}
