// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for function calls.

#include <memory>

#include "nullability_test.h"

namespace std {
template <class T>
struct optional {
  bool has_value() const;
  T *operator->();
  const T *operator->() const;
  const T &operator*() const;
  const T &value() const;
};
}  // namespace std

namespace absl {
template <typename T>
class StatusOr {
 public:
  bool ok() const;
  const T &operator*() const &;
  T &operator*() &;
  const T *operator->() const;
  T *operator->();
  const T &value() const;
  T &value();
};
}  // namespace absl

namespace call_expr_with_pointer_return_type_free_function {

Nonnull<int *> makeNonnull();
Nullable<int *> makeNullable();
int *makeUnannotated();

TEST void callExprWithPointerReturnTypeFreeFunction() {
  nonnull(makeNonnull());
  nullable(makeNullable());
  unknown(makeUnannotated());
}

}  // namespace call_expr_with_pointer_return_type_free_function

namespace call_expr_with_pointer_return_type_member_function {

struct Foo {
  Nonnull<int *> makeNonnull();
  Nullable<int *> makeNullable();
  int *makeUnannotated();
};

TEST void callExprWithPointerReturnTypeMemberFunction(Foo foo) {
  nonnull(foo.makeNonnull());
  nullable(foo.makeNullable());
  unknown(foo.makeUnannotated());
}

}  // namespace call_expr_with_pointer_return_type_member_function

TEST void callExprWithPointerReturnTypeFunctionPointer(
    Nonnull<int *> (*makeNonnull)(), Nullable<int *> (*makeNullable)(),
    int *(*makeUnannotated)()) {
  nonnull(makeNonnull());
  nullable(makeNullable());
  unknown(makeUnannotated());
}

TEST void callExprWithPointerReturnTypePointerToFunctionPointer(
    Nonnull<int *> (**makeNonnull)(), Nullable<int *> (**makeNullable)(),
    int *(**makeUnannotated)()) {
  nonnull((*makeNonnull)());
  nullable((*makeNullable)());
  unknown((*makeUnannotated)());
}

namespace call_expr_with_pointer_return_type_function_pointer_nested {

// Function returning a function pointer which returns a pointer.

typedef int *_Nonnull (*MakeNonnullT)();
typedef int *_Nullable (*MakeNullableT)();
typedef int *(*MakeUnannotatedT)();

TEST void callExprWithPointerReturnTypeFunctionPointerNested(
    MakeNonnullT (*makeNonnull)(), MakeNullableT (*makeNullable)(),
    MakeUnannotatedT (*makeUnannotated)()) {
  nonnull((*makeNonnull)()());
  nullable((*makeNullable)()());
  unknown((*makeUnannotated)()());
}

}  // namespace call_expr_with_pointer_return_type_function_pointer_nested

namespace call_expr_with_pointer_return_type_pointer_ref {

// Free function returns reference to pointer.

int *_Nonnull &makeNonnull();
int *_Nullable &makeNullable();
int *&makeUnannotated();

TEST void callExprWithPointerReturnTypePointerRef() {
  nonnull(makeNonnull());
  nullable(makeNullable());
  unknown(makeUnannotated());

  // Check that we can take the address of the returned reference and still
  // see the correct nullability "behind" the resulting pointer.
  type<Nonnull<Nonnull<int *> *>>(&makeNonnull());
  type<Nonnull<Nullable<int *> *>>(&makeNullable());
  type<Nonnull<NullabilityUnknown<int *> *>>(&makeUnannotated());
}

}  // namespace call_expr_with_pointer_return_type_pointer_ref

namespace call_expr_with_pointer_return_type_in_loop {

// Function called in loop.

Nullable<int *> makeNullable();
bool makeBool();

TEST void callExprWithPointerReturnTypeInLoop() {
  bool first = true;
  while (true) {
    int *x = makeNullable();
    if (first && x == nullptr) return;
    first = false;
    nullable(x);
  }
}

}  // namespace call_expr_with_pointer_return_type_in_loop

namespace output_parameter_basic {

void maybeModifyPtr(int **p);

TEST void outputParameterBasic() {
  int *p = nullptr;
  maybeModifyPtr(&p);
  unknown(p);
}

}  // namespace output_parameter_basic

