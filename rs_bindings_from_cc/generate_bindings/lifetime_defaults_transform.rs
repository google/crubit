// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![deny(missing_docs)]

//! Adds default lifetimes to (partially-) unannotated IR.

use ir::{
    make_ir, CcType, CcTypeVariant, FlatIR, Func, Item, LifetimeId, LifetimeName, PointerType,
    PointerTypeKind, IR,
};
use std::cmp::max;
use std::rc::Rc;

struct LifetimeDefaults {
    /// Used to generate unique lifetime ids. This field is always set to one more than the highest
    /// lifetime id in the IR.
    lifetime_gensym: i32,
    /// The name of the static lifetime (if used).
    static_lifetime: Option<LifetimeName>,
}

// TODO: b/454627672 - It appears that `int& f(int& $a i1)` drops `$a` entirely (before we even get
// the IR to transform). Same with `int& $a f(int& i1)`, and `int& $a f(int& $a i1, int& i2). Is
// something trying to be more helpful than we expect earlier on?

/// Used to keep track of the state we're in when ascribing lifetimes.
#[derive(Clone, PartialEq)]
enum LifetimeState {
    /// No lifetimes have been seen.
    Unseen,
    /// A single lifetime has been seen.
    Single(LifetimeId),
    /// The `static` lifetime should be used.
    Static,
    /// We can't ascribe a lifetime.
    Unknown,
}

impl LifetimeState {
    fn update(&mut self, next: &LifetimeState) {
        match (&self, next) {
            (LifetimeState::Unseen, n) => *self = n.clone(),
            (_, LifetimeState::Unseen) => (),
            (LifetimeState::Single(a), LifetimeState::Single(b)) if a == b => (),
            (LifetimeState::Static, LifetimeState::Static) => (),
            _ => *self = LifetimeState::Unknown,
        }
    }
}

/// The result of adding lifetimes to a type.
struct LifetimeResult {
    /// The rewritten type.
    ty: CcType,
    /// Output state for default lifetime assignment.
    state: LifetimeState,
    /// New bindings that were made.
    bindings: Vec<LifetimeName>,
}

impl LifetimeDefaults {
    fn new(ir: &IR) -> Self {
        let mut static_lifetime = None;
        let mut lifetime_gensym = 0;
        // nb: this is wrong in make_ir if other items can bind lifetime names.
        for (id, name) in ir.lifetimes() {
            lifetime_gensym = max(lifetime_gensym, id.0 + 1);
            if *(name.name) == *"static" {
                static_lifetime = Some(name)
            }
        }
        LifetimeDefaults { lifetime_gensym, static_lifetime: static_lifetime.cloned() }
    }

    /// Returns or creates the global static lifetime.
    fn static_lifetime(&mut self) -> LifetimeName {
        if let Some(static_lifetime) = &self.static_lifetime {
            return static_lifetime.clone();
        }
        let new_lifetime = self.make_lifetime(Some(Rc::from("static")));
        self.static_lifetime = Some(new_lifetime.clone());
        new_lifetime
    }

    /// Returns a state representing the given `lifetime`.
    fn get_state_for_annotated_lifetime(&mut self, lifetime: Option<&LifetimeId>) -> LifetimeState {
        match lifetime {
            None => LifetimeState::Unseen,
            Some(id) => {
                if let Some(static_lifetime) = &self.static_lifetime {
                    if *id == static_lifetime.id {
                        return LifetimeState::Static;
                    }
                }
                LifetimeState::Single(*id)
            }
        }
    }

    /// Returns a lifetime (suitable for use as an annotation) from `state` if `state`
    /// unambiguously refers to a lifetime.
    fn get_lifetime_for_state(&mut self, state: &LifetimeState) -> Option<LifetimeId> {
        match state {
            LifetimeState::Single(lifetime) => Some(*lifetime),
            LifetimeState::Static => Some(self.static_lifetime().id),
            _ => None,
        }
    }

    /// Generates a new lifetime variable using `name` to name it. The id is guaranteed to be
    /// fresh.
    fn make_lifetime(&mut self, name: Option<Rc<str>>) -> LifetimeName {
        let id = LifetimeId(self.lifetime_gensym);
        self.lifetime_gensym += 1;
        LifetimeName {
            name: name.unwrap_or_else(|| format!("lt_{0}", id.0).to_string().into()),
            id,
        }
    }

