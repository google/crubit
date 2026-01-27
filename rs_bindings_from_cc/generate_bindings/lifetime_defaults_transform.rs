// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![deny(missing_docs)]

//! Adds default lifetimes to (partially-) unannotated IR.

use database::BindingsGenerator;
use ir::{
    make_ir, CcType, CcTypeVariant, FlatIR, Func, Item, LifetimeId, LifetimeName, PointerType,
    PointerTypeKind, IR,
};
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// A binding for one (possibly renamed) variable.
pub struct VariableBinding {
    /// A stack of renamings for this variable.
    pub renamed: Vec<Rc<str>>,
}

/// Keeps track of bound variables.
pub struct BindingContext {
    /// Maps user variable names to bindings.
    pub bindings: HashMap<Rc<str>, VariableBinding>,
    /// All variable names we've seen.
    pub names: HashSet<Rc<str>>,
}

/// Manages bindings for lifetime names. We expect to start with a `new` `BindingContext` for each
/// item I being imported. This `BindingContext` should contain bindings for all parent items of I.
/// The `BindingContext` will rename bindings that are shadowed and will never use the same renaming
/// twice.
impl BindingContext {
    /// Binds the variable `id`, returning a unique name for it.
    pub fn push_new_binding(&mut self, id: &Rc<str>) -> Rc<str> {
        let mut binding =
            self.bindings.remove(id).unwrap_or_else(|| VariableBinding { renamed: vec![] });
        let fresh_name = self.fresh_name_for(id);
        binding.renamed.push(fresh_name.clone());
        self.bindings.insert(id.clone(), binding);
        self.names.insert(fresh_name.clone());
        fresh_name
    }

    /// Returns a fresh variable name.
    pub fn push_fresh_binding(&mut self, hint: Option<&Rc<str>>) -> Rc<str> {
        let fresh_name = match hint {
            Some(id) => self.fresh_name_for(id),
            None => self.fresh_name_for(&Rc::from("lt")),
        };
        self.push_new_binding(&fresh_name)
    }

    /// If `id` is already bound, returns its renaming; otherwise, creates a new binding for it,
    /// and calls the provided function with the renaming.
    pub fn get_or_push_new_binding<F: FnOnce(&Rc<str>)>(&mut self, id: &Rc<str>, f: F) -> Rc<str> {
        if let Some(binding) = self.bindings.get(id) {
            return binding.renamed.last().unwrap().clone();
        }
        let new_binding = self.push_new_binding(id);
        f(&new_binding);
        new_binding
    }

    /// Returns a fresh variable name based on `id`.
    fn fresh_name_for(&self, id: &Rc<str>) -> Rc<str> {
        if !self.names.contains(id) {
            return id.clone();
        }
        let mut ix = 0;
        loop {
            let name = format!("{}_{}", id, ix);
            if !self.names.contains(name.as_str()) {
                return Rc::from(name);
            }
            ix += 1;
        }
    }

    /// Removes the last binding for `id`.
    pub fn pop_binding(&mut self, id: &Rc<str>) {
        let mut last_binding = self.bindings.remove(id).unwrap();
        last_binding.renamed.pop();
        if !last_binding.renamed.is_empty() {
            self.bindings.insert(id.clone(), last_binding);
        }
    }

    /// Creates a binding context with builtin lifetimes configured.
    pub fn new() -> Self {
        let mut ctx = BindingContext { bindings: HashMap::new(), names: HashSet::new() };
        ctx.push_new_binding(&Rc::from("static"));
        ctx
    }
}

impl Default for BindingContext {
    fn default() -> Self {
        BindingContext::new()
    }
}

struct LifetimeDefaults {
    bindings: BindingContext,
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
    Single(Rc<str>),
    /// We can't ascribe a lifetime.
    Unknown,
}

