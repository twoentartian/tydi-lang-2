use std::sync::{Arc, RwLock, atomic::AtomicUsize};
use std::collections::BTreeMap;

use crate::error::TydiLangError;
use crate::tydi_memory_representation::{TypedValue, CodeLocation, EvaluationStatus, GetScope, GlobalIdentifier, Implementation, Instance, Net, Port, PortDirection, PortOwner, Project, Scope, ScopeRelationType, TraitCodeLocationAccess, Variable};
use crate::evaluation::{Evaluator, evaluate_var, EvaluationTrace};
use crate::trait_common::GetName;

const STD_LIB_PACKAGE_NAME: &str = "std";

/*
impl void_i<type_in: type> of void_s<type_in> @External {}
 */
const STD_VOID_IMPL_NAME: &str = "void_i";

/*
impl void_i<type_in: type> of void_s<type_in> @External {}
*/
const STD_DUPLICATOR_IMPL_NAME: &str = "duplicator_i";

// automatically insert duplicators and voiders
pub fn sugaring_add_duplicator_voider(project: Arc<RwLock<Project>>, target_name: String, package_name: String) -> Result<Arc<RwLock<Evaluator>>, TydiLangError> {
    let evaluator = Evaluator::new(project.clone());

    evaluator.write().unwrap().add_trace(EvaluationTrace::new_region_begin(format!("sugaring - add duplicator and voider")));

    // if current_std_package_info.lock().unwrap().duplicator_implementation.is_none() || current_std_package_info.lock().unwrap().voider_implementation.is_none() {
    //     std_package_info::try_load_std_lib(project.clone())?;
    // }

    let project_packages = project.read().unwrap().get_packages();

    //find starting implementation and its scope
    let starting_implementation;
    {
        let target_package = project_packages.get(&package_name);
        if target_package.is_none() {
            return Err(TydiLangError::new(format!("no such package: {}", &package_name), CodeLocation::new_unknown()));
        }
        let target_package = target_package.unwrap();
        let target_package_scope = target_package.read().unwrap().get_scope();
        let (target_var, _) = Scope::resolve_identifier(&target_name, &None, &CodeLocation::new_unknown(), target_package_scope.clone(), target_package_scope.clone(), ScopeRelationType::resolve_id_default(), evaluator.clone())?;
    
        assert!(target_var.read().unwrap().get_evaluated() == EvaluationStatus::Evaluated, "implementation ({}) should be evaluated before sugaring", target_name); 
        let target_var_value = target_var.read().unwrap().get_value();
        let starting_implementation_inner = match &target_var_value {
            crate::tydi_memory_representation::TypedValue::Implementation(implementation) => implementation,
            _ => return Err(TydiLangError::new(format!("sugaring starting point ({}) should be an implementation", target_name), target_var.read().unwrap().get_code_location())),
        };
        starting_implementation = starting_implementation_inner.clone();
    }

    sugaring_add_duplicator_voider_for_single_implementation(project.clone(), starting_implementation.clone(), evaluator.clone())?;

    evaluator.write().unwrap().add_trace(EvaluationTrace::new_region_end(format!("sugaring - add duplicator and voider")));

    return Ok(evaluator);
}

