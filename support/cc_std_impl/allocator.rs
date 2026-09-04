// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
// #![feature(allocator_api)]

use crate::crubit_cc_std_internal::std_allocator::{
    cpp_delete, cpp_new, DynControlBlock, FunctionToCall,
};
use core::alloc::AllocError;
use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::ffi::c_void;
use core::ptr::{self, NonNull};

/// An allocator that uses C++ `new` and `delete` to allocate and deallocate memory.
///
/// This allocator should be used when allocating memory that will be deallocated by C++,
/// or when deallocating memory allocated by C++.
#[derive(Copy, Clone, Debug, Default)]
pub struct Allocator;

unsafe impl core::alloc::Allocator for Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // SAFETY: Allocator allows for zero-sized allocations.
        let raw_ptr = unsafe { self.alloc(layout) };
        let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // SAFETY: Allocator and Dealloc have identical preconditions.
        unsafe {
            self.dealloc(ptr.as_ptr(), layout);
        }
    }

    // NOTE: Also change the GlobalAlloc impl if you add grow/etc.
}

/// `Allocator` is a global allocator which also accepts zero-sized allocations.
///
/// This allows allocations (even of size 0) in Rust to be mixed with deallocations in C++.
/// (Though, since `Global` will not pass through 0-sized allocations, this is currently of
/// limited use.)
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        cpp_new(layout.size(), layout.align()) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            cpp_delete(ptr as *mut c_void, layout.size(), layout.align());
        }
    }
}

/// Invokes a type-erased deleter function pointer on the Rust side of the FFI boundary.
///
/// TODO(b/526962187): Remove this thunk once CFI works properly. This serves as a direct-call thunk
/// so C++ `std::shared_ptr` control block hooks (`__on_zero_shared` and `__on_zero_shared_weak`)
/// can invoke the Rust deleter without performing an indirect cross-language call that would
/// trigger Clang Control Flow Integrity (`-fsanitize=cfi-icall`) violations.
///
/// # Safety
///
/// ## Preconditions
/// - `cntrl` must be a non-dangling pointer to a live `DynControlBlock`.
/// - `cntrl.deleter` must be initialized with a valid `DeleterFn` callback whose preconditions are
///   met.
/// - Must only be called at the valid lifecycle transition points of `std::shared_ptr`:
///   - `FunctionToCall::kDestroyValue` must be called when the strong reference count drops to 0
///     (to destroy the managed payload).
///   - `FunctionToCall::kDeleteControlBlock` must be called when the weak reference count drops to 0
///     (to deallocate the control block and its allocation).
///   Calling them out of order, multiple times, or at inappropriate lifecycle states violates the
///   `deleter`'s preconditions and causes undefined behavior.
///
/// ## Postconditions
/// - Forwards the lifecycle event to `cntrl.deleter`, executing the appropriate destruction or
///   deallocation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __crubit_invoke_dyn_control_block_deleter(
    function_to_call: FunctionToCall,
    cntrl: NonNull<DynControlBlock>,
) {
    let p = cntrl.as_ptr();

    // SAFETY: Caller guarantees that `cntrl` points to a live, valid `DynControlBlock`.
    // Reading `deleter` is properly aligned and dereferences valid initialized memory.
    let deleter = unsafe { ptr::read(&raw const (*p).deleter) };

    // SAFETY: Caller guarantees that `cntrl` was initialized with a non-null `deleter` function
    // pointer.
    let deleter = unsafe { deleter.unwrap_unchecked() };

    // SAFETY: Caller guarantees that `p` points to a live `DynControlBlock` and `function_to_call`
    // corresponds to the valid lifecycle event, satisfying all preconditions of `deleter`.
    unsafe { deleter(function_to_call, p) };
}
