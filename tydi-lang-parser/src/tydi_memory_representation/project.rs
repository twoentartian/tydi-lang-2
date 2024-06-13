use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize};

use crate::error::TydiLangError;
use crate::evaluation::{Evaluator, evaluate_var, EvaluationTrace};
use crate::{generate_get_pub, generate_set_pub, generate_access_pub};
use crate::tydi_memory_representation::{Package, CodeLocation, GetScope, Scope, ScopeRelationType, Variable, SrcInfo};

#[derive(Clone, Debug, Serialize)]
pub struct ProjectItem {
    pub item_name: String,
    pub package_name: String,
}

impl ProjectItem {
    pub fn new(item_name: String, package_name: String) -> Self {
        return Self {
            item_name: item_name,
            package_name: package_name,
        };
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Project {
    name: String,

    #[serde(with = "crate::serde_serialization::arc_rwlock_in_btree_map_value")]
    packages: BTreeMap<String, Arc<RwLock<Package>>>,

    #[serde(skip)]
    self_arc: Option<Arc<RwLock<Project>>>,

    #[serde(skip)]
    sugaring_entry_point: BTreeMap<usize, ProjectItem>,
}

impl Project {
    pub fn new(name: String) -> Arc<RwLock<Self>> {
        let output = Project { 
            name: name, 
            packages: BTreeMap::new(),
            self_arc: None,
            sugaring_entry_point: BTreeMap::new(),
        };
        let project_arc = Arc::new(RwLock::new(output));
        project_arc.write().unwrap().self_arc = Some(project_arc.clone());
        return project_arc;
    }

    pub fn add_package(&mut self, file_path: String, file_content: String) -> Result<(), TydiLangError> {
        let src_info = SrcInfo::new(file_path.clone(), file_content.clone());
        let file_package = crate::tydi_lang_src_to_memory_representation::tydi_lang_src_to_memory_representation(file_content.clone(), src_info)?;
        {
            let mut file_package_write = file_package.write().unwrap();
            file_package_write.set_file_path(file_path.clone());
            file_package_write.set_file_content(file_content.clone());
        }
        let package_name = file_package.read().unwrap().get_name();
        self.packages.insert(package_name.clone(), file_package.clone());
        return Ok(());
    }

    pub fn get_pretty_json(&self) -> String {
        let json_output = serde_json::to_string_pretty(self).ok().unwrap();
        return json_output;
    }

    pub fn evaluate_target(&self, target_name: String, package_name: String) -> Result<Arc<RwLock<Evaluator>>, TydiLangError> {
        let target_package = self.packages.get(&package_name);
        if target_package.is_none() {
            return Err(TydiLangError::new(format!("no such package: {}", &package_name), CodeLocation::new_unknown()));
        }
        let target_package = target_package.unwrap();
        let target_package_scope = target_package.read().unwrap().get_scope();

        let evaluator = match &self.self_arc {
            Some(self_arc) => Evaluator::new(self_arc.clone()),
            None => unreachable!(),
        };

        evaluator.write().unwrap().add_trace(EvaluationTrace::new_region_begin(format!("evaluation")));
        let (target_var, target_var_scope) = Scope::resolve_identifier(&target_name, &None, &CodeLocation::new_unknown(), target_package_scope.clone(), target_package_scope.clone(), ScopeRelationType::resolve_id_default(), evaluator.clone())?;
        evaluate_var(target_var.clone(), target_var_scope.clone(), evaluator.clone())?;
        evaluator.write().unwrap().add_trace(EvaluationTrace::new_region_end(format!("evaluation")));

        return Ok(evaluator);
    }

    pub fn get_variable(&self, package_name: String, target_name: String) -> Result<Arc<RwLock<Variable>>, String> {
        let packages = self.get_packages();
        let target_package = packages.get(&package_name);
        if target_package.is_none() {
            return Err(format!("package {} not found", &package_name));
        }
        let target_package = target_package.unwrap();
        let target_package_scope = target_package.read().unwrap().get_scope();
        let all_variables_in_target_package = target_package_scope.read().unwrap().get_variables();
        let target_variable = all_variables_in_target_package.get(&target_name);
        if target_variable.is_none() {
            return Err(format!("variable {} not found in package {}", &target_name, &package_name));
        }
        let target_variable = target_variable.unwrap();
        return Ok(target_variable.clone());
    }

    generate_get_pub!(packages, BTreeMap<String, Arc<RwLock<Package>>>, get_packages);
    generate_get_pub!(self_arc, Option<Arc<RwLock<Project>>>, get_self_arc);
    generate_access_pub!(sugaring_entry_point, BTreeMap<usize, ProjectItem>, get_sugaring_entry_point, set_sugaring_entry_point);
}

