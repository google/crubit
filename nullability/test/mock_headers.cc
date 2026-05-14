// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/test/mock_headers.h"

#include <utility>
#include <vector>

#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability::test {

// This header is included in all code passed to checkDiagnostics.
static constexpr char CheckDiagnosticsPreamble[] = R"cc(
#ifndef CRUBIT_NULLABILITY_TEST_CHECK_DIAGNOSTICS_PREAMBLE_H_
#define CRUBIT_NULLABILITY_TEST_CHECK_DIAGNOSTICS_PREAMBLE_H_

#include "nullability_annotations.h"  // IWYU pragma: export

  enum NullabilityKind {
    NK_nonnull,
    NK_nullable,
    NK_unspecified,
  };

  template <NullabilityKind... NK, typename T>
  void __assert_nullability(const T&);

  template <typename T>
  T value();

#endif  // CRUBIT_NULLABILITY_TEST_CHECK_DIAGNOSTICS_PREAMBLE_H_
)cc";

// Mock of CHECK macros from Abseil's logging library.
static constexpr char CheckHeader[] = R"cc(
#ifndef CRUBIT_NULLABILITY_TEST_CHECK_H_
#define CRUBIT_NULLABILITY_TEST_CHECK_H_

  struct string;

  // The definitions below model assorted definitions in Abseil's logging
  // library.

  // `something` is chosen to indicate that we don't want to depend on the
  // details of which particular namespace the real implementations reside in.
  // They need only be somewhere in `absl`.
  namespace absl::something {
  template <typename T>
  const T& GetReferenceableValue(const T&);

  template <typename T1, typename T2>
  string* Check_NEImpl(const T1&, const T2&, const char*);

  class LogMessageFatal {
   public:
    LogMessageFatal();
    [[noreturn]] ~LogMessageFatal();
    LogMessageFatal& InternalStream();
  };
  }  // namespace absl::something

#define CHECK_OP(name, op, a, b)                                  \
    while (string* result = ::absl::something::name##Impl(          \
               ::absl::something::GetReferenceableValue(a),         \
               ::absl::something::GetReferenceableValue(b), "msg")) \
    ::absl::something::LogMessageFatal().InternalStream()

#define CHECK_NE(a, b) CHECK_OP(Check_NE, !=, (a), (b))

#endif  // CRUBIT_NULLABILITY_TEST_CHECK_H_
)cc";

// Mock of standard library <memory> header.
static constexpr char MemoryHeader[] = R"cc(
#ifndef CRUBIT_NULLABILITY_TEST_MEMORY_
#define CRUBIT_NULLABILITY_TEST_MEMORY_