namespace output_parameter_reference {

void maybeModifyPtr(int *&r);

TEST void outputParameterReference() {
  int *p = nullptr;
  maybeModifyPtr(p);
  unknown(p);
}

}  // namespace output_parameter_reference

namespace output_parameter_reference_const {

void pointerNotModified(int *const &r);
TEST void outputParameterReferenceConst() {
  int *p = nullptr;
  pointerNotModified(p);
  nullable(p);
}

}  // namespace output_parameter_reference_const

namespace output_parameter_reference_pointer_to_pointer {

void maybeModifyPtr(int **&r);

TEST void outputParameterReferencePointerToPointer() {
  int **pp = nullptr;
  maybeModifyPtr(pp);
  unknown(pp);
  unknown(*pp);
}

}  // namespace output_parameter_reference_pointer_to_pointer

namespace output_parameter_member_pointer_to_pointer {

// This is a crash repro. We don't yet support pointers-to-members -- we just
// care that this doesn't cause the analysis to crash.

struct S {
  int *p;
};

void callee(int *S::*);

TEST void outputParameterMemberPointerToPointer(int *S::*ptr_to_member_ptr) {
  callee(ptr_to_member_ptr);
}

}  // namespace output_parameter_member_pointer_to_pointer

namespace output_parameter_const {

void pointerNotModified(int *const *p);

TEST void outputParameterConst(int *_Nullable p) {
  pointerNotModified(&p);
  nullable(p);
}

// The only const qualifier that should be considered is on the inner
// pointer, otherwise we should assume that the pointer may be modified.

void maybeModifyPtr(const int **const p);

TEST void innerPointerNotConst() {
  const int *p = nullptr;
  maybeModifyPtr(&p);
  unknown(p);
}

}  // namespace output_parameter_const

namespace output_parameter_nonnull {

void pointerNotModified(int *_Nonnull *p);

TEST void outputParameterNonnull(int *_Nonnull p) {
  pointerNotModified(&p);
  nonnull(p);
}

}  // namespace output_parameter_nonnull

namespace output_parameter_checked_nullable {

void maybeModify(int *_Nullable *p);

TEST void outputParameterCheckedNullable(int *_Nullable p) {
  if (!p) return;
  maybeModify(&p);
  // The analysis comes to the wrong conclusion here -- the pointer is actually
  // nullable.
  nonnull(p);
}

}  // namespace output_parameter_checked_nullable

namespace output_parameter_nullable {

void maybeModifyPtr(int *_Nullable *p);

TEST void outputParameterNullable() {
  int *p = nullptr;
  maybeModifyPtr(&p);
  nullable(p);
}

}  // namespace output_parameter_nullable

namespace output_parameter_conditional {

// This tests that flow sensitivity is preserved, to catch for example if the
// underlying pointer was always set to Nonnull once it's passed as an
// output parameter.

void maybeModifyPtr(int **p);

TEST void outputParameterConditional(int *_Nullable j, bool b) {
  if (b) {
    maybeModifyPtr(&j);
  }
  if (b) {
    unknown(j);
  }
  if (!b) {
    nullable(j);
  }
}

}  // namespace output_parameter_conditional

namespace output_parameter_without_ampersand_operator {

// This tests that the call to maybeModifyPtr works as expected if the param
// passed in doesn't directly use the & operator

void maybeModifyPtr(int **p);

TEST void outputParameterWithoutAmpersandOperator(int *_Nullable p) {
  auto pp = &p;
  maybeModifyPtr(pp);
  unknown(p);
}

}  // namespace output_parameter_without_ampersand_operator

namespace output_parameter_template {

template <typename T>
struct S {
  void maybeModify(T &ref);
};

TEST void outputParameterTemplate(S<int *> s, int *_Nullable p) {
  s.maybeModify(p);
  unknown(p);
}

TEST void outputParameterTemplateNullable(S<int *_Nullable> s,
                                          int *_Nullable p) {
  s.maybeModify(p);
  // Doesn't correctly pass on nullability in template argument.
  unknown(p);
}

TEST void outputParameterTemplateNonnull(S<int *_Nonnull> s, int *_Nonnull p) {
  s.maybeModify(p);
  // Doesn't correctly pass on nullability in template argument.
  unknown(p);
}

}  // namespace output_parameter_template

