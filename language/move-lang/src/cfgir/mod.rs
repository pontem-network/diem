// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

mod absint;
pub mod ast;
mod borrows;
pub(crate) mod cfg;
mod constant_fold;
mod eliminate_locals;
mod inline_blocks;
mod liveness;
mod locals;
mod remove_no_ops;
mod simplify_jumps;
pub mod translate;

use crate::{
    errors::Errors,
    expansion::ast::AbilitySet,
    hlir::ast::*,
    parser::ast::{ModuleIdent, StructName, Var},
    shared::unique_map::UniqueMap,
};
use cfg::*;
use move_ir_types::location::*;
use std::collections::{BTreeMap, BTreeSet};

pub fn refine_inference_and_verify(
    errors: &mut Errors,
    struct_declared_abilities: &UniqueMap<ModuleIdent, UniqueMap<StructName, AbilitySet>>,
    signature: &FunctionSignature,
    acquires: &BTreeMap<StructName, Loc>,
    locals: &UniqueMap<Var, SingleType>,
    cfg: &mut BlockCFG,
    infinite_loop_starts: &BTreeSet<Label>,
) {
    liveness::last_usage(errors, locals, cfg, infinite_loop_starts);
    let locals_states = locals::verify(
        errors,
        struct_declared_abilities,
        signature,
        acquires,
        locals,
        cfg,
    );

    liveness::release_dead_refs(&locals_states, locals, cfg, infinite_loop_starts);
    borrows::verify(errors, signature, acquires, locals, cfg);
}

pub fn optimize(
    signature: &FunctionSignature,
    _locals: &UniqueMap<Var, SingleType>,
    cfg: &mut BlockCFG,
) {
    loop {
        let mut changed = false;
        changed |= eliminate_locals::optimize(signature, cfg);
        changed |= constant_fold::optimize(cfg);
        changed |= simplify_jumps::optimize(cfg);
        changed |= inline_blocks::optimize(cfg);

        if !changed {
            break;
        }
    }
}