#include <type_traits>
#include <utility>

  namespace std {

  // Adding some definitions here for convenience that really belong in other
  // headers.

  using nullptr_t = decltype(nullptr);

  using size_t = decltype(sizeof(int));

  template <class T>
  struct remove_extent {
    using type = T;
  };
  template <class T>
  struct remove_extent<T[]> {
    using type = T;
  };
  template <class T, size_t N>
  struct remove_extent<T[N]> {
    using type = T;
  };

  template <class T>
  struct default_delete {};
  template <class T>
  struct allocator {};

  template <class...>
  using void_t = void;

  template <class T, class Deleter, class = void>
  struct __underlying_ptr_type {
    using type = T*;
  };

  template <class T, class Deleter>
  // The standard says we should apply `std::remove_reference` to `Deleter`,
  // but we don't need this for our tests, so we omit it for simplicity.
  struct __underlying_ptr_type<T, Deleter,
                               std::void_t<typename Deleter::pointer>> {
    using type = typename Deleter::pointer;
  };

  template <class T, class Deleter = default_delete<T>>
  class unique_ptr {
   public:
    using pointer = __underlying_ptr_type<T, Deleter>::type;

    unique_ptr();
    unique_ptr(nullptr_t);
    unique_ptr(pointer);
    // We're restricting ourselves to just one possible case for the deleter
    // parameter.
    unique_ptr(pointer, const Deleter&);
    unique_ptr(unique_ptr&&);
    template <class OtherT, class OtherDeleter>
    unique_ptr(unique_ptr<OtherT, OtherDeleter>&&);
    unique_ptr(const unique_ptr&) = delete;

    // Make sure `unique_ptr` is not trivially destructible so that the AST for
    // expressions using this mock look the same as for the real class.
    ~unique_ptr();

    unique_ptr& operator=(unique_ptr&&);
    template <class OtherT, class OtherDeleter>
    unique_ptr& operator=(unique_ptr<OtherT, OtherDeleter>&&);
    unique_ptr& operator=(nullptr_t);
    unique_ptr& operator=(const unique_ptr&) = delete;

    T& operator*() const;
    T* operator->() const;

    pointer release();

    void reset(pointer p = pointer());

    void swap(unique_ptr&);

    pointer get() const;

    explicit operator bool() const;
  };

  // Array specialization of `unique_ptr`. We only introduce the members that
  // are used in tests.
  template <class T, class Deleter>
  class unique_ptr<T[], Deleter> {
   public:
    using pointer = T*;

    unique_ptr();
    template <class ArrayT>
    explicit unique_ptr(ArrayT);
    template <class ArrayT>
    explicit unique_ptr(ArrayT, const Deleter&);
    unique_ptr(const unique_ptr&) = delete;

    ~unique_ptr();

    template <class ArrayT>
    void reset(ArrayT);
    void reset(nullptr_t = nullptr);

    pointer get() const;

    T& operator[](size_t) const;
  };

  // We're restricting ourselves to just some of the overloads that are required
  // to provide sufficient test coverage.
  template <class T, class... Args, enable_if<!__is_array(T), int>::type = 0>
  unique_ptr<T> make_unique(Args&&... args) {
    return unique_ptr<T>(new T(std::forward<Args>(args)...));
  }

  template <class T, enable_if<__is_unbounded_array_v<T>, int>::type = 0>
  unique_ptr<T> make_unique(size_t n) {
    using ET = remove_extent<T>::type;
    return unique_ptr<T>(new ET[n]);
  }

  template <class T>
  unique_ptr<T> make_unique_for_overwrite();
  template <class T>
  unique_ptr<T> make_unique_for_overwrite(size_t);

  template <class T, class Deleter>
  void swap(unique_ptr<T, Deleter>&, unique_ptr<T, Deleter>&);

  template <class T>
  bool operator==(const unique_ptr<T>&, std::nullptr_t);

  // C++20 and later do not define the following overloads because they are
  // provided by rewritten candidates instead.
#if __cplusplus < 202002L
  template <class T>
  bool operator==(std::nullptr_t, const unique_ptr<T>&);
#endif  // __cplusplus < 202002L

  template <class T>
  class weak_ptr;

  // This definition of `shared_ptr` only introduces the members that are used
  // in tests.
  template <class T>
  class shared_ptr {
   public:
    using element_type = typename remove_extent<T>::type;

    shared_ptr();
    template <class OtherT>
    shared_ptr(OtherT*);
    template <class Deleter>
    shared_ptr(nullptr_t, Deleter);
    template <class Deleter, class Alloc>
    shared_ptr(nullptr_t, Deleter, Alloc);
    template <class OtherT, class Deleter, class Alloc>
    shared_ptr(OtherT*, Deleter, Alloc);
    template <class OtherT>
    shared_ptr(const shared_ptr<OtherT>&, element_type*);
    template <class OtherT>
    shared_ptr(shared_ptr<OtherT>&&, element_type*);
    shared_ptr(const shared_ptr&);
    template <class OtherT>
    explicit shared_ptr(const weak_ptr<OtherT>&);
    template <class OtherT, class Deleter>
    explicit shared_ptr(unique_ptr<OtherT, Deleter>&&);

    ~shared_ptr();

    shared_ptr& operator=(const shared_ptr&);
    template <class OtherT>
    shared_ptr& operator=(const shared_ptr<OtherT>&);

    T& operator*() const;

    void reset();
    template <class OtherT>
    void reset(OtherT*);
    template <class OtherT, class Deleter>
    void reset(OtherT*, Deleter);
    template <class OtherT, class Deleter, class Alloc>
    void reset(OtherT*, Deleter, Alloc);

    element_type* get() const;

    explicit operator bool() const;
  };

  // We're restricting ourselves to just some of the overloads that are required
  // to provide sufficient test coverage.
  template <class T>
  shared_ptr<T> make_shared();
  template <class T, class... Args>
  shared_ptr<T> make_shared(Args&&... args);
  template <class T>
  shared_ptr<T> make_shared_for_overwrite();
  template <class T>
  shared_ptr<T> make_shared_for_overwrite(size_t);

  template <class T, class Alloc>
  shared_ptr<T> allocate_shared(const Alloc&);
  template <class T, class Alloc, class... Args>
  shared_ptr<T> allocate_shared(const Alloc&, Args&&... args);
  template <class T, class Alloc>
  shared_ptr<T> allocate_shared_for_overwrite(const Alloc&);
  template <class T, class Alloc>
  shared_ptr<T> allocate_shared_for_overwrite(const Alloc&, size_t);

  template <class DestT, class SrcT>
  shared_ptr<DestT> static_pointer_cast(const shared_ptr<SrcT>&);
  template <class DestT, class SrcT>
  shared_ptr<DestT> static_pointer_cast(const shared_ptr<SrcT>&&);
  template <class DestT, class SrcT>
  shared_ptr<DestT> dynamic_pointer_cast(const shared_ptr<SrcT>&);
  template <class DestT, class SrcT>
  shared_ptr<DestT> dynamic_pointer_cast(const shared_ptr<SrcT>&&);
  template <class DestT, class SrcT>
  shared_ptr<DestT> const_pointer_cast(const shared_ptr<SrcT>&);
  template <class DestT, class SrcT>
  shared_ptr<DestT> const_pointer_cast(const shared_ptr<SrcT>&&);
  template <class DestT, class SrcT>
  shared_ptr<DestT> reinterpret_pointer_cast(const shared_ptr<SrcT>&);
  template <class DestT, class SrcT>
  shared_ptr<DestT> reinterpret_pointer_cast(const shared_ptr<SrcT>&&);

  template <class T1, class T2>
  bool operator==(const shared_ptr<T1>&, const shared_ptr<T2>&);
  template <class T>
  bool operator==(const shared_ptr<T>&, std::nullptr_t);

  // C++20 and later do not define the following overloads because they are
  // provided by rewritten candidates instead.
#if __cplusplus < 202002L
  template <class T1, class T2>
  bool operator!=(const shared_ptr<T1>&, const shared_ptr<T2>&);
  template <class T>
  bool operator==(std::nullptr_t, const shared_ptr<T>&);
  template <class T>
  bool operator!=(const shared_ptr<T>&, std::nullptr_t);
  template <class T>
  bool operator!=(std::nullptr_t, const shared_ptr<T>&);
#endif  // __cplusplus < 202002L

  // This mock only provides the definitions needed to test that
  // `weak_ptr::lock()` returns a nullable pointer.
  template <class T>
  class weak_ptr {
   public:
    template <class OtherT>
    weak_ptr(const shared_ptr<OtherT>&);
    shared_ptr<T> lock() const;
  };

  }  // namespace std

