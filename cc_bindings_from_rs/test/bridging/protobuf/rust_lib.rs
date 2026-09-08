// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_annotate::must_bind;
use foo_rust_proto::{FooRequestStats, FooRequestStatsView, FooRequestView, FooResponseMut};

#[must_bind]
#[derive(Default)]
pub struct FooService {
    stats: FooRequestStats,
}

impl FooService {
    #[must_bind]
    pub fn handle_request(&mut self, req: FooRequestView, mut rsp: FooResponseMut) -> bool {
        self.stats.set_num_requests(self.stats.num_requests() + 1);

        rsp.set_output(req.input());
        true
    }

    #[must_bind]
    pub fn request_stats(&self) -> FooRequestStatsView<'_> {
        self.stats.as_view()
    }

    #[must_bind]
    pub fn clone_request_stats(&self) -> FooRequestStats {
        self.stats.clone()
    }

    #[must_bind]
    pub fn update_request_stats(&mut self, updated_stats: FooRequestStats) {
        self.stats = updated_stats;
    }

    #[must_bind]
    pub fn enum_in_signature(_e: foo_rust_proto::FooEnum) {}
}

#[must_bind]
#[derive(Default)]
pub struct StructWithProto {
    pub stats: FooRequestStats,
}

#[must_bind]
pub fn create_struct_with_proto(num: i32) -> StructWithProto {
    let mut stats = FooRequestStats::new();
    stats.set_num_requests(num);
    StructWithProto { stats }
}

/// # Safety
///
/// `p` must be valid for reads.
#[must_bind]
pub unsafe fn read_proto_pointer(p: *const FooRequestStats) -> i32 {
    // SAFETY: The caller guarantees `p` is valid for reads.
    unsafe { (*p).num_requests() }
}

#[must_bind]
pub fn read_proto_ref(p: &FooRequestStats) -> i32 {
    p.num_requests()
}

#[doc = "CRUBIT_ANNOTATE: cpp_type=absl::StatusOr<{T}>"]
#[doc = "CRUBIT_ANNOTATE: include_path=third_party/absl/status/statusor.h"]
#[repr(C)]
pub struct NewStatusOr<T> {
    status: usize,
    data: std::mem::MaybeUninit<T>,
}

impl<T> NewStatusOr<T> {
    pub fn ok(value: T) -> Self {
        Self {
            status: 1, // Inlined absl::Status::Ok() representation
            data: std::mem::MaybeUninit::new(value),
        }
    }
}

impl<T> Drop for NewStatusOr<T> {
    fn drop(&mut self) {
        if self.status == 1 {
            unsafe {
                self.data.assume_init_drop();
            }
        }
    }
}

#[must_bind]
pub fn create_proto_vec(num: i32) -> Vec<FooRequestStats> {
    let mut stats = FooRequestStats::new();
    stats.set_num_requests(num);
    vec![stats]
}

#[must_bind]
pub fn create_proto_status_or(num: i32) -> NewStatusOr<FooRequestStats> {
    let mut stats = FooRequestStats::new();
    stats.set_num_requests(num);
    NewStatusOr::ok(stats)
}