    /// Adds lifetimes to a type in input position. Returns the new type paired with a LifetimeState
    /// describing the lifetimes we encountered and a list of any lifetimes we had to bind.
    /// `name_hint` is used to name the lifetime parameter when we need to make one.
    fn add_lifetime_to_input_type(
        &mut self,
        name_hint: Option<Rc<str>>,
        ty: &CcType,
    ) -> LifetimeResult {
        match &ty.variant {
            CcTypeVariant::Pointer(pty) if pty.kind == PointerTypeKind::LValueRef => {
                let LifetimeResult { ty: pointee_type, state: _, mut bindings } =
                    self.add_lifetime_to_input_type(name_hint.clone(), &pty.pointee_type);
                let mut state = self.get_state_for_annotated_lifetime(pty.lifetime.as_ref());
                if state == LifetimeState::Unseen {
                    let lifetime = self.make_lifetime(name_hint.clone());
                    bindings.push(lifetime.clone());
                    state = LifetimeState::Single(lifetime.id);
                }
                let mut new_ty = ty.clone();
                new_ty.variant = CcTypeVariant::Pointer(PointerType {
                    lifetime: self.get_lifetime_for_state(&state),
                    pointee_type: pointee_type.into(),
                    ..pty.clone()
                });
                LifetimeResult { ty: new_ty, state, bindings }
            }
            _ => LifetimeResult { ty: ty.clone(), state: LifetimeState::Unseen, bindings: vec![] },
        }
    }

    /// Adds lifetimes to a type in output position. `lifetime_hint` is used to assign a lifetime
    /// when one is not otherwise available. If `lifetime_hint` is None, no new lifetimes will be
    /// assigned.
    fn add_lifetime_to_output_type(
        &mut self,
        lifetime_hint: Option<LifetimeId>,
        ty: &CcType,
    ) -> CcType {
        match &ty.variant {
            CcTypeVariant::Pointer(pty) if pty.kind == PointerTypeKind::LValueRef => {
                let mut new_ty = ty.clone();
                // Stop changing annotations if we see a previously-annotated lifetime.
                if pty.lifetime.is_some() || lifetime_hint.is_none() {
                    return new_ty;
                }
                let pointee_type =
                    self.add_lifetime_to_output_type(lifetime_hint, &pty.pointee_type);
                new_ty.variant = CcTypeVariant::Pointer(PointerType {
                    lifetime: lifetime_hint,
                    pointee_type: pointee_type.into(),
                    ..pty.clone()
                });
                new_ty
            }
            _ => ty.clone(),
        }
    }

    /// Transforms a function to use default lifetime rules. Every lifetime variable has a globally
    /// unique id, so we don't need to worry about avoiding captures, and the incoming IR should
    /// already have lifetime_params annotated where bindings need to be made.
    fn add_lifetime_to_func(&mut self, func: &Func) -> Func {
        let mut new_func = func.clone();
        let mut state = LifetimeState::Unseen;
        new_func.params = func
            .params
            .iter()
            .map(|param| {
                let mut new_param = param.clone();
                let LifetimeResult { ty: new_type, state: new_state, bindings: mut new_bindings } =
                    self.add_lifetime_to_input_type(
                        Some(new_param.identifier.identifier.clone()),
                        &new_param.type_,
                    );
                new_func.lifetime_params.append(&mut new_bindings);
                state.update(&new_state);
                new_param.type_ = new_type;
                new_param
            })
            .collect();
        let lifetime = self.get_lifetime_for_state(&state);
        new_func.return_type = self.add_lifetime_to_output_type(lifetime, &new_func.return_type);
        new_func
    }

    /// Since we keep all item ids stable, we only have to deep-clone the objects that we need to
    /// change. We may need to introduce lifetime param binders whenever we see a type (but not on
    /// decls).
    fn add_lifetime_to_item(&mut self, item: &Item) -> Item {
        match item {
            Item::Func(func) => Item::Func(self.add_lifetime_to_func(func).into()),
            _ => item.clone(),
        }
    }
}

/// Creates a copy of `ir` with default lifetimes filled in.
pub fn lifetime_defaults_transform(ir: &IR) -> IR {
    let mut ctx = LifetimeDefaults::new(ir);
    let new_items = ir.items().map(|item| ctx.add_lifetime_to_item(item)).collect();
    make_ir(FlatIR {
        public_headers: ir.flat_ir().public_headers.clone(),
        current_target: ir.flat_ir().current_target.clone(),
        items: new_items,
        top_level_item_ids: ir.flat_ir().top_level_item_ids.clone(),
        crate_root_path: ir.flat_ir().crate_root_path.clone(),
        crubit_features: ir.flat_ir().crubit_features.clone(),
    })
}