#endif  // CRUBIT_NULLABILITY_TEST_MEMORY_
)cc";

// Mock of standard library <new> header.
static constexpr char NewHeader[] = R"cc(
#ifndef CRUBIT_NULLABILITY_TEST_NEW_
#define CRUBIT_NULLABILITY_TEST_NEW_

  namespace std {
  struct nothrow_t {
    explicit nothrow_t() = default;
  };
  extern const nothrow_t nothrow;
  using size_t = decltype(sizeof(int));
  }  // namespace std
  void* operator new(std::size_t size, const std::nothrow_t&) noexcept;

#endif  // CRUBIT_NULLABILITY_TEST_NEW_
)cc";

// Macros for nullability annotations.
static constexpr char NullabilityAnnotationsHeader[] = R"cc(
#ifndef CRUBIT_NULLABILITY_TEST_NULLABILITY_ANNOTATIONS_H_
#define CRUBIT_NULLABILITY_TEST_NULLABILITY_ANNOTATIONS_H_

  template <typename T>
  using Nullable [[clang::annotate("Nullable")]] = T _Nullable;

  template <typename T>
  using Nonnull [[clang::annotate("Nonnull")]] = T _Nonnull;

  template <typename T>
  using NullabilityUnknown [[clang::annotate("Nullability_Unspecified")]] =
      T _Null_unspecified;

