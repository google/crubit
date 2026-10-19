# Crubit: C++/Rust Bidirectional Interop Tool

[![rust workflow](https://github.com/google/crubit/actions/workflows/rust.yml/badge.svg)](https://github.com/google/crubit/actions/workflows/rust.yml)
[![GitHub](https://img.shields.io/badge/github-google%2Fcrubit-blue?logo=github)](https://github.com/google/crubit)

Crubit is a bidirectional bindings generator for C++ and Rust, allowing users
to:

*   **Automatically reuse existing crates and headers**: no IDL or glue code is
    necessary.
*   **Call back and forth between Rust and C++ with no performance cost**: no
    copies or heap allocations required.
*   **Use idiomatic types from both Rust and C++**, such as `Vec`, `Result`,
    `Option`, `std::vector`, `std::unique_ptr`, `std::shared_ptr`, and more!

## Example

### Calling Rust from C++

Consider the following Rust library:

```rust
pub struct Account {
    pub id: u64,
    pub balance: f64,
}

impl std::fmt::Display for Account { ... }

// Takes shared references, mapped to const references in C++.
pub fn calculate_interest(account: &Account, rate: f64) -> f64 { ... }

// Takes a string slice, mapped to rs_std::StrRef in C++.
pub fn is_valid_username(username: &str) -> bool { ... }
```

You can call these Rust functions from C++:

```c++
#include "path/to/account.h"
#include <iostream>

void demo() {
  account::Account my_account{.id = 123, .balance = 1000.0};
  double interest = account::calculate_interest(my_account, 0.05);

  // my_account is printable because Account implements Display in Rust!
  std::cout << my_account << std::endl;

  if (account::is_valid_username("bob")) {
    std::cout << "Valid user" << std::endl;
  }
}
```

### Calling C++ from Rust

Consider the following C++ header:

```c++
#include <string_view>
#include <optional>
#include <memory>

struct User {
  int id;
  double balance;
};

std::optional<User> FindUser(int id);
std::unique_ptr<User> CreateUser(std::string_view name, int id);
```

You can call these C++ functions from Rust:

```rs
use cpp_std::unique_ptr;
use ffi_11::c_int;
use user_api::{CreateUser, FindUser, User};

let id: c_int = 123;
let user: Option<User> = FindUser(id);
if let Some(u) = user {
    let balance: f64 = u.balance;
    println!("User {} has balance {}", u.id, balance);
}

let new_user: unique_ptr<User> = CreateUser("Alice".into(), 456);
```

## Why Crubit?

*   **No hand-written glue code.** Crubit reads C++ headers and Rust crates
    directly and generates bindings in both directions. Your signatures can't
    drift out of sync with the code they wrap, and you don't need to maintain a
    separate interface file.
*   **More than the C ABI.** Crubit can call functions that aren't
    `extern "C"`, pass structs and enums by value, and match the memory layout
    of Rust structs and enums that aren't `#[repr(C)]`.
*   **Idiomatic types on both sides.** References, slices, `&str`, `Option`,
    `Result`, tuples, smart pointers, `std::vector`/`Vec`, callbacks,
    `absl::Status`, and more map to their natural counterparts. See the
    [status](http://crubit.rs/overview/status) page for the full table.
*   **Traits and operators, too.** C++ can call Rust trait implementations,
    and Rust traits such as `Display`, `Clone`, and `Default` become the
    matching C++ operators and special member functions. In the other
    direction, C++ operator overloads and special member functions become the
    matching Rust traits, such as `PartialEq`, `Add`, `Clone`, and `Drop`.
*   **Safer than hand-written FFI.** Crubit generates bindings from the real
    C++ headers and Rust crates at build time, with the same compiler and
    flags, so mismatched ABIs, struct layouts, and signatures can't creep in.
    C++ references get Rust lifetimes through lifetime elision and
    [lifetime annotations](http://crubit.rs/cpp/references_and_lifetimes), so
    the Rust borrow checker can check them. Slices, strings, `Option`, and
    other [types](http://crubit.rs/types) carry their bounds and invariants
    across the boundary instead of degrading to raw pointers. See
    [Memory Safety](http://crubit.rs/overview/safety) for the details.

## Getting Started

### 1. Set up your build

Crubit runs as part of your build, so that the bindings always match the code
they wrap. Pick your build system:

Build system                                    | C++ calling Rust (`cpp_api_from_rust`)                                                                                                       | Rust calling C++ (`rust_api_from_cpp`)
:---------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------
[Bazel](http://crubit.rs/overview/bazel)       | [Supported](http://crubit.rs/overview/bazel#cpp_api_from_rust) ([example](https://github.com/google/crubit/tree/main/examples/build_systems/bazel)) | [Supported](http://crubit.rs/overview/bazel#rust_api_from_cpp)
[CMake](http://crubit.rs/overview/cmake)       | [Supported](http://crubit.rs/overview/cmake) ([example](https://github.com/google/crubit/tree/main/examples/build_systems/cmake))                   | Not yet
[Other](http://crubit.rs/overview/build_systems#other-build-systems) | Through the `cargo cpp_api_from_rust` subcommand                                                                       | Not yet

Crubit works with nightly Rust toolchains and with the two most recent stable
releases, as long as the `rustc-dev` component is installed (see the
[test matrix](#test-matrix) below). There are no binary releases yet; pin a
commit of this repository.
[Build System Integrations](http://crubit.rs/overview/build_systems) covers
toolchain requirements and how to set up each build system.

### 2. Use your Rust and C++ APIs together

Add a `cpp_api_from_rust` or `rust_api_from_cpp` target next to your library,
and start calling it from the other language. Each direction has a walkthrough
that explains what the generated API looks like and how to customize it, plus
copy-pastable examples that include snapshots of the generated bindings:

*   **C++ calling Rust:**
    [C++ Bindings for Rust Libraries](http://crubit.rs/rust) walkthrough and
    [`examples/rust/`](https://github.com/google/crubit/tree/main/examples/rust).
*   **Rust calling C++:**
    [Rust Bindings for C++ Libraries](http://crubit.rs/cpp) walkthrough and
    [`examples/cpp/`](https://github.com/google/crubit/tree/main/examples/cpp).

## Test Matrix

|Nightly|Stable|
| ---   | ---  |
| [![nightly-0](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/google/crubit/test-matrix/nightly-0.json)](https://github.com/google/crubit/actions/workflows/nightly.yaml) | [![stable-0](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/google/crubit/test-matrix/stable-0.json)](https://github.com/google/crubit/actions/workflows/nightly.yaml) |
| [![nightly-1](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/google/crubit/test-matrix/nightly-1.json)](https://github.com/google/crubit/actions/workflows/nightly.yaml) | [![stable-1](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/google/crubit/test-matrix/stable-1.json)](https://github.com/google/crubit/actions/workflows/nightly.yaml) |
| [![nightly-2](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/google/crubit/test-matrix/nightly-2.json)](https://github.com/google/crubit/actions/workflows/nightly.yaml) | |
| [![nightly-3](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/google/crubit/test-matrix/nightly-3.json)](https://github.com/google/crubit/actions/workflows/nightly.yaml) | |
| [![nightly-4](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/google/crubit/test-matrix/nightly-4.json)](https://github.com/google/crubit/actions/workflows/nightly.yaml) | |
| [![nightly-5](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/google/crubit/test-matrix/nightly-5.json)](https://github.com/google/crubit/actions/workflows/nightly.yaml) | |
| [![nightly-6](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/google/crubit/test-matrix/nightly-6.json)](https://github.com/google/crubit/actions/workflows/nightly.yaml) | |