namespace output_parameter_variadic_callee {

void maybeModifyPtr(int **p, ...);

TEST void outputParameterVariadicCallee() {
  int *p = nullptr;
  maybeModifyPtr(&p, 0);
  unknown(p);
}

}  // namespace output_parameter_variadic_callee

namespace output_parameter_member_operator {

struct MaybeModifyPtr {
  void operator()(int **p);
};

TEST void outputParameterMemberOperator() {
  int *p = nullptr;
  MaybeModifyPtr()(&p);
  unknown(p);
}

}  // namespace output_parameter_member_operator

namespace can_overwrite_ptr_with_ptr_created_from_ref_return_type {

// Test that if we create a pointer from a function returning a reference, we
// can use that pointer to overwrite an existing nullable pointer and make it
// nonnull.

int &get_int();

TEST void canOverwritePtrWithPtrCreatedFromRefReturnType(int *_Nullable i) {
  i = &get_int();
  nonnull(i);
}

}  // namespace can_overwrite_ptr_with_ptr_created_from_ref_return_type

namespace can_overwrite_ptr_with_ptr_returned_by_function {

int *_Nonnull get_int();

TEST void canOverwritePtrWithPtrReturnedByFunction(int *_Nullable i) {
  i = get_int();
  nonnull(i);
}

}  // namespace can_overwrite_ptr_with_ptr_returned_by_function

namespace call_member_operator_no_params {

struct MakeNonnull {
  int *_Nonnull operator()();
};
struct MakeNullable {
  int *_Nullable operator()();
};
struct MakeUnannotated {
  int *operator()();
};

TEST void callMemberOperatorNoParams() {
  MakeNonnull makeNonnull;
  nonnull(makeNonnull());

  MakeNullable makeNullable;
  nullable(makeNullable());

  MakeUnannotated makeUnannotated;
  unknown(makeUnannotated());
}

}  // namespace call_member_operator_no_params

namespace call_free_operator {

// No nullability involved. This is just a regression test to make sure we can
// process a call to a free overloaded operator.

struct A {};
A operator+(A, A);

TEST void callFreeOperator() {
  A a;
  a = a + a;
}

}  // namespace call_free_operator

namespace distinguish_function_return_type_and_params {

int *_Nullable callee(int *_Nonnull);

TEST void callExprDistinguishFunctionReturnTypeAndParams() {
  int i = 0;
  type<Nullable<int *>>(callee(&i));
}

}  // namespace distinguish_function_return_type_and_params

namespace distinguish_method_return_type_and_params {

struct S {
  int *_Nullable callee(int *_Nonnull);
};

TEST void distinguishMethodReturnTypeAndParams(S s) {
  int i = 0;
  type<Nullable<int *>>(s.callee(&i));
}

}  // namespace distinguish_method_return_type_and_params

namespace class_template_distinguish_method_return_type_and_params {

template <typename T0, typename T1>
struct S {
  T0 callee(T1);
};

TEST void classTemplateDistinguishMethodReturnTypeAndParams(
    S<int *_Nullable, int *_Nonnull> s) {
  int i = 0;
  type<Nullable<int *>>(s.callee(&i));
}

}  // namespace class_template_distinguish_method_return_type_and_params

namespace call_function_template_template_arg_in_return_type_has_null_type_source_info {

// This test sets up a function call where we don't have a `TypeSourceInfo`
// for the argument to a template parameter used in the return type.
// This is a regression test for a crash that we observed on real-world code.

template <class T>
struct A {
  using Type = T;
};
template <int, class T>
typename A<T>::Type f(T);

TEST void callFunctionTemplate_TemplateArgInReturnTypeHasNullTypeSourceInfo() {
  f<0>(1);
}

}  // namespace
   // call_function_template_template_arg_in_return_type_has_null_type_source_info