#define absl_nullable _Nullable
#define absl_nonnull _Nonnull
#define absl_nullability_unknown _Null_unspecified
#define absl_nullability_conflict _Null_unspecified

#endif  // CRUBIT_NULLABILITY_TEST_NULLABILITY_ANNOTATIONS_H_
)cc";

static constexpr char NullabilityTestHeader[] = R"cc(
  // This header defines functions available in nullability_tests.
  //
  // A test is a C++ source file that contains code to be analyzed.
  // Any functions marked with TEST are analysis targets.
  // These can include calls to assertion functions like nullable() defined
  // here. Such calls assert details of analysis results (nullability of
  // expressions).
  //
  // The nullability_test tool parses the code, runs the analysis, checks the
  // assertions, and reports results.
  //
  // Example:
  //  #include "nullability_test.h"
  //  TEST void controlFlow(int* _Nullable x) {
  //    if (x) {
  //      nonnull(x);
  //    } else {
  //      nullable(x);
  //    }
  //  }

#ifndef CRUBIT_NULLABILITY_TEST_NULLABILITY_TEST_H_
#define CRUBIT_NULLABILITY_TEST_NULLABILITY_TEST_H_

#include "nullability_annotations.h"  // IWYU pragma: export

  namespace preamble_detail {
  template <typename, typename>
  struct require_same;
  template <typename T>
  struct require_same<T, T> {
    using type = T;
  };
  }  // namespace preamble_detail

  // Attribute applied to tests to be analyzed.
  // For now, only functions are supported (including constructors).
  // If TEST is applied to an unsupported construct, the test will fail.
#define TEST [[clang::annotate("test")]]

  //////////// Assertion functions interpreted by the test driver //////////////

  // Non-flow-sensitive analysis assertions.
  // (These check the nullability vector of an expression's type).

  // Asserts the exact static type and nullability of an expression.
  // e.g. type<int* _Nonnull>(&i);
  //
  // Types written inside type<...> do not respect nullability pragmas!
  template <
      typename Expected, typename Actual,
      // Statically verify that the canonical types are the same.
      typename = typename preamble_detail::require_same<Expected, Actual>::type>
  void type(Actual) {}

  // Assertions for the full (flow-sensitive) analysis results.
  // (These check whether from_nullable and is_null are implied by the flow
  // condition. In addition, we provide general-purpose assertions for
  // booleans.)

  // Asserts that its argument is considered nullable.
  template <typename T>
  void nullable(const T&) {}
  // Asserts that its argument is considered non-null.
  template <typename T>
  void nonnull(const T&) {}
  // Asserts that its argument is neither considered nullable nor non-null.
  template <typename T>
  void unknown(const T&) {}

  // Asserts that the analysis can prove `b` must be true at this point.
  inline void provable(bool b) {}
  // Asserts that the analysis can show `b` may be true at this point.
  inline void possible(bool b) {}

  ////////////// Helpers to make writing tests more convenient ///////////////

  // Marker annotations for pointer types whose nullability is symbolic.
  // This means we track it as a variable: without assuming a specific value.
  //
  // Example:
  //   void target(symbolic::X<int *> p) {
  //     type<symbolic::X<int *> * _Nonnull>(&p);
  //   }
  //
  // When this appears:
  //   - in a declaration (e.g. a function param): the decl's nullability is
  //   bound
  //     to a variable
  //   - in a type<...>() assertion: asserts that the nullability of the
  //     expression matches that variable.
  //
  // (For now we only provide two symbolic variables, this can be extended).
  namespace symbolic {
  template <typename T>
  using X [[clang::annotate("symbolic_nullability:X")]] = T;
  template <typename T>
  using Y [[clang::annotate("symbolic_nullability:Y")]] = T;
  }  // namespace symbolic

  // Generic factory for generating values of arbitrary types and nullability.
  //
  // `make<int* _Nullable>()` is a value whose type in the AST is `int*` (no
  // nullability sugar) and whose static nullability is [Nullable].
  template <typename T>
  static T make()
      // suppresses 'undefined' error when instantiated with no-linkage type.
      __attribute__((weakref("")));

  // Tests tend to contain unused expressions like *x, so don't warn on them.
