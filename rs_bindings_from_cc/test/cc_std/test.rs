// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//TODO(rosica): We need namespaces in order to be able to test more here.

#[cfg(test)]
mod tests {
    #[test]
    fn test_return_value() {
        use cc_std::*;
        let _t = tm {
            tm_gmtoff: 0,
            tm_hour: 1,
            tm_isdst: 2,
            tm_mday: 3,
            tm_min: 4,
            tm_mon: 5,
            tm_sec: 6,
            tm_wday: 7,
            tm_yday: 8,
            tm_year: 9,
            tm_zone: "zone".as_ptr(),
        };
    }
}