impl LifetimeState {
    fn update(&mut self, next: &LifetimeState) {
        match (&self, next) {
            (LifetimeState::Unseen, n) => *self = n.clone(),
            (_, LifetimeState::Unseen) => (),
            (LifetimeState::Single(a), LifetimeState::Single(b)) if a == b => (),
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
}

impl LifetimeDefaults {
    fn new(ir: &IR) -> Self {
        LifetimeDefaults { bindings: BindingContext::new() }
    }

    /// Returns a state representing the given `lifetime`.
    fn get_state_for_annotated_lifetime(
        &mut self,
        lifetime: &[Rc<str>],
        new_bindings: &mut Vec<Rc<str>>,
    ) -> LifetimeState {
        match lifetime {
            [] => LifetimeState::Unseen,
            [id] => LifetimeState::Single(
                self.bindings.get_or_push_new_binding(id, |name| new_bindings.push(name.clone())),
            ),
            // TODO(b/454627672): multiple variables.
            _ => LifetimeState::Unknown,
        }
    }

    /// Returns a lifetime (suitable for use as an annotation) from `state` if `state`
    /// unambiguously refers to a lifetime.
    fn get_lifetime_for_state(&mut self, state: &LifetimeState) -> Vec<Rc<str>> {
        match state {
            LifetimeState::Single(lifetime) => vec![lifetime.clone()],
            _ => vec![],
        }
    }

    /// Adds lifetimes to a type in input position. Returns the new type paired with a LifetimeState
    /// describing the lifetimes we encountered and a list of any lifetimes we had to bind.
    /// `name_hint` is used to name the lifetime parameter when we need to make one.
    fn add_lifetime_to_input_type(
        &mut self,
        name_hint: Option<&Rc<str>>,
        new_bindings: &mut Vec<Rc<str>>,
        ty: &CcType,
    ) -> LifetimeResult {
        match &ty.variant {
            CcTypeVariant::Pointer(pty) if pty.kind == PointerTypeKind::LValueRef => {
                let LifetimeResult { ty: pointee_type, state: _ } =
                    self.add_lifetime_to_input_type(name_hint, new_bindings, &pty.pointee_type);
                let mut state =
                    self.get_state_for_annotated_lifetime(&ty.explicit_lifetimes, new_bindings);
                if state == LifetimeState::Unseen {
                    let lifetime = self.bindings.push_fresh_binding(name_hint);
                    new_bindings.push(lifetime.clone());
                    state = LifetimeState::Single(lifetime);
                }
                let mut new_ty = ty.clone();
                new_ty.variant = CcTypeVariant::Pointer(PointerType {
                    pointee_type: pointee_type.into(),
                    ..pty.clone()
                });
                new_ty.explicit_lifetimes = self.get_lifetime_for_state(&state);
                LifetimeResult { ty: new_ty, state }
            }
            _ => LifetimeResult { ty: ty.clone(), state: LifetimeState::Unseen },
        }
    }

    /// Adds lifetimes to a type in output position. `lifetime_hint` is used to assign a lifetime
    /// when one is not otherwise available. If `lifetime_hint` is empty, no new lifetimes will be
    /// assigned.
    fn add_lifetime_to_output_type(
        &mut self,
        lifetime_hint: &Vec<Rc<str>>,
        new_bindings: &mut Vec<Rc<str>>,
        ty: &CcType,
    ) -> CcType {
        match &ty.variant {
            CcTypeVariant::Pointer(pty) if pty.kind == PointerTypeKind::LValueRef => {
                let mut new_ty = ty.clone();
                // If there's a previously-annotated lifetime, use that.
                if !ty.explicit_lifetimes.is_empty() {
                    new_ty.explicit_lifetimes = ty
                        .explicit_lifetimes
                        .iter()
                        .map(|l| {
                            self.bindings
                                .get_or_push_new_binding(l, |name| new_bindings.push(name.clone()))
                        })
                        .collect();
                    return new_ty;
                }
                // If there is no viable inferred lifetime, there is nothing to do.
                if lifetime_hint.is_empty() {
                    return new_ty;
                }
                let pointee_type = self.add_lifetime_to_output_type(
                    lifetime_hint,
                    new_bindings,
                    &pty.pointee_type,
                );
                new_ty.variant = CcTypeVariant::Pointer(PointerType {
                    pointee_type: pointee_type.into(),
                    ..pty.clone()
                });
                new_ty.explicit_lifetimes = lifetime_hint.clone();
                new_ty
            }
            _ => ty.clone(),
        }
    }

    /// Transforms a function to use default lifetime rules.
    fn add_lifetime_to_func(&mut self, func: &Func) -> Func {
        let mut new_func = func.clone();
        let mut state = LifetimeState::Unseen;
        let mut ctx = BindingContext::new();
        new_func.lifetime_inputs.clear();
        // TODO(b/454627672): add bindings for parent item lifetimes.
        func.lifetime_inputs
            .iter()
            .for_each(|name| new_func.lifetime_inputs.push(ctx.push_new_binding(name)));
        new_func.params = func
            .params
            .iter()
            .map(|param| {
                let mut new_param = param.clone();
                let LifetimeResult { ty: new_type, state: new_state } = self
                    .add_lifetime_to_input_type(
                        Some(&new_param.identifier.identifier),
                        &mut new_func.lifetime_inputs,
                        &new_param.type_,
                    );
                state.update(&new_state);
                new_param.type_ = new_type;
                new_param
            })
            .collect();
        let lifetime = self.get_lifetime_for_state(&state);
        new_func.return_type = self.add_lifetime_to_output_type(
            &lifetime,
            &mut new_func.lifetime_inputs,
            &new_func.return_type,
        );
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

/// Creates a copy of `func` with default lifetimes filled in.
pub fn lifetime_defaults_transform_func(db: &dyn BindingsGenerator, func: &Func) -> Func {
    LifetimeDefaults::new(db.ir()).add_lifetime_to_func(func)
}

/// Creates a copy of `ir` with default lifetimes filled in. This is mostly useful for testing;
/// prefer to transform items on demand.
pub fn lifetime_defaults_transform(ir: &IR) -> IR {
    let new_items =
        ir.items().map(|item| LifetimeDefaults::new(ir).add_lifetime_to_item(item)).collect();
    make_ir(FlatIR {
        public_headers: ir.flat_ir().public_headers.clone(),
        current_target: ir.flat_ir().current_target.clone(),
        items: new_items,
        top_level_item_ids: ir.flat_ir().top_level_item_ids.clone(),
        crate_root_path: ir.flat_ir().crate_root_path.clone(),
        crubit_features: ir.flat_ir().crubit_features.clone(),
    })
}
