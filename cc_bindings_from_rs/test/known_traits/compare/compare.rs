// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// `None` is treated as `NaN`.
pub struct MyPartialOrd(pub Option<i32>);

impl PartialEq for MyPartialOrd {
    fn eq(&self, other: &Self) -> bool {
        let (Some(a), Some(b)) = (self.0, other.0) else {
            return false;
        };
        a == b
    }
}

impl PartialOrd for MyPartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0?.partial_cmp(&other.0?)
    }
}

pub struct MyOrd(pub i32);

impl PartialEq for MyOrd {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for MyOrd {}

impl PartialOrd for MyOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MyOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

pub struct MyUnordered(pub f32);

impl PartialEq for MyUnordered {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for MyUnordered {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
