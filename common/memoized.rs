// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Memoization of functions, in the style of Salsa.
//!
//! See the documentation for `query_group!` for more details.
//!
//! # Why not Salsa?
//!
//! Salsa is not compatible with rustc: types like `Ty<'tcx>`, which are not
//! `'static`, cannot be used as function parameters or return values in Salsa.
//! See https://github.com/salsa-rs/salsa/issues/424.
//!
//! Since Salsa is off the table, this module offers a very simplified
//! alternative, which implements _only_ memoization, as that is all we need.
//!
//! # Differences with Salsa
//!
//! * Supports non-`'static` types.
//! * Syntactic differences when initially setting up the trait and database.
//! * Immutable input and no support for recomputation given mutated inputs.
//! * Correspondingly, no requirement that the *return* types implement `Eq` or
//!   `Hash`.
//! * Supports `#[break_cycles_with = <default-value>]`, which generates a
//!   function that returns <default-value> if a cycle is detected.
//! * Uses function pointers in the interior of a concrete type, instead of
//!   `dyn Trait`.
//!
//! There are more substantial differences with Salsa 2022 - this was written
//! based on Salsa 0.16. We don't need to match exactly the API, but the
//! differences are kept relatively small so as to support eventually either
//! going back to Salsa, or evolving towards something closer to what
//! Salsa implements.

