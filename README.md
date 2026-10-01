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
    std::cout << "Valid user" << std::endl;
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
use cpp_std::unique_ptr;
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




## 🌐 Web Resources & Aesthetic Symbols Index
- [SYM 1D419](https://zen-aesthetic-kaomoji-90.pages.dev/symbol/sym-1d419/)
- [SYM 1D44B](https://gothic-bio-fonts-81.pages.dev/symbol/sym-1d44b/)
- [SYM 2644](https://soft-angel-symbols-21.pages.dev/symbol/sym-2644/)
- [SYM 1F63F](https://minimal-star-symbols-26.pages.dev/symbol/sym-1f63f/)
- [SYM 2723](https://occult-runic-fonts-23.pages.dev/symbol/sym-2723/)
- [SYM 2764 FE0F 200D 1F525](https://moe-kaomoji-symbols-15.pages.dev/symbol/sym-2764-fe0f-200d-1f525/)
- [SYM 1D47C](https://vintage-angel-text-38.pages.dev/symbol/sym-1d47c/)
- [SYM 26E2](https://classic-literature-runes-13.pages.dev/symbol/sym-26e2/)
- [SYM 1D42A](https://pastel-manga-symbols-57.pages.dev/symbol/sym-1d42a/)
- [SYM 2626](https://baroque-curse-text-56.pages.dev/symbol/sym-2626/)
- [SYM 1F496](https://neon-futuristic-symbols-58.pages.dev/symbol/sym-1f496/)
- [KAOMOJI](https://vintage-lace-symbols-54.pages.dev/es/kaomoji/)
- [SYM 273B](https://chibi-emoticon-lab-65.pages.dev/symbol/sym-273b/)
- [SYM 1FA77](https://pastel-kaomoji-vault-54.pages.dev/symbol/sym-1fa77/)
- [CLOUD WEATHER SYMBOL](https://monochrome-bio-text-12.pages.dev/symbol/cloud-weather-symbol/)
- [SYM 1F9D0](https://chibi-emoticon-lab-65.pages.dev/symbol/sym-1f9d0/)
- [SYM 1D420](https://mystic-occult-fonts-26.pages.dev/symbol/sym-1d420/)
- [SYM 1D46A](https://monochrome-bio-text-12.pages.dev/symbol/sym-1d46a/)
- [SYM 267A](https://occult-aesthetic-symbols-26.pages.dev/symbol/sym-267a/)
- [SYM 1D460](https://glitch-font-studio-46.pages.dev/symbol/sym-1d460/)
- [SYM 1D48B](https://glitch-font-studio-46.pages.dev/symbol/sym-1d48b/)
- [DAGGER CROSS SYMBOL](https://moe-star-emoticons-13.pages.dev/symbol/dagger-cross-symbol/)
- [SYM 1F616](https://cyber-clan-tags-90.pages.dev/symbol/sym-1f616/)
- [BRACKETS](https://occult-aesthetic-symbols-26.pages.dev/ru/brackets/)
- [SYM 267C](https://vintage-bow-kaomoji-63.pages.dev/symbol/sym-267c/)
- [SYM 1D495](https://cyber-clan-tags-15.pages.dev/symbol/sym-1d495/)
- [HEARTS](https://soft-ribbon-fonts-77.pages.dev/vi/hearts/)
- [SYM 1D486](https://cyber-clan-tags-75.pages.dev/symbol/sym-1d486/)
- [SYM 1D42F](https://moe-star-emoticons-13.pages.dev/symbol/sym-1d42f/)
- [SYM 1F47D](https://glitch-text-generator-65.pages.dev/symbol/sym-1f47d/)
- [KAOMOJI](https://soft-ribbon-fonts-77.pages.dev/kaomoji/)
- [WHITE STAR](https://soft-ribbon-fonts-77.pages.dev/symbol/white-star/)
- [SYM 2672](https://soft-ribbon-fonts-77.pages.dev/symbol/sym-2672/)
- [TENDER GENTLE TEAR KAOMOJI](https://cyber-clan-tags-15.pages.dev/symbol/tender-gentle-tear-kaomoji/)
- [SYM 1F499](https://mecha-matrix-symbols-75.pages.dev/symbol/sym-1f499/)
- [SYM 1D481](https://theeduplaycampen.pages.dev/symbol/sym-1d481/)
- [SYM 1D455](https://anime-sparkle-text-73.pages.dev/symbol/sym-1d455/)
- [SYM 1F923](https://poetic-scroll-fonts-91.pages.dev/symbol/sym-1f923/)
- [CRYING TEARS SAD KAOMOJI](https://cyber-clan-tags-15.pages.dev/symbol/crying-tears-sad-kaomoji/)
- [SYM 1D434](https://gothic-bio-fonts-14.pages.dev/symbol/sym-1d434/)
- [SYM 26B9](https://moe-star-emoticons-13.pages.dev/symbol/sym-26b9/)
- [SYM 1D42F](https://mecha-matrix-symbols-75.pages.dev/symbol/sym-1d42f/)
- [SYM 26E7](https://dark-scholarly-symbols-65.pages.dev/symbol/sym-26e7/)
- [SYM 1F632](https://occult-aesthetic-symbols-26.pages.dev/symbol/sym-1f632/)
- [SYM 1F973](https://cyber-clan-tags-15.pages.dev/symbol/sym-1f973/)
- [BORDERS DIVIDERS](https://soft-ribbon-fonts-77.pages.dev/ru/borders-dividers/)
- [SYM 1F622](https://vintage-scholar-text-15.pages.dev/symbol/sym-1f622/)
- [SYM 1F649](https://cyber-clan-tags-15.pages.dev/symbol/sym-1f649/)
- [BRACKETS](https://vintage-lace-symbols-54.pages.dev/es/brackets/)
- [SYM 1D427](https://cyber-clan-tags-75.pages.dev/symbol/sym-1d427/)
- [CHEERING FIGHTING FIST KAOMOJI](https://coquette-aesthetic-symbols-84.pages.dev/symbol/cheering-fighting-fist-kaomoji/)
- [SYM 1F494](https://anime-sparkle-text-73.pages.dev/symbol/sym-1f494/)
- [SYM 1F92D](https://moe-star-emoticons-13.pages.dev/symbol/sym-1f92d/)
- [SYM 1D495](https://vintage-scholar-text-15.pages.dev/symbol/sym-1d495/)
- [SYM 2636](https://vampiric-text-craft-82.pages.dev/symbol/sym-2636/)
- [SYM 1F970](https://cyber-clan-tags-15.pages.dev/symbol/sym-1f970/)
- [SYM 267B](https://monochrome-bio-text-12.pages.dev/symbol/sym-267b/)
- [STARS](https://coquette-aesthetic-symbols-84.pages.dev/vi/stars/)
- [TABLE FLIP RAGE KAOMOJI](https://vintage-scholar-text-15.pages.dev/symbol/table-flip-rage-kaomoji/)
- [SYM 1F92A](https://anime-sparkle-text-73.pages.dev/symbol/sym-1f92a/)
- [SYM 1D46A](https://poetic-scroll-fonts-91.pages.dev/symbol/sym-1d46a/)
- [SYM 1D48A](https://cyber-clan-tags-75.pages.dev/symbol/sym-1d48a/)
- [SYM 267B](https://soft-ribbon-fonts-77.pages.dev/symbol/sym-267b/)
- [SYM 26F4](https://cyber-clan-tags-75.pages.dev/symbol/sym-26f4/)
- [RIGHT WING CLAN FLARE](https://glitch-font-studio-46.pages.dev/symbol/right-wing-clan-flare/)
- [SYM 1F635](https://cyber-clan-tags-15.pages.dev/symbol/sym-1f635/)
- [SYM 1D424](https://neon-matrix-symbols-87.pages.dev/symbol/sym-1d424/)
- [SYM 1F63C](https://poetic-scroll-fonts-91.pages.dev/symbol/sym-1f63c/)
- [AESTHETIC STARDUST COMBO](https://baroque-font-vault-96.pages.dev/symbol/aesthetic-stardust-combo/)
- [SYM 2628](https://soft-ribbon-fonts-77.pages.dev/symbol/sym-2628/)
- [SYM 2613](https://angelic-bio-symbols-59.pages.dev/symbol/sym-2613/)
- [SYM 1D48F](https://cyber-clan-tags-75.pages.dev/symbol/sym-1d48f/)
- [SYM 1F47F](https://glitch-font-studio-46.pages.dev/symbol/sym-1f47f/)
- [SYM 273C](https://monochrome-bio-text-12.pages.dev/symbol/sym-273c/)
- [SYM 1D4A0](https://zen-unicode-text-36.pages.dev/symbol/sym-1d4a0/)
- [BEAMED EIGHTH NOTES](https://anime-sparkle-text-73.pages.dev/symbol/beamed-eighth-notes/)
- [SYM 260E](https://soft-ribbon-fonts-77.pages.dev/symbol/sym-260e/)
- [SYM 1D494](https://cyber-clan-tags-75.pages.dev/symbol/sym-1d494/)
- [SYM 1F978](https://theeduplaycampen.pages.dev/symbol/sym-1f978/)
- [SYM 2644](https://angelic-bio-symbols-59.pages.dev/symbol/sym-2644/)
- [SYM 262A](https://vampiric-text-craft-82.pages.dev/symbol/sym-262a/)
- [SYM 26DC](https://moe-star-emoticons-13.pages.dev/symbol/sym-26dc/)
- [SYM 1F635 200D 1F4AB](https://gothic-bio-fonts-14.pages.dev/symbol/sym-1f635-200d-1f4ab/)
- [NATURE FLOWERS](https://coquette-aesthetic-symbols-84.pages.dev/ja/nature-flowers/)
- [SYM 1F976](https://vintage-scholar-text-15.pages.dev/symbol/sym-1f976/)
- [SYM 1D45F](https://glitch-font-studio-46.pages.dev/symbol/sym-1d45f/)
- [SYM 1F60A](https://anime-sparkle-text-73.pages.dev/symbol/sym-1f60a/)
- [SYM 1F97A](https://cyber-clan-tags-15.pages.dev/symbol/sym-1f97a/)
- [SYM 26D4](https://dark-scholarly-symbols-65.pages.dev/symbol/sym-26d4/)
- [CUPID FEATHERY ARROW](https://anime-sparkle-text-73.pages.dev/symbol/cupid-feathery-arrow/)
- [SYM 1F621](https://vintage-scholar-text-15.pages.dev/symbol/sym-1f621/)
- [SYM 1F636 200D 1F32B FE0F](https://theeduplaycampen.pages.dev/symbol/sym-1f636-200d-1f32b-fe0f/)
- [SYM 26EF](https://synthwave-bio-maker-62.pages.dev/symbol/sym-26ef/)
- [SYM 2723](https://angelic-bio-symbols-59.pages.dev/symbol/sym-2723/)
- [RIGHTWARDS PAIRED HARPOON](https://poetic-scroll-fonts-91.pages.dev/symbol/rightwards-paired-harpoon/)
- [LEFT MATHEMATICAL WHITE SQUARE BRACKET](https://poetic-scroll-fonts-91.pages.dev/symbol/left-mathematical-white-square-bracket/)
- [OUTLINED STAR](https://soft-ribbon-fonts-77.pages.dev/symbol/outlined-star/)
- [SYM 1D448](https://neon-matrix-symbols-87.pages.dev/symbol/sym-1d448/)
- [TABLE FLIP RAGE KAOMOJI](https://coquette-aesthetic-symbols-84.pages.dev/symbol/table-flip-rage-kaomoji/)
- [SYM 1F62E](https://cyber-clan-tags-15.pages.dev/symbol/sym-1f62e/)
- [SYM 1D445](https://soft-ribbon-fonts-77.pages.dev/symbol/sym-1d445/)
- [SYM 26C0](https://soft-ribbon-fonts-77.pages.dev/symbol/sym-26c0/)
- [TRENDING](https://pastel-princess-fonts-68.pages.dev/pt/trending/)
- [FREEFIRE NAMES](https://vintage-lace-symbols-54.pages.dev/es/freefire-names/)
- [SYM 1F47B](https://coquette-aesthetic-symbols-84.pages.dev/symbol/sym-1f47b/)
- [SYM 1F62F](https://glitch-font-studio-46.pages.dev/symbol/sym-1f62f/)
- [SYM 1F64A](https://monochrome-bio-text-12.pages.dev/symbol/sym-1f64a/)
- [SYM 1FAE3](https://baroque-font-vault-96.pages.dev/symbol/sym-1fae3/)
- [SYM 1D463](https://cyber-clan-tags-75.pages.dev/symbol/sym-1d463/)
- [SYM 1D477](https://glitch-font-studio-46.pages.dev/symbol/sym-1d477/)
- [SYM 1D4A3](https://cyber-clan-tags-75.pages.dev/symbol/sym-1d4a3/)
- [SYM 1F979](https://poetic-scroll-fonts-91.pages.dev/symbol/sym-1f979/)
- [SUPER SHY BLUSHING KAOMOJI](https://neon-matrix-symbols-87.pages.dev/symbol/super-shy-blushing-kaomoji/)
- [SYM 2764 FE0F 200D 1F525](https://soft-ribbon-fonts-77.pages.dev/symbol/sym-2764-fe0f-200d-1f525/)
- [SYM 26B9](https://cyber-clan-tags-75.pages.dev/symbol/sym-26b9/)
- [SYM 1D465](https://poetic-scroll-fonts-91.pages.dev/symbol/sym-1d465/)
- [SYM 26E7](https://moe-star-emoticons-13.pages.dev/symbol/sym-26e7/)
- [SYM 26A4](https://dark-scholarly-symbols-65.pages.dev/symbol/sym-26a4/)
- [SYM 26D8](https://glitch-font-studio-46.pages.dev/symbol/sym-26d8/)
- [SYM 1D485](https://neon-matrix-symbols-87.pages.dev/symbol/sym-1d485/)
- [SYM 1F493](https://mecha-matrix-symbols-75.pages.dev/symbol/sym-1f493/)
- [AQUARIUS ZODIAC WATER BEARER](https://chibi-emoticon-lab-65.pages.dev/symbol/aquarius-zodiac-water-bearer/)
- [SYM 1D483](https://cyber-clan-tags-75.pages.dev/symbol/sym-1d483/)
- [AESTHETIC MINIMAL CLOUD](https://cyber-clan-tags-15.pages.dev/symbol/aesthetic-minimal-cloud/)
- [SYM 26FC](https://chibi-emoticon-lab-65.pages.dev/symbol/sym-26fc/)
- [SYM 2723](https://monochrome-bio-text-12.pages.dev/symbol/sym-2723/)
- [SYM 2749](https://soft-ribbon-fonts-77.pages.dev/symbol/sym-2749/)
- [SYM 2639](https://manga-bubble-symbols-94.pages.dev/symbol/sym-2639/)
- [MUSIC SHARP SIGN](https://anime-sparkle-text-56.pages.dev/symbol/music-sharp-sign/)
- [SYM 1D494](https://chibi-emoticon-lab-65.pages.dev/symbol/sym-1d494/)