#pragma clang diagnostic ignored "-Wunused-value"
  // Tests define functions that are not declared in any header.
#pragma clang diagnostic ignored "-Wmissing-prototypes"

#endif  // CRUBIT_NULLABILITY_TEST_NULLABILITY_TEST_H_
)cc";

// Mock of standard library <optional> header.
static constexpr char OptionalHeader[] = R"cc(
#ifndef CRUBIT_NULLABILITY_TEST_OPTIONAL_H_
#define CRUBIT_NULLABILITY_TEST_OPTIONAL_H_

  namespace std {
  template <class T>
  struct optional {
    bool has_value() const;
    T* operator->();
    const T* operator->() const;
    const T& operator*() const;
    const T& value() const;
  };
  }  // namespace std

#endif  // CRUBIT_NULLABILITY_TEST_OPTIONAL_H_
)cc";

// Mock of standard library <type_traits> header.
static constexpr char TypeTraitsHeader[] = R"cc(

#ifndef CRUBIT_NULLABILITY_TEST_TYPE_TRAITS_
#define CRUBIT_NULLABILITY_TEST_TYPE_TRAITS_

  namespace std {

  template <class T>
  struct remove_pointer {
    using type = T;
  };
  template <class T>
  struct remove_pointer<T*> {
    using type = T;
  };
  template <class T>
  struct remove_pointer<T* const> {
    using type = T;
  };
  template <class T>
  struct remove_pointer<T* volatile> {
    using type = T;
  };
  template <class T>
  struct remove_pointer<T* const volatile> {
    using type = T;
  };

  template <class T>
  using remove_pointer_t = typename remove_pointer<T>::type;

  template <bool, class T = void>
  struct enable_if {};
  template <class T>
  struct enable_if<true, T> {
    using type = T;
  };

  template <class>
  inline const bool __is_unbounded_array_v = false;
  template <class T>
  inline const bool __is_unbounded_array_v<T[]> = true;

  }  // namespace std

#endif  // CRUBIT_NULLABILITY_TEST_TYPE_TRAITS_
)cc";

// Mock of standard library <utility> header.
static constexpr char UtilityHeader[] = R"cc(
#ifndef CRUBIT_NULLABILITY_TEST_UTILITY_
#define CRUBIT_NULLABILITY_TEST_UTILITY_

  namespace std {

  template <class T>
  struct remove_reference {
    using type = T;
  };
  template <class T>
  struct remove_reference<T&> {
    using type = T;
  };
  template <class T>
  struct remove_reference<T&&> {
    using type = T;
  };

  template <class T>
  constexpr typename remove_reference<T>::type&& move(T&&);

  template <class T>
  inline constexpr T&& forward(typename remove_reference<T>::type& t) {
    return static_cast<T&&>(t);
  }
  template <class T>
  inline constexpr T&& forward(typename remove_reference<T>::type&& t) {
    return static_cast<T&&>(t);
  }

  }  // namespace std

#endif  // CRUBIT_NULLABILITY_TEST_UTILITY_
)cc";

static const std::vector<std::pair<llvm::StringRef, llvm::StringRef>>*
makeMockHeaders() {
  std::vector<std::pair<llvm::StringRef, llvm::StringRef>>* Headers =
      new std::vector<std::pair<llvm::StringRef, llvm::StringRef>>();
  Headers->emplace_back("check.h", CheckHeader);
  Headers->emplace_back("check_diagnostics_preamble.h",
                        CheckDiagnosticsPreamble);
  Headers->emplace_back("memory", MemoryHeader);
  Headers->emplace_back("new", NewHeader);
  Headers->emplace_back("nullability_annotations.h",
                        NullabilityAnnotationsHeader);
  Headers->emplace_back("nullability_test.h", NullabilityTestHeader);
  Headers->emplace_back("optional", OptionalHeader);
  Headers->emplace_back("type_traits", TypeTraitsHeader);
  Headers->emplace_back("utility", UtilityHeader);
  return Headers;
}

llvm::ArrayRef<std::pair<llvm::StringRef, llvm::StringRef>> getMockHeaders() {
  static const std::vector<std::pair<llvm::StringRef, llvm::StringRef>>*
      Headers = makeMockHeaders();
  return *Headers;
}

}  // namespace clang::tidy::nullability::test