/// `query_group!` defines a collection of memoized functions, and the shared
/// inputs that all of those functions can access.
///
/// These functions are defined as a trait, as well as a concrete type that
/// stores the inputs and memoized values, and implements that trait.
///
/// The structure of an invocation is always as follows:
///
/// ```
/// query_group! {
///   // QueryGroup name is the name of the memoized query object, including
///   // all relevant state.
///   struct QueryGroup {
///     // First, all shared inputs are specified, in order.
///     //
///     // These are available to all of the memoized functions, by calling `db.some_input()`, etc.,
///     // in order to access the input value. In other words, they form a sort of immutable global
///     // state, useful to pass immutable configuration values to _all_ of the memoized functions.
///     //
///     // The inputs are immutable, and must be `Clone`. Because they are immutable and global,
///     // they do not need to be `Eq` or `Hash`, and they are not compared when memoizing
///     // functions.
///     //
///     // As a rule of thumb, anything which is a constant for the lifetime of an execution, but
///     // is not an actual compile time constant, should be an input.
///
///     #[input]
///     /// Doc comment goes after `#[input]`.
///     fn some_input(&self) -> InputType;
///     //...
///
///     // After all of the inputs, the actual memoized functions are specified.
///     //
///     // Memoized functions are pure functions, which take any number of (`Clone+Eq+Hash`)
///     // parameters, and return a `Clone` return type.
///     //
///     // Each of these must be implemented in the _same module_ as a top-level function, with
///     // the same function signature.
///     //
///     // The functions can call methods on the `&QueryGroup` self argument to access both
///     // the shared inputs, as well to call other memoized functions.
///     //
///     // Inputs to the computation which change from call to call should be specified as function
///     // parameters. Inputs which never change can be either `#[input]` functions, or actual
///     // program globals.
///     //
///     // When called on the `&QueryGroup`, the functions will be memoized, and the return
///     // value will be cached, automatically.
///     //
///     // Some functions may need to gracefully handle cycles, in which case they should be
///     // annotated with `#[break_cycles_with = <default_value>]`. This will generate a function
///     // that returns <default_value> if a cycle is detected, but will _not_ cache the result.
///     // All `#[break_cycles_with = ..]` functions must appear before all
///     // non-`#[break_cycles_with = ..]` functions.
///
///     #[break_cycles_with = ReturnType::default()]
///     /// Doc comment goes after `#[break_cycles_with = ..]`.
///     fn may_be_cyclic(&self, arg: ArgType) -> ReturnType;
///
///     /// Doc comment goes here.
///     fn some_function(&self, arg: ArgType) -> ReturnType;
///   }
/// }
///
/// // The non-memoized implementation of the memoized functions
/// fn may_by_cyclic(db: &QueryGroup, arg: ArgType) -> ReturnType {
///   // ...
/// }
///
/// fn some_function(db: &QueryGroup, arg: ArgType) -> ReturnType {
///   // ...
/// }
/// ```
///
/// A new instance of `QueryGroup` can be created by using `QueryGroup::new(input,
/// values, here)`. It will implement the trait defined above: the `#[input]`
/// functions will return the corresponding values passed in to `QueryGroup::new`,
/// and the memoized functions will call the corresponding top-level functions.
///
/// For example, above, one could run `let db =
/// QueryGroup::new(InputType::default())`.
///
/// Now, if you call `db.some_function(...)`, it will either return a cached
/// value (if one is present), or else execute `some_function(&db)`, and cache
/// and return the result. A direct call to `some_function` (instead of
/// `db.some_function`) is not directly memoized.
///
/// Because the results are saved, it's very important that the functions are
/// pure and have no side-effects. In particular, internal mutability on the
/// inputs and arguments is best avoided.
///
/// Important notes:
///
/// * In the trait definition, all `#[input]` functions must be declared before
///   all (non-`#[input]`) memoized functions. The order of the input functions
///   is the order of their parameters in `new()`.
/// * Every (non-`#[input]`) trait method _must_ have a matching function passed
///   to `new()`.
/// * Since all trait methods are memoized, their arguments must be `Clone`,
///   `Eq`, and `Hash`.
///
/// # Non-`'static` types
///
/// The trait and struct can accept a lifetime parameter, but the syntax is a
/// little trickier:
///
/// ```
/// query_group! {
///   struct QueryGroup<'a> {
///     #[input]
///     fn some_input(&self) -> &'a InputType;
///     fn some_function(&self, arg: &'a ArgType) -> &'a ReturnType;
///   }
/// }
///
/// fn<'a> some_function(db: &QueryGroup<'a>, arg: &'a ArgType) -> &'a ReturnType {
///   // ...
/// }
/// ```
///
/// Note that under the hood, `QueryGroup` will bedefined as something
/// like:
///
/// ```
/// struct QueryGroup<'a> {...}
/// ```
///
/// And so you may need to specify the lifetime in some uses.
#[macro_export]
macro_rules! query_group {
  (
    $vis:vis struct $database_struct:ident $(<$($type_param:tt),*>)?{
      $(
        // TODO(jeanpierreda): Ideally would allow putting the doc-comment first,
        // but this causes parsing ambiguity.
        #[input]
        $(#[doc = $input_doc:literal])*
        fn $input_function:ident(&self $(,)?) -> $input_type:ty;
      )*
      $(
        // TODO(jeanpierreda): Ideally would allow putting the doc-comment first,
        // but this causes parsing ambiguity.
        #[break_cycles_with = $break_cycles_default_value:expr]
        $(#[doc = $break_cycles_doc:literal])*
        fn $break_cycles_function:ident(
          &self
          $(
            , $break_cycles_arg:ident : $break_cycles_arg_type:ty
          )*
          $(,)?
        ) -> $break_cycles_return_type:ty;
      )*
      $(
        $(#[doc = $function_doc:literal])*
        fn $function:ident(
          &self
          $(
            , $arg:ident : $arg_type:ty
          )*
          $(,)?
        ) -> $return_type:ty;
      )*
      $(
        #[provided]
        $(#[doc = $provided_doc:literal])*
        fn $provided_function:ident(&$provided_self:ident $(, $provided_arg:ident : $provided_arg_type:ty)* $(,)?) -> $provided_type:ty { $($provided_body:tt)* }
      )*
    }
  ) => {
    // The database struct, which contains the lookup tables.
    $vis struct $database_struct $(<$($type_param),*>)? {
      pub __unwinding_cycles: ::core::cell::Cell<u32>,
      pub __db_index: u8,
      $(
        $input_function: $input_type,
      )*
      $(
        $break_cycles_function: $crate::internal::FnAndTable<
            fn(&Self, $($break_cycles_arg_type),*) -> $break_cycles_return_type,
            ($($break_cycles_arg_type,)*),
            $break_cycles_return_type
          >,
      )*
      $(
        $function: $crate::internal::FnAndTable<
            fn(&Self, $($arg_type),*) -> $return_type,
            ($($arg_type,)*),
            $return_type
          >,
      )*
    }

    // ...and the methods.
    impl $(<$($type_param),*>)? $database_struct $(<$($type_param),*>)? {
      $(
        $(#[doc = $input_doc])*
        $vis fn $input_function(&self) -> $input_type {
          // Have to be very careful to clone whatever the top level value is.
          // In particular, if it's a reference `&T`, clone the _reference_ to get another `&T`,
          // not the referent to get a `T`, like if we just cloned `self.$input_function`.
          (&self.$input_function).clone()
        }
      )*
      $(
        $(#[doc = $break_cycles_doc])*
        $vis fn $break_cycles_function(
          &self,
          $(
            $break_cycles_arg : $break_cycles_arg_type
          ),*
        ) -> $break_cycles_return_type {
          self.$break_cycles_function.table.internal_memoized_call(
            ($(
              $break_cycles_arg,
            )*),
            |($($break_cycles_arg,)*)| {
              (self.$break_cycles_function.fn_ptr)(self, $($break_cycles_arg),*)
            },
            &self.__unwinding_cycles,
          ).unwrap_or($break_cycles_default_value)
        }
      )*
      $(
        $(#[doc = $function_doc])*
        $vis fn $function(
          &self,
          $(
            $arg : $arg_type
          ),*
        ) -> $return_type {
          self.$function.table.internal_memoized_call(
            ($(
              $arg,
            )*),
            |($($arg,)*)| {
              (self.$function.fn_ptr)(self, $($arg),*)
            },
            &self.__unwinding_cycles,
          ).unwrap_or_else(
            || panic!("Cycle detected: '{}' depends on its own return value", stringify!($function)),
          )
        }
      )*
      $(
        $(#[doc = $provided_doc])*
        $vis fn $provided_function(
          &$provided_self,
          $(
            $provided_arg : $provided_arg_type
          ),*
        ) -> $provided_type {
          $( $provided_body )*
        }
      )*
    }
    // And the new() functions for initialization.
    impl $(<$($type_param),*>)? $database_struct $(<$($type_param),*>)? {
      $vis fn new(
          $($input_function: $input_type,)*
          $($break_cycles_function: fn(&Self, $($break_cycles_arg_type),*) -> $break_cycles_return_type,)*
          $($function: fn(&Self, $($arg_type),*) -> $return_type,)*
      ) -> Self {
        Self {
          __unwinding_cycles: ::core::cell::Cell::new(0),
          __db_index: $crate::internal::db_index(),
          $(
            $input_function,
          )*
          $(
            $break_cycles_function: $crate::internal::FnAndTable {
              fn_ptr: $break_cycles_function,
              table: Default::default(),
            },
          )*
          $(
            $function: $crate::internal::FnAndTable {
              fn_ptr: $function,
              table: Default::default(),
            },
          )*
        }
      }
    }
  }
}

#[macro_export]
macro_rules! memoized {
  (
    $vis:vis fn $memoized_ident:ident(
      db: $db_type:ty
      $(, $arg:ident : $arg_type:ty)*
      $(,)?
    ) -> $return_type:ty = $original_fn:ident
  ) => {
    mod $memoized_ident {
      std::thread_local! {
        /// A table off to the side -- one per db instance.
        ///
        /// This acts the same as if the db had a table field (or one per thread, anyway),
        /// but doesn't require centrally registering with the db.
        // TODO(jeanpierreda): make the `db` part of the key instead of holding it separately.
        // Probably this means writing a new MemoizationKey trait which subsumes Hash -- that is,
        // the `&db` gets compared using the __db_index. But this is very tricky to implement,
        // and might add a lot more code than it saves. The real benefit isn't simplicity,
        // but removing the double layer of RefCell / the fringe risk of crashes if memoization
        // tables cross-reference each other.
        pub static TABLES: ::std::cell::RefCell<::std::vec::Vec<$crate::internal::MemoizationTable<($($arg_type,)*), $return_type>>> = ::std::cell::RefCell::new(vec![Default::default()]);
      }
    }
    $vis fn $memoized_ident(db: $db_type, $($arg: $arg_type),*) -> $return_type {
      $memoized_ident::TABLES.with(|tables| {
        let db_index = db.__db_index as usize;
        if tables.borrow().len() <= db_index {
          tables.borrow_mut().resize_with(db_index + 1, Default::default);
        }
        let table = &tables.borrow()[db_index];
        table.internal_memoized_call(
          ($($arg,)*),
          |($($arg,)*)| {
            $original_fn(db, $($arg),*)
          },
          &db.__unwinding_cycles,
        ).unwrap_or_else(
          || panic!("Cycle detected: '{}' depends on its own return value", stringify!($memoized_ident)),
        )
      })
    }
  }
}

#[doc(hidden)]
pub mod internal {
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::hash::Hash;

    #[derive(Copy, Clone, PartialEq, Eq)]
    enum FoundCycle {
        No,
        Yes,
    }

    pub struct FnAndTable<F, Args, Return>
    where
        Args: Clone + Eq + Hash,
        Return: Clone,
    {
        pub fn_ptr: F,
        pub table: MemoizationTable<Args, Return>,
    }

    pub struct MemoizationTable<Args, Return>
    where
        Args: Clone + Eq + Hash,
        Return: Clone,
    {
        memoized: RefCell<HashMap<Args, Return>>,
        active: RefCell<HashMap<Args, FoundCycle>>,
    }

    // Separate `impl` instead of `#[derive(Default)]` because the `derive` would
    // needlessly require that `Args` and `Return` also implement `Default`.
    impl<Args, Return> Default for MemoizationTable<Args, Return>
    where
        Args: Clone + Eq + Hash,
        Return: Clone,
    {
        fn default() -> Self {
            Self { memoized: RefCell::new(HashMap::new()), active: RefCell::new(HashMap::new()) }
        }
    }

    impl<Args, Return> MemoizationTable<Args, Return>
    where
        Args: Clone + Eq + Hash,
        Return: Clone,
    {
        pub fn internal_memoized_call<F>(
            &self,
            args: Args,
            f: F,
            unwinding_cycles: &Cell<u32>,
        ) -> Option<Return>
        where
            F: FnOnce(Args) -> Return,
        {
            if let Some(return_value) = self.memoized.borrow().get(&args) {
                return Some(return_value.clone());
            }
            if let Some(found_cycle) = self.active.borrow_mut().get_mut(&args) {
                // We're in a cycle.
                if *found_cycle == FoundCycle::No {
                    // Only increase the count if we haven't hit this cycle before.
                    unwinding_cycles.set(unwinding_cycles.get() + 1);
                }
                *found_cycle = FoundCycle::Yes;
                return None;
            }
            self.active.borrow_mut().insert(args.clone(), FoundCycle::No);
            let return_value = f(args.clone());
            let found_cycle = self.active.borrow_mut().remove(&args).expect(
                "Internal error: currently-active cycle detection args not found. \
                      Most likely this is because of a buggy Clone/Eq/Hash impl.",
            );

            if found_cycle == FoundCycle::Yes {
                // We did hit outselves in a cycle but now we've broken out of it.
                // If we hit ourselves multiple times, we were careful to only increment this
                // count once.
                unwinding_cycles.set(unwinding_cycles.get() - 1);
            }
            if unwinding_cycles.get() == 0 {
                // No cycles, we can safely cache the result knowing that we haven't depended on
                // any cycle default values.
                self.memoized.borrow_mut().insert(args, return_value.clone());
            }
            Some(return_value)
        }
    }

    pub fn db_index() -> u8 {
        static DB_INDEX: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        DB_INDEX.fetch_add(1, std::sync::atomic::Ordering::AcqRel)
    }
}

#[cfg(test)]
pub mod tests {
    use googletest::prelude::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    #[gtest]
    fn test_basic_memoization() {
        crate::query_group! {
          pub struct Add10 {
            #[input]
            /// Tracker for how many times this function is called so we can check
            /// that memoization is indeed happening. This is just for testing;
            /// memoized functions in non-test code shouldn't have side effects,
            /// and inputs in non-test code shouldn't have internal mutability.
            fn call_counter(&self) -> Rc<Cell<i32>>;
            fn add10(&self, arg: i32) -> i32;
          }
        }
        fn add10(db: &Add10, arg: i32) -> i32 {
            db.call_counter().set(db.call_counter().get() + 1);
            arg + 10
        }
        let db = Add10::new(Rc::new(Cell::new(0)), add10);

        assert_eq!(db.add10(100), 110);
        assert_eq!(db.call_counter().get(), 1);

        assert_eq!(db.add10(100), 110);
        assert_eq!(db.call_counter().get(), 1);

        assert_eq!(db.add10(200), 210);
        assert_eq!(db.call_counter().get(), 2);
    }

    /// The raison d'etre of this module: memoization with an attached lifetime.
    ///
    /// This test is similar to test_basic_memoization, except that it accepts
    /// and returns references.
    #[gtest]
    fn test_nonstatic_memoization() {
        crate::query_group! {
          pub struct Add10<'a> {
            #[input]
            fn call_counter(&self) -> &'a Cell<i32>;
            fn add10(&self, arg: &'a i32) -> i32;
            // non-input function with 'a in return type
            fn identity(&self, arg: &'a i32) -> &'a i32;
          }
        }
        fn add10<'a>(db: &Add10<'a>, arg: &'a i32) -> i32 {
            db.call_counter().set(db.call_counter().get() + 1);
            *arg + 10
        }
        fn identity<'a>(db: &Add10<'a>, arg: &'a i32) -> &'a i32 {
            db.call_counter().set(db.call_counter().get() + 1);
            arg
        }
        let count = Cell::new(0);
        let db = Add10::new(&count, add10, identity);

        assert_eq!(db.add10(&100), 110);
        assert_eq!(count.get(), 1);

        assert_eq!(db.add10(&100), 110);
        assert_eq!(count.get(), 1);

        assert_eq!(db.add10(&200), 210);
        assert_eq!(count.get(), 2);

        assert_eq!(db.identity(&100), &100);
        assert_eq!(count.get(), 3);

        assert_eq!(db.identity(&100), &100);
        assert_eq!(count.get(), 3);
    }

    #[gtest]
    #[should_panic(expected = "Cycle detected: 'add10' depends on its own return value")]
    fn test_cycle() {
        crate::query_group! {
          pub struct Add10 {
            fn add10(&self, arg: i32) -> i32;
          }
        }
        fn add10(db: &Add10, arg: i32) -> i32 {
            db.add10(arg) // infinite recursion!
        }
        let db = Add10::new(add10);
        db.add10(1);
    }

    #[gtest]
    fn test_break_cycles_with_option() {
        crate::query_group! {
          pub struct Add10 {
            #[break_cycles_with = None]
            fn add10(&self, arg: i32) -> Option<i32>;
          }
        }
        fn add10(db: &Add10, arg: i32) -> Option<i32> {
            db.add10(arg)
        }
        let db = Add10::new(add10);
        assert_eq!(db.add10(1), None);
    }

    #[gtest]
    fn test_break_cycles_with_sentinel() {
        crate::query_group! {
          pub struct Add10 {
            #[break_cycles_with = -1]
            fn add10(&self, arg: i32) -> i32;
          }
        }
        fn add10(db: &Add10, arg: i32) -> i32 {
            db.add10(arg)
        }
        let db = Add10::new(add10);
        assert_eq!(db.add10(1), -1);
    }

    #[gtest]
    fn test_calls_in_cycle_are_not_memoized() {
        crate::query_group! {
          pub struct Table {
            #[input]
            fn logging(&self) -> Rc<RefCell<Vec<String>>>;

            #[input]
            fn records(&self) -> &'static [Record];

            #[break_cycles_with = false]
            fn is_unsafe(&self, name: &'static str) -> bool;

            fn record(&self, name: &'static str) -> Record;
          }
        }

        #[derive(Clone)]
        struct Record {
            name: &'static str,
            is_unsafe: bool,
            fields: &'static [&'static str],
        }

        // Returns whether or not a record is unsafe, checking recursively.
        fn is_unsafe(db: &Table, name: &'static str) -> bool {
            let record = db.record(name);
            let outcome =
                record.is_unsafe || record.fields.iter().any(|&field| db.is_unsafe(field));
            db.logging().borrow_mut().push(format!("is_unsafe({name}) = {outcome}"));
            outcome
        }

        // Helper function so we can refer to records by name instead of by index.
        fn record(db: &Table, name: &'static str) -> Record {
            db.records()
                .iter()
                .find(|record| record.name == name)
                .expect("Record not found")
                .clone()
        }

        let logging = Rc::default();

        let db = Table::new(
            Rc::clone(&logging),
            &[
                Record { name: "A", is_unsafe: false, fields: &["B", "Unsafe"] },
                Record { name: "B", is_unsafe: false, fields: &["A"] },
                Record { name: "Unsafe", is_unsafe: true, fields: &[] },
            ],
            is_unsafe,
            record,
        );
        // When checking if A is unsafe, it will first ask B, which will try to ask A
        // again, defaulting to false. So B says "I guess I'm safe", but _doesn't_
        // memoize that result. A will then see that it has Unsafe which is unsafe, so A
        // will memoize itself as unsafe. But when we go to ask B if it's unsafe now, it
        // will have correctly _not_ memoized that it's safe, and so it will ask
        // A again, which will again say "I am unsafe", and so B will correctly memoize
        // that it's unsafe.
        assert!(db.is_unsafe("A"));
        assert!(db.is_unsafe("B"));
        assert_eq!(
            logging.borrow().clone(),
            vec![
                "is_unsafe(B) = false".to_string(), // this is the cycle-default value
                "is_unsafe(Unsafe) = true".to_string(),
                "is_unsafe(A) = true".to_string(),
                "is_unsafe(B) = true".to_string(), // as we can see, the default wasn't memoized
            ]
        );
    }

    #[gtest]
    fn test_finite_recursion() {
        crate::query_group! {
          pub struct Add10 {
            #[input]
            fn call_counter(&self) -> Rc<Cell<i32>>;
            fn add10(&self, arg: i32) -> i32;
          }
        }
        fn add10(db: &Add10, arg: i32) -> i32 {
            db.call_counter().set(db.call_counter().get() + 1);
            if (arg % 10) != 0 {
                db.add10(arg - 1) + 1 // Some recursion, but not infinite!
            } else {
                arg + 10
            }
        }
        let db = Add10::new(Rc::new(Cell::new(0)), add10);

        assert_eq!(db.add10(100), 110);
        assert_eq!(db.call_counter().get(), 1);

        assert_eq!(db.add10(100), 110);
        assert_eq!(db.call_counter().get(), 1);

        assert_eq!(db.add10(205), 215);
        assert_eq!(db.call_counter().get(), 7);

        assert_eq!(db.add10(205), 215);
        assert_eq!(db.call_counter().get(), 7);
    }

    /// As an edge case (which perhaps isn't optimized well[^1]), you can even
    /// memoize a function which accepts no additional arguments, as a way
    /// of running a fixed computation at most once.
    ///
    /// [^1]: Since it has no inputs at all, we could store it as a bare `Option<ReturnType>`, but
    /// instead we use `HashMap<(), ReturnType>`. Well, good enough.
    #[gtest]
    fn test_argless() {
        crate::query_group! {
          pub struct Argless {
            #[input]
            fn call_counter(&self) -> Rc<Cell<i32>>;
            fn argless_function(&self) -> Rc<i32>;
          }
        }
        fn argless_function(db: &Argless) -> Rc<i32> {
            db.call_counter().set(db.call_counter().get() + 1);
            Rc::new(0)
        }
        let db = Argless::new(Rc::new(Cell::new(0)), argless_function);

        assert_eq!(db.call_counter().get(), 0);
        let argless_return = db.argless_function();
        assert_eq!(db.call_counter().get(), 1);
        let argless_return_2 = db.argless_function();
        assert_eq!(db.call_counter().get(), 1);
        assert!(Rc::ptr_eq(&argless_return, &argless_return_2));
    }

    #[gtest]
    fn test_provided_fn() {
        crate::query_group! {
          pub struct Db {
            #[input]
            fn input_fn(&self) -> i32;
            #[provided]
            fn provided_fn(&self, x: i32) -> i32 { self.input_fn() + x }
          }
        }
        let db = Db::new(42);
        let result = db.provided_fn(10);
        expect_eq!(result, 52);
    }

    #[gtest]
    fn test_separate_memoization() {
        crate::query_group! {
          pub struct Add10 {
            #[input]
            /// Tracker for how many times this function is called so we can check
            /// that memoization is indeed happening. This is just for testing;
            /// memoized functions in non-test code shouldn't have side effects,
            /// and inputs in non-test code shouldn't have internal mutability.
            fn call_counter(&self) -> Rc<Cell<i32>>;
          }
        }
        fn add10_impl(db: &Add10, arg: i32) -> i32 {
            db.call_counter().set(db.call_counter().get() + 1);
            arg + 10
        }
        crate::memoized!(pub fn add10(db: &Add10, arg: i32) -> i32 = add10_impl);
        let db = Add10::new(Rc::new(Cell::new(0)));

        assert_eq!(add10(&db, 100), 110);
        assert_eq!(db.call_counter().get(), 1);

        assert_eq!(add10(&db, 100), 110);
        assert_eq!(db.call_counter().get(), 1);

        assert_eq!(add10(&db, 200), 210);
        assert_eq!(db.call_counter().get(), 2);
    }
}
