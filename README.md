# Crubit: C++/Rust Bidirectional Interop Tool

[![rust workflow](https://github.com/google/crubit/actions/workflows/rust.yml/badge.svg)](https://github.com/google/crubit/actions/workflows/rust.yml)
[![GitHub](https://img.shields.io/badge/github-google%2Fcrubit-blue?logo=github)](https://github.com/google/crubit)

Crubit is a bidirectional bindings generator for C++ and Rust, with the goal of
integrating the C++ and Rust ecosystems.

## Status

See the [status](http://crubit.rs/overview/status) page for an overview of the
current supported features.

## Example

{{#tabs}}

{{#tab name="Calling Rust from C++"}}

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
    std::cout << "Valid user\n";
  }
}
```

{{#endtab}}

{{#tab name="Calling C++ from Rust"}}

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
use cc_std::std::unique_ptr;
use ffi_11::{c_double, c_int};
use user_api::{CreateUser, FindUser, User};

let id: c_int = 123;
let user: Option<User> = FindUser(id);
if let Some(u) = user {
    let balance: c_double = u.balance;
    println!("User {} has balance {}", u.id, balance);
}

let new_user: unique_ptr<User> = CreateUser("Alice".into(), 456);
```

{{#endtab}}

{{#endtabs}}

## Getting Started

We have detailed walkthroughs on how to use C++ from Rust, or Rust from C++,
using Crubit, as well as copy-pastable example code. The example code also
includes spanshots of what the generated bindings look like.

*   Walkthrough: [C++ Bindings for Rust Libraries](http://crubit.rs/rust)
    *   Examples:
        [`examples/rust/`](https://github.com/google/crubit/tree/main/examples/rust)
*   Walkthrough: [Rust Bindings for C++ Libraries](http://crubit.rs/cpp)
    *   Examples:
        [`examples/cpp/`](https://github.com/google/crubit/tree/main/examples/cpp)


