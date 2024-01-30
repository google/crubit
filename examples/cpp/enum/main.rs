// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

fn main() {
    let green = example_lib::Color::kGreen;
    match green {
        example_lib::Color::kGreen => println!("Green is green!"),
        _ => todo!(),
    }
}
