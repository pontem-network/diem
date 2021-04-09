pub extern crate bytecode_verifier;
pub extern crate bytecode_source_map;
pub extern crate ir_to_bytecode_syntax;
pub extern crate diem_types;
pub extern crate diem_workspace_hack;
pub extern crate move_core_types;
pub extern crate move_ir_types;
pub extern crate vm as move_vm;
pub extern crate move_coverage;

pub extern crate bcs;
pub extern crate move_lang;
pub extern crate move_vm_types;
pub extern crate move_vm_runtime;
pub extern crate move_vm_natives;

pub extern crate diem_vm;
pub extern crate diem_state_view;
pub extern crate diem_logger;

pub extern crate compiler as move_compiler;
pub use move_compiler as diem_compiler;

pub mod prelude {
    pub use super::account::*;
    pub use super::result::*;
    pub use super::ds::*;
    pub use super::module::*;
    pub use super::bcs;
}

pub mod bf {
    pub use bytecode_verifier::control_flow_graph::{VMControlFlowGraph, ControlFlowGraph, BlockId};
}

pub mod module {
    pub use move_core_types::language_storage::ModuleId;
    pub use diem_types::transaction::Module;
    pub use vm::access::{ModuleAccess, ScriptAccess};
    pub use vm::file_format::{
        Bytecode, CompiledScript, CompiledModule, ModuleHandle, SignatureToken,
    };
    pub use move_lang::compiled_unit::CompiledUnit;
    pub use move_lang::parser::ast::{Definition, ModuleDefinition, Script};
    pub use move_core_types::value::MoveValue;
}

pub mod account {
    pub use diem_types::account_address::AccountAddress;
    pub use diem_types::account_config::CORE_CODE_ADDRESS;
    pub use move_core_types::identifier::Identifier;
}

pub mod result {
    pub use move_core_types::vm_status::{
        StatusCode, VMStatus, DiscardedVMStatus, KeptVMStatus, AbortLocation as AbortLoc,
    };
    pub use vm::errors::{Location, VMResult, PartialVMResult, PartialVMError, VMError};
}

pub mod ds {
    pub use diem_types::access_path::AccessPath;
    pub use move_vm_runtime::data_cache::RemoteCache;
    pub use diem_types::write_set::{WriteOp, WriteSet, WriteSetMut};
    pub use move_vm_runtime::loader::ModuleCache;
    pub use move_vm_runtime::data_cache::TransactionDataCache;
    pub use move_vm_runtime::loader::ScriptCache;
    pub use move_vm_runtime::loader::TypeCache;
    pub use move_vm_types::data_store::DataStore;
    pub use diem_vm::data_cache::RemoteStorage;
    pub use move_core_types::language_storage::{TypeTag, ResourceKey};
}

pub mod compiler {
    pub use move_lang::{compiled_unit, errors, parse_program};
    pub use move_lang::parser::ast::*;
    pub use move_lang::shared::Address;
    pub use move_lang::errors::{FilesSourceText, Errors, output_errors};
    pub use move_lang::name_pool::ConstPool;
    pub use move_lang::move_check;
}

pub mod file_format {
    pub use vm::file_format::*;
    pub use vm::file_format_common::*;
    pub use vm::access::ModuleAccess;
}

pub mod vm {
    pub use diem_types::contract_event::ContractEvent;
    pub use diem_types::transaction::TransactionStatus;
    pub use move_vm_runtime::move_vm::MoveVM;
    pub use move_core_types::language_storage::StructTag;
    pub use move_vm_types::values::Value;
    pub use move_vm_runtime::loader::Loader;
    pub use move_vm_runtime::session::Session;
}

pub mod gas {
    pub use move_core_types::gas_schedule::*;
    pub use move_vm_types::gas_schedule::*;
}

pub mod logger {
    pub use diem_logger::*;
}