namespace call_function_template_partially_deduced {

template <int, class T>
T f(T);

TEST void callFunctionTemplate_PartiallyDeduced() { f<0>(1); }

}  // namespace call_function_template_partially_deduced

// Crash repro.

TEST void callBuiltinFunction() { __builtin_operator_new(0); }

namespace const_method_no_params_check_first {

struct C {
  int *_Nullable property() const { return x; }
  int *_Nullable x = nullptr;
};

TEST void constMethodNoParamsCheckFirst() {
  C obj;
  if (obj.property() != nullptr) nonnull(obj.property());
}

}  // namespace const_method_no_params_check_first

namespace const_method_no_impl {

struct C {
  int *_Nullable property() const;
  void may_mutate();
  C &operator=(const C &);
};

TEST void constMethodNoImpl() {
  C obj;
  if (obj.property() != nullptr) {
    obj.may_mutate();
    nullable(obj.property());
  };
  if (obj.property() != nullptr) {
    // A non-const operator call may mutate as well.
    obj = C();
    nullable(obj.property());
  };
  if (obj.property() != nullptr) nonnull(obj.property());
}

}  // namespace const_method_no_impl

namespace const_method_returns_reference {

struct C {
  int *const _Nullable &property() const { return x; }
  int *_Nullable x = nullptr;
};

TEST void constMethodReturnsReference() {
  C obj;
  if (obj.property() != nullptr) nonnull(obj.property());
}

}  // namespace const_method_returns_reference

namespace const_method_early_return {

struct C {
  int *_Nullable property() const;
};

TEST void constMethodEarlyReturn() {
  C c;
  if (!c.property()) return;
  // We correctly deduce nonnull here as there is no join.
  nonnull(c.property());
}

}  // namespace const_method_early_return

namespace const_method_with_conditional {

struct C {
  int *_Nullable property() const;
};
bool cond();
void some_operation(int);

TEST void constMethodWithConditional() {
  C c;
  if (!c.property()) return;
  if (cond()) {
    some_operation(1);
  } else {
    some_operation(2);
  }
  // Verify that we still model `c.property()` as returning the same value
  // after the join, i.e. a null check performed before control flow
  // diverges is still valid when the paths rejoin.
  nonnull(c.property());
}

}  // namespace const_method_with_conditional

namespace const_method_null_check_on_only_one_branch {

struct C {
  int *_Nullable property() const;
};
bool cond();

TEST void constMethodNullCheckOnOnlyOneBranch() {
  C c;
  if (cond()) {
    if (!c.property()) return;
  }
  // We didn't check for null on all paths that reach this dereference, so
  // the return value is still nullable.
  nullable(c.property());
}

}  // namespace const_method_null_check_on_only_one_branch

namespace const_method_conditional_with_separate_null_checks {

struct C {
  int *_Nullable property() const;
};
bool cond();

TEST void constMethodConditionalWithSeparateNullChecks() {
  C c;
  if (cond()) {
    if (!c.property()) return;
  } else {
    if (!c.property()) return;
  }
  // TODO: The analysis reaches a wrong conclusion here: We checked for null on
  // all paths that reach this point, but the lattice doesn't join the return
  // values we generated for `c.property()` on the two branches, so we don't see
  // that the pointer is nonnull. This pattern is likely to be rare in practice,
  // so it doesn't seem worth making the join operation more complex to support
  // this.
  nullable(c.property());
}

}  // namespace const_method_conditional_with_separate_null_checks

