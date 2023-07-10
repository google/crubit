// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_EXTERN_DEFINITION_EXTERN_DEFINITION_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_EXTERN_DEFINITION_EXTERN_DEFINITION_H_

#pragma clang lifetime_elision

// This is a regression test for b/245610602.  Note that to trigger the
// regression:
// * `MyTemplate` needs to be in a `namespace` (i.e.  the `namespace` below
// isn't
//   purely for code organisation).  This helps cover in the test that the IR
//   `Namespace` doesn't include `Record` of the `MyTemplate<int>` instantiation
//   as a child of the `extern_definition` namespace.
// * There should be no instantiations of the template in this header and target
//   (e.g. no `using TypeAlias = MyTemplate<int>`), because this would generate
//   a `Record` for the instantiation in the top-level namespace (which would
//   mask the bug).
//
// This testcase mimics code from `llvm/include/c++/v1/__locale`:
//
//     _LIBCPP_BEGIN_NAMESPACE_STD
//     ...
//     template <class _CharT>
//     class _LIBCPP_TEMPLATE_VIS collate
//         : public locale::facet
//     {
//       ...
//     };
//     ...
//     extern template class _LIBCPP_EXTERN_TEMPLATE_TYPE_VIS collate<char>;
// ```
namespace extern_definition {

template <typename T>
class MyTemplate final {
 public:
  static MyTemplate Create(T value) {
    MyTemplate result;
    result.value_ = value;
    return result;
  }

  const T& value() const { return value_; }

 private:
  T value_;
};

extern template class MyTemplate<int>;

}  // namespace extern_definition

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_EXTERN_DEFINITION_EXTERN_DEFINITION_H_