fn sugaring_add_duplicator_voider_for_single_implementation(project: Arc<RwLock<Project>>, target_implementation: Arc<RwLock<Implementation>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<(), TydiLangError> {
    let mut src_port_mapping = BTreeMap::<String, Arc<RwLock<Port>>>::new();
    let mut src_port_owner_mapping = BTreeMap::<String, PortOwner>::new();
    let mut src_to_sink_port_mapping = BTreeMap::<String, Vec<Arc<RwLock<Port>>>>::new();
    let mut src_to_sink_port_owner_mapping = BTreeMap::<String, Vec<PortOwner>>::new();
    let mut src_to_sink_port_nets_mapping = BTreeMap::<String, Vec<Arc<RwLock<Net>>>>::new();

    //find all src ports on instances
    let all_instances = target_implementation.read().unwrap().get_all_instances();
    for single_instance in all_instances {
        let instance_name = single_instance.read().unwrap().get_name();

        if instance_name == "self" {
            let all_ports = single_instance.read().unwrap().get_all_ports();
            for port in all_ports {
                let port_direction = port.read().unwrap().get_direction();
                if port_direction == PortDirection::In {
                    let long_port_name = format!("{}__{}", &instance_name, port.read().unwrap().get_name());
                    assert!(src_port_mapping.get(&long_port_name).is_none(), "port name ({}) already exists", &long_port_name);
                    src_port_mapping.insert(long_port_name.clone(), port.clone());
                    src_port_owner_mapping.insert(long_port_name.clone(), PortOwner::ImplSelf);
                }
            }

            continue;   //jump to next variable
        }

        //get all ports
        let single_instance_name = single_instance.read().unwrap().get_name();
        let parent_implementation = single_instance.read().unwrap().get_derived_impl().expect(&format!("parent_implementation for instance ({}) should not be empty", &single_instance_name));
        let parent_implementation_name = parent_implementation.read().unwrap().get_name();
        let parent_streamlet = parent_implementation.read().unwrap().get_derived_streamlet().expect(&format!("parent_streamlet for implementation ({}) should not be empty", &parent_implementation_name));
        let all_ports_of_single_instance = parent_streamlet.read().unwrap().get_all_ports();
        for port in all_ports_of_single_instance {
            let port_direction = port.read().unwrap().get_direction();
            if port_direction == PortDirection::Out {
                let long_port_name = format!("{}__{}", &single_instance_name, port.read().unwrap().get_name());
                assert!(src_port_mapping.get(&long_port_name).is_none(), "port name ({}) already exists", &long_port_name);
                src_port_mapping.insert(long_port_name.clone(), port.clone());
                src_port_owner_mapping.insert(long_port_name.clone(), PortOwner::ImplInstance(single_instance.clone()));
            }
        }
    }

    //find all nets
    let all_nets = target_implementation.read().unwrap().get_all_nets();
    for single_net in all_nets {
        let src_port = single_net.read().unwrap().get_source_port().unwrap();
        let src_port_owner = single_net.read().unwrap().get_source_port_owner();
        let src_port_owner_name = match src_port_owner {
            crate::tydi_memory_representation::PortOwner::ImplSelf => String::from("self"),
            crate::tydi_memory_representation::PortOwner::ImplInstance(inst) => inst.read().unwrap().get_name(),
            _ => unreachable!(),
        };
        let src_port_long_name = format!("{}__{}", src_port_owner_name, src_port.read().unwrap().get_name());
        let sink_port = single_net.read().unwrap().get_sink_port().unwrap();
        let sink_port_owner = single_net.read().unwrap().get_sink_port_owner();

        // push to src_to_sink_port_mapping
        {
            let src_to_sink_port_mapping_entry = src_to_sink_port_mapping.get_mut(&src_port_long_name);
            match src_to_sink_port_mapping_entry {
                Some(ports) => {
                    ports.push(sink_port.clone());
                },
                None => {
                    src_to_sink_port_mapping.insert(src_port_long_name.clone(), vec![sink_port.clone()]);
                },
            }
        }

        // push to src_to_sink_port_owner_mapping
        {
            let entry = src_to_sink_port_owner_mapping.get_mut(&src_port_long_name);
            match entry {
                Some(owners) => {
                    owners.push(sink_port_owner.clone());
                },
                None => {
                    src_to_sink_port_owner_mapping.insert(src_port_long_name.clone(), vec![sink_port_owner.clone()]);
                },
            }
        }

        // push to src_to_sink_port_nets_mapping
        {
            let entry = src_to_sink_port_nets_mapping.get_mut(&src_port_long_name);
            match entry {
                Some(owners) => {
                    owners.push(single_net.clone());
                },
                None => {
                    src_to_sink_port_nets_mapping.insert(src_port_long_name.clone(), vec![single_net.clone()]);
                },
            }
            
        }
    }

    static mut DUPLICATOR_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static mut VOIDER_COUNTER: AtomicUsize = AtomicUsize::new(0);

    for (_, (src_port_name, src_port)) in src_port_mapping.iter().enumerate() {
        let project_packages = project.read().unwrap().get_packages();
        let std_package = project_packages.get(STD_LIB_PACKAGE_NAME);
        if std_package.is_none() {
            return Err(TydiLangError::new(format!("std package not found: {}", &STD_LIB_PACKAGE_NAME), CodeLocation::new_unknown()));
        }
        let std_package = std_package.unwrap();
        let std_package_scope = std_package.read().unwrap().get_scope();

        let sink_ports = src_to_sink_port_mapping.get(src_port_name);
        let sink_ports = match sink_ports {
            Some(s) => s.clone(),
            None => vec![],
        };

        // append voider
        if sink_ports.len() == 0 {
            let voider_counter;
            unsafe {
                VOIDER_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                voider_counter = VOIDER_COUNTER.load(std::sync::atomic::Ordering::SeqCst);
            }
            let src_port_owner = src_port_owner_mapping.get(src_port_name).expect("bug: src_port_name not found");
            // add voider instance
            let new_voider;
            {
                let voider_name = format!("voider_{}", voider_counter);
                // resolve voider
                let mut arg_map = BTreeMap::new();
                let src_port_type_var = src_port.read().unwrap().get_logical_type();
                let src_port_type = src_port_type_var.read().unwrap().get_value();
                arg_map.insert(0, src_port_type);
                let (new_voider_var, target_var_scope) = Scope::resolve_identifier(&String::from(STD_VOID_IMPL_NAME), &Some(arg_map), &CodeLocation::new_unknown(), std_package_scope.clone(), std_package_scope.clone(), ScopeRelationType::resolve_id_default(), evaluator.clone())?;
                evaluate_var(new_voider_var.clone(), target_var_scope.clone(), evaluator.clone())?;
                let target_var_value = new_voider_var.read().unwrap().get_value();
                let new_voider_implementation = match target_var_value {
                    crate::tydi_memory_representation::TypedValue::Implementation(i) => i,
                    _ => unreachable!(),
                };

                new_voider = Instance::new_with_derived_implementation(voider_name.clone(), new_voider_implementation.clone());
                target_implementation.read().unwrap().add_instance(target_implementation.clone(), new_voider.clone())?;
            }
            // add net to voider
            {
                let all_voider_ports = new_voider.read().unwrap().get_all_ports();
                let sink_port = all_voider_ports.get(0).expect("bug: voider without ports");
                let new_net_to_voider = Net::new_with_known_src_sink(src_port.clone(), src_port_owner.clone(), sink_port.clone(), PortOwner::ImplInstance(new_voider.clone()));
                target_implementation.read().unwrap().add_net(target_implementation.clone(), new_net_to_voider)?;
            }
        }

        // append duplicator
        if sink_ports.len() >= 2 {
            let duplicator_counter;
            unsafe {
                DUPLICATOR_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                duplicator_counter = DUPLICATOR_COUNTER.load(std::sync::atomic::Ordering::SeqCst);
            }

            let src_port_owner = src_port_owner_mapping.get(src_port_name).expect("bug: src_port_name not found");

            // add duplicator instance
            let new_duplicator;
            {
                let duplicator_name = format!("duplicator_{}", duplicator_counter);
                // resolve duplicator
                let mut arg_map = BTreeMap::new();
                let src_port_type_var = src_port.read().unwrap().get_logical_type();
                let src_port_type = src_port_type_var.read().unwrap().get_value();
                arg_map.insert(0, src_port_type);
                arg_map.insert(1, TypedValue::IntValue(sink_ports.len() as i128));

                let (new_duplicator_var, target_var_scope) = Scope::resolve_identifier(&String::from(STD_DUPLICATOR_IMPL_NAME), &Some(arg_map), &CodeLocation::new_unknown(), std_package_scope.clone(), std_package_scope.clone(), ScopeRelationType::resolve_id_default(), evaluator.clone())?;
                evaluate_var(new_duplicator_var.clone(), target_var_scope.clone(), evaluator.clone())?;
                let target_var_value = new_duplicator_var.read().unwrap().get_value();
                let new_duplicator_implementation = match target_var_value {
                    crate::tydi_memory_representation::TypedValue::Implementation(i) => i,
                    _ => unreachable!(),
                };

                new_duplicator = Instance::new_with_derived_implementation(duplicator_name.clone(), new_duplicator_implementation.clone());
                target_implementation.read().unwrap().add_instance(target_implementation.clone(), new_duplicator.clone())?;
            }

            // remove existing nets
            {
                let implementation_scope = target_implementation.read().unwrap().get_scope();
                let mut all_vars_in_implementation_scope = implementation_scope.read().unwrap().get_variables();
                let old_nets = src_to_sink_port_nets_mapping.get(src_port_name).expect("bug: src_port_name not found");
                for single_old_net in old_nets {
                    let single_old_net_name = single_old_net.read().unwrap().get_name();
                    all_vars_in_implementation_scope.remove(&single_old_net_name).expect("bug: single old net name not found");
                }
                implementation_scope.write().unwrap().set_variables(all_vars_in_implementation_scope);
            }

            // add new nets to duplicator
            {
                // net between src port and duplicator
                {
                    let mut duplicator_sink_port = None;
                    {
                        let all_duplicator_ports = new_duplicator.read().unwrap().get_all_ports();
                        for p in all_duplicator_ports {
                            if p.read().unwrap().get_direction() == PortDirection::In {
                                duplicator_sink_port = Some(p.clone());
                                break;
                            }
                        }
                    }
                    let duplicator_sink_port = match duplicator_sink_port {
                        Some(i) => i,
                        None => unreachable!("bug: no sink port for duplicator"),
                    };

                    let new_net_to_duplicator = Net::new_with_known_src_sink(src_port.clone(), src_port_owner.clone(), duplicator_sink_port.clone(), PortOwner::ImplInstance(new_duplicator.clone()));
                    target_implementation.read().unwrap().add_net(target_implementation.clone(), new_net_to_duplicator)?;
                }

                // nets between duplicator and sink ports
                {
                    let mut duplicator_src_ports = vec![];
                    {
                        let all_duplicator_ports = new_duplicator.read().unwrap().get_all_ports();
                        for p in all_duplicator_ports {
                            if p.read().unwrap().get_direction() == PortDirection::Out {
                                duplicator_src_ports.push(p.clone());
                            }
                        }
                    }
                    assert!(duplicator_src_ports.len() == sink_ports.len());
                    for i in 0..duplicator_src_ports.len() {
                        let sink_port = sink_ports.get(i).expect("bug: invalid index");
                        let sink_port_owner = src_to_sink_port_owner_mapping.get(src_port_name).expect("bug: src_port_name not found").get(i).expect("bug: invalid index");
                        let duplicator_src_port = duplicator_src_ports.get(i).expect("bug: invalid index");

                        let new_net_to_duplicator = Net::new_with_known_src_sink(duplicator_src_port.clone(), PortOwner::ImplInstance(new_duplicator.clone()), sink_port.clone(), sink_port_owner.clone());
                        target_implementation.read().unwrap().add_net(target_implementation.clone(), new_net_to_duplicator)?;
                    }
                }
            }
        }
    }

    return Ok(());
}