namespace const_method_join_loses_information {

struct A {
  bool cond() const;
};
struct C {
  int *_Nullable property() const;
  A a() const;
};

TEST void constMethodJoinLosesInformation(const C &c1, const C &c2) {
  if (c1.property() == nullptr || c2.property() == nullptr || c2.a().cond())
    return;
  nonnull(c1.property());
  // TODO(b/359457439): This is a false positive, caused by a suboptimal CFG
  // structure. All of the possible edges out of the if statement's
  // condition join in a single CFG block before branching out again to
  // the `return` block on the one hand and the block that performs the
  // dereferences on the other.
  // When we perform the join for the various edges out of the condition,
  // we discard the return value for `c2.property()` because in the case
  // where `c1.property()` is null, we never evaluate `c2.property()` and
  // hence don't have a return value for it. When we call `c2.property()`
  // again, we therefore create a fresh return value for it, and we hence
  // cannot infer that this value is nonnull.
  // The false positive does not occur if `c2.a().cond()` is replaced with
  // a simpler condition, e.g. `c2.cond()` (assuming that `cond()` is
  // moved to `C`). In this case, the CFG is structured differently: All of
  // the edges taken when one of the conditions in the if state is true
  // lead directly to the `return` block, and the edge taken when all
  // conditions are false leads diresctly to the block that performs the
  // dereferences. No join is performed, and we can therefore conclude that
  // `c2.property()` is nonnull.
  // I am not sure what causes the different CFG structure in the two cases,
  // but it may be triggered by the `A` temporary that is returned by `a()`.
  nullable(c2.property());
}

}  // namespace const_method_join_loses_information

namespace const_method_no_record_for_call_object {

struct S {
  int *_Nullable property() const;
};

S makeS();

TEST void constMethodNoRecordForCallObject() {
  if (makeS().property()) {
    // This is a const member call on a different object, so we can't infer
    // anything about the return value of `makeS().property()`.
    // But this line and the line above also don't cause any crashes.
    nullable(makeS().property());
  }
}

}  // namespace const_method_no_record_for_call_object

namespace const_method_returning_bool {

// This tests (indirectly) that we also model const methods returning
// booleans. We use `operator bool()` as the specific const method because
// this then also gives us coverage of this special case (which is quite
// common, for example in `std::function`).

struct S {
  operator bool() const;
};

TEST void constMethodReturningBool(S s) {
  int *p = nullptr;
  int i = 0;
  if (s) p = &i;
  if (s)
    // We know `p` is nonnull because we know `operator bool()` will return the
    // same thing both times.
    nonnull(p);
}

}  // namespace const_method_returning_bool

namespace const_method_returning_smart_pointer {

struct S {
  Nullable<std::shared_ptr<int>> property() const;
};

TEST void constMethodReturningSmartPointer() {
  S s;
  if (s.property() != nullptr) {
    nonnull(s.property());
  }
}

}  // namespace const_method_returning_smart_pointer

namespace const_method_returning_smart_pointer_by_reference {

struct S {
  const Nullable<std::shared_ptr<int>> &property() const;
};

TEST void constMethodReturningSmartPointerByReference() {
  S s;
  if (s.property() != nullptr) {
    nonnull(s.property());
  }
}

}  // namespace const_method_returning_smart_pointer_by_reference

namespace const_operator_returning_pointer {

struct S {
  Nullable<int *> x;
};
struct SmartPtr {
  S *operator->() const;
  void clear();
};

TEST void constOperatorReturningPointer() {
  SmartPtr ptr;
  if (ptr->x != nullptr) {
    nonnull(ptr->x);
    ptr.clear();
    nullable(ptr->x);
  }
}

}  // namespace const_operator_returning_pointer

namespace non_const_method_clears_smart_pointer {

struct S {
  Nullable<std::shared_ptr<int>> property() const;
  void writer();
};

TEST void nonConstMethodClearsSmartPointer() {
  S s;
  if (s.property() != nullptr) {
    s.writer();
    nullable(s.property());
  }
}

}  // namespace non_const_method_clears_smart_pointer

namespace non_const_method_clears_pointer_members {

struct S {
  Nullable<int *> p;
  void writer();
};

TEST void nonConstMethodClearsPointerMembers(S s) {
  if (s.p != nullptr) {
    s.writer();
    nullable(s.p);
  }
}

}  // namespace non_const_method_clears_pointer_members

namespace non_const_method_does_not_clear_const_pointer_members {

struct S {
  const Nullable<int *> cp;
  void writer();
};

TEST void nonConstMethodDoesNotClearConstPointerMembers(S s) {
  if (s.cp != nullptr) {
    s.writer();
    nonnull(s.cp);
  }
}

}  // namespace non_const_method_does_not_clear_const_pointer_members

