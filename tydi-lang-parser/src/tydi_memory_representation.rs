pub mod code_location;
pub(in crate) use code_location::*;
pub use code_location::CodeLocation;

pub mod package;
pub(in crate) use package::*;
pub use package::Package;

pub mod scope;
pub(in crate) use scope::*;
pub use scope::{Scope, ScopeType, ScopeRelationType, ScopeRelationship};

pub mod var_type;
pub(in crate) use var_type::*;
pub use var_type::{TypedValue, TypeIndication};

pub mod var;
pub(in crate) use var::*;
pub use var::Variable;

pub mod logic_type;
pub(in crate) use logic_type::*;
pub use logic_type::{LogicType, logic_bit::LogicBit, logic_group::LogicGroup, logic_union::LogicUnion, logic_stream::LogicStream};

pub mod template_args;
pub(in crate) use template_args::*;
pub use template_args::{TemplateArg};

pub mod project;
pub(in crate) use project::*;
pub use project::Project;

pub mod streamlet;
pub(in crate) use streamlet::*;
pub use streamlet::Streamlet;

pub mod attributes;
pub(in crate) use attributes::*;
pub use attributes::Attribute;

pub mod port;
pub(in crate) use port::*;
pub use port::{Port, PortDirection};