// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use foo_rust_proto::{FooRequestStats, FooRequestStatsView, FooRequestView, FooResponseMut};

#[derive(Default)]
pub struct FooService {
    stats: FooRequestStats,
}

impl FooService {
    pub fn handle_request(&mut self, req: FooRequestView, mut rsp: FooResponseMut) -> bool {
        self.stats.set_num_requests(self.stats.num_requests() + 1);

        rsp.set_output(req.input());
        true
    }

    pub fn request_stats(&self) -> FooRequestStatsView {
        self.stats.as_view()
    }

    pub fn clone_request_stats(&self) -> FooRequestStats {
        self.stats.clone()
    }

    pub fn update_request_stats(&mut self, updated_stats: FooRequestStats) {
        self.stats = updated_stats;
    }
}