namespace optional_operator_arrow_and_star_call {

// Check that repeated accesses to a pointer behind an optional are considered
// to yield the same pointer -- but only if the optional is not modified in
// the meantime.

struct S {
  int *_Nullable p;
};

TEST void optionalOperatorArrowAndStarCall(std::optional<S> opt1,
                                           std::optional<S> opt2) {
  if (!opt1.has_value() || !opt2.has_value()) return;
  *opt1->p;  // [[unsafe]]
  if (opt1->p != nullptr) {
    nonnull(opt1->p);
    nonnull((*opt1).p);
    nonnull(opt1.value().p);
    opt1 = opt2;
    nullable(opt1->p);
  }
}

}  // namespace optional_operator_arrow_and_star_call

namespace status_or_operator_arrow_and_star_call {

// Check that repeated accesses to a pointer behind a StatusOr (or similar
// smart pointer-like class) are considered to yield the same pointer --
// but only if it is not modified in the meantime.

struct S {
  int *_Nullable p;
};

TEST void statusOrOperatorArrowAndStarCall(absl::StatusOr<S> sor1,
                                           absl::StatusOr<S> sor2) {
  if (!sor1.ok() || !sor2.ok()) return;
  nullable(sor1->p);
  if (sor1->p != nullptr) {
    nonnull(sor1->p);
    nonnull((*sor1).p);
    nonnull(sor1.value().p);
    sor1 = sor2;
    nullable(sor1->p);
  }
}

}  // namespace status_or_operator_arrow_and_star_call

namespace field_undefined_value {

struct C {
  int *_Nullable property() const { return x; }
  int *_Nullable x = nullptr;
};
C foo();

TEST void fieldUndefinedValue() {
  if (foo().x != nullptr) nullable(foo().x);
}

}  // namespace field_undefined_value

namespace get_reference_then_call_accessor {

struct C {
  int *_Nullable property() const { return x; }
  int *_Nullable x = nullptr;
};
C &foo();

TEST void getReferenceThenCallAccessor() {
  const C &c = foo();
  if (c.property() != nullptr) {
    nonnull(c.property());
  }
}

}  // namespace get_reference_then_call_accessor

namespace accessor_to_get_reference_then_call_accessor {

struct C {
  int *_Nullable property() const { return x; }
  int *_Nullable x = nullptr;
};

struct SmartPtrLike {
  C &operator*() const;
  C *operator->() const;
  C *get() const;
};

TEST void accessorToGetReferenceThenCallAccessor(SmartPtrLike &d) {
  const C &obj = *d;
  if (obj.property() != nullptr) {
    nonnull(obj.property());
  }
}

}  // namespace accessor_to_get_reference_then_call_accessor

namespace get_reference_then_call_accessor_then_get_reference {

struct A {
  int *_Nullable x;
};

struct B {
  const A &f;
};

struct C {
  B &nonConstGetRef();
};

TEST void getReferenceThenCallAccessorThenGetReference(C c) {
  B &b = c.nonConstGetRef();
  if (b.f.x == nullptr) return;
  // TODO(b/396431434): `b.f.x` should be nullable here. However we currently
  // don't get a storage location for `b.f` when we dynamically create the
  // parent storage location for `b` from the `nonConstGetRef` call. Then we
  // fail to get nullability properties for `b.f.x`.
  nullable(b.f.x);
}

}  // namespace get_reference_then_call_accessor_then_get_reference

namespace method_no_params_undefined_value {

struct C {
  int *_Nullable property() const { return x; }
  int *_Nullable x = nullptr;
};

TEST void methodNoParamsUndefinedValue() {
  if (C().property() != nullptr) {
    nullable(C().property());
  }
  C obj;
  if (obj.property() != nullptr) {
    nonnull(obj.property());
  }
}

}  // namespace method_no_params_undefined_value

namespace call_pseudo_destructor {

// Repro for assertion failure:
// We used to assert-fail on calls to `CXXPseudoDestructorExpr` because we
// didn't detect that they were "bound member function types" (with which we
// don't associate nullability as they aren't pointers).

using Int = int;
TEST void callPseudoDestructor(Int i) { i.~Int(); }

}  // namespace call_pseudo_destructor
