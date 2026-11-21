// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![cfg(test)]
#![feature(negative_impls)]

use cc_helper_functions::test_variant::{
    modify_trivial_variant, pass_trivial_variant_by_value, return_coproduct17_double,
    return_destructor_variant, return_duplicate_variant_0, return_duplicate_variant_1,
    return_non_trivial_not_unpin_variant_string, return_non_trivial_unpin_variant_int,
    return_small_variant_bool, return_trivial_variant_bool, return_trivial_variant_float,
    return_trivial_variant_int, take_coproduct17_by_value,
    take_non_trivial_not_unpin_variant_by_const_ref, take_non_trivial_unpin_variant_by_const_ref,
    take_non_trivial_unpin_variant_by_value, take_small_variant_by_value,
    take_trivial_variant_by_const_ref, take_trivial_variant_by_value, DuplicateVariant,
    NonTrivialNotUnpinVariant, NonTrivialUnpinVariant, SmallVariant, TrivialVariant,
};
use cpp_std::variant_enums::{Variant2, Variant3};
use ctor::CtorNew;
use googletest::{expect_eq, expect_false, expect_ne, expect_true, gtest};

#[gtest]
fn test_trivial_variant_int() {
    let v = return_trivial_variant_int(42);
    expect_eq!(v.get_at::<0>().copied(), Some(42));
    expect_eq!(v.get::<i32, _>().copied(), Some(42));
    expect_true!(v.holds_alternative::<i32, _>());
    expect_false!(v.holds_alternative::<f32, _>());
    expect_eq!(v.as_ref(), Variant3::V0(&42));
    expect_eq!(v.into_enum(), Variant3::V0(42));
    expect_eq!(Variant3::from(v), Variant3::V0(42));
    expect_eq!(take_trivial_variant_by_value(v), 42);
    expect_eq!(take_trivial_variant_by_const_ref(&v), 42);
    expect_eq!(pass_trivial_variant_by_value(v).get::<i32, _>().copied(), Some(42));
}

#[gtest]
fn test_trivial_variant_float() {
    let v = return_trivial_variant_float(3.5);
    expect_eq!(v.get_at::<1>().copied(), Some(3.5));
    expect_eq!(v.get::<f32, _>().copied(), Some(3.5));
    expect_true!(v.holds_alternative::<f32, _>());
    expect_eq!(v.as_ref(), Variant3::V1(&3.5));
    expect_eq!(take_trivial_variant_by_value(v), 3);
    expect_eq!(take_trivial_variant_by_const_ref(&v), 3);
}

#[gtest]
fn test_trivial_variant_bool() {
    let v = return_trivial_variant_bool(true);
    expect_eq!(v.get_at::<2>().copied(), Some(true));
    expect_eq!(v.get::<bool, _>().copied(), Some(true));
    expect_true!(v.holds_alternative::<bool, _>());
    expect_eq!(v.as_ref(), Variant3::V2(&true));
    expect_eq!(take_trivial_variant_by_value(v), 1);
    expect_eq!(take_trivial_variant_by_const_ref(&v), 1);
}

#[gtest]
fn test_trivial_variant_construction_and_traits() {
    let v_default = TrivialVariant::default();
    expect_eq!(v_default.get::<i32, _>().copied(), Some(0));

    let v_new: TrivialVariant = TrivialVariant::new(true);
    expect_eq!(v_new.get::<bool, _>().copied(), Some(true));

    let v_from_enum: TrivialVariant = Variant3::V1(2.5f32).into();
    expect_eq!(v_from_enum.get::<f32, _>().copied(), Some(2.5));
    expect_eq!(v_from_enum, TrivialVariant::new(2.5f32));
    expect_ne!(v_from_enum, v_default);

    let non_trivial: cpp_std::variant<(i32, f32, bool)> = v_from_enum.into();
    expect_eq!(non_trivial.get::<f32, _>().copied(), Some(2.5));
    let back_to_trivial: TrivialVariant = non_trivial.into();
    expect_eq!(back_to_trivial, v_from_enum);
}

#[gtest]
fn test_trivial_variant_modify() {
    let mut v = return_trivial_variant_int(42);
    modify_trivial_variant(&mut v);
    expect_eq!(v.get::<i32, _>().copied(), Some(43));
    if let Variant3::V0(x) = v.as_mut() {
        *x = 100;
    }
    expect_eq!(v.get::<i32, _>().copied(), Some(100));
}

#[gtest]
fn test_non_trivial_unpin_variant() {
    let v = return_non_trivial_unpin_variant_int(42);
    expect_eq!(v.get::<i32, _>().copied(), Some(42));
    expect_eq!(take_non_trivial_unpin_variant_by_const_ref(&v), 42);
    expect_eq!(take_non_trivial_unpin_variant_by_value(v), 42);

    let mut vec = cpp_std::vector::<i32>::new();
    vec.push(10);
    vec.push(20);
    let v_vec = NonTrivialUnpinVariant::new(vec.clone());
    expect_eq!(take_non_trivial_unpin_variant_by_const_ref(&v_vec), 30);
    expect_eq!(take_non_trivial_unpin_variant_by_value(v_vec), 30);

    let v_cloneable: cpp_std::variant<(i32, cpp_std::vector<i32>)> = cpp_std::variant::new(vec);
    let v_cloned = v_cloneable.clone();
    expect_eq!(v_cloneable, v_cloned);
    expect_eq!(v_cloned.into_enum(), Variant2::V1(cpp_std::vector::from([10, 20])));
}

#[gtest]
fn test_non_trivial_not_unpin_variant_string() {
    let mut v = ctor::emplace!(return_non_trivial_not_unpin_variant_string());
    expect_eq!(take_non_trivial_not_unpin_variant_by_const_ref(&v), 5);
    expect_true!(v.holds_alternative::<cpp_std::string, _>());

    v.as_mut().emplace(99i32);
    expect_eq!(take_non_trivial_not_unpin_variant_by_const_ref(&v), 99);

    let v_default = ctor::emplace!(NonTrivialNotUnpinVariant::default());
    expect_eq!(take_non_trivial_not_unpin_variant_by_const_ref(&v_default), 0);

    let v_ctor_at = ctor::emplace!({
        NonTrivialNotUnpinVariant::ctor_new_at::<1>(cpp_std::string::ctor_new("crubit"))
    });
    expect_eq!(take_non_trivial_not_unpin_variant_by_const_ref(&v_ctor_at), 6);

    let v_ctor =
        ctor::emplace!({ NonTrivialNotUnpinVariant::ctor_new(cpp_std::string::ctor_new("rust")) });
    expect_eq!(take_non_trivial_not_unpin_variant_by_const_ref(&v_ctor), 4);
}

#[gtest]
fn test_destructor_variant() {
    let mut count = 0;
    {
        let _v = ctor::emplace!(unsafe { return_destructor_variant(&mut count) });
    }
    // _v is dropped here, so the destructor increments count.
    expect_eq!(count, 1);
}

#[gtest]
fn test_coproduct17() {
    let v = return_coproduct17_double(3.5);
    expect_eq!(v.get_at::<16>().copied(), Some(3.5));
    expect_eq!(v.get::<f64, _>().copied(), Some(3.5));
    expect_eq!(take_coproduct17_by_value(v), 3.5);
}

#[gtest]
fn test_coproduct_type_list_directly() {
    type MyCoprod = cpp_std::coproduct::Coproduct<
        i32,
        cpp_std::coproduct::Coproduct<f32, cpp_std::coproduct::CNil>,
    >;
    let v: cpp_std::trivial_variant<MyCoprod> = cpp_std::trivial_variant::new(1.5f32);
    expect_eq!(v.index(), 1);
    expect_eq!(v.get::<f32, _>().copied(), Some(1.5f32));
    expect_eq!(
        v.into_enum(),
        cpp_std::coproduct::Coproduct::Inr(cpp_std::coproduct::Coproduct::Inl(1.5f32))
    );
}

#[gtest]
fn test_duplicate_variant() {
    let v0 = return_duplicate_variant_0(42);
    expect_eq!(v0.get_at::<0>().copied(), Some(42));
    expect_eq!(v0.get_at::<1>().copied(), None);

    let v1 = return_duplicate_variant_1(42);
    expect_eq!(v1.get_at::<1>().copied(), Some(42));
    expect_eq!(v1.get_at::<0>().copied(), None);

    let v_new_0 = DuplicateVariant::new_at::<0>(100);
    expect_true!(v_new_0.holds_alternative_at::<0>());
    let v_new_1 = DuplicateVariant::new_at::<1>(100);
    expect_true!(v_new_1.holds_alternative_at::<1>());
}

struct FallibleCtor;
unsafe impl ctor::Ctor for FallibleCtor {
    type Output = core::ffi::c_int;
    type Error = ();
    unsafe fn ctor(self, _dest: *mut Self::Output) -> Result<(), Self::Error> {
        Err(())
    }
}
impl !ctor::SelfCtor for FallibleCtor {}

#[gtest]
fn test_valueless_by_exception() {
    let mut count = 0;
    {
        let mut v = ctor::emplace!(unsafe { return_destructor_variant(&mut count) });
        expect_false!(v.valueless_by_exception());
        expect_eq!(count, 0);

        let res = v.as_mut().try_emplace_at::<0, _>(FallibleCtor);
        expect_true!(res.is_err());
        // The previous active DestructorCounter was dropped once before the failed ctor.
        expect_eq!(count, 1);
        expect_true!(v.valueless_by_exception());
        expect_eq!(v.index(), cpp_std::variant_internal::VARIANT_NPOS);
    }
    // Dropping a valueless variant does not drop the previous element a second time.
    expect_eq!(count, 1);

    let v2: cpp_std::variant<(i32, cpp_std::vector<i32>)> = cpp_std::variant::new(42);
    let mut v2 = core::pin::pin!(v2);
    let res = v2.as_mut().try_emplace::<i32, _, _>(FallibleCtor);
    expect_true!(res.is_err());
    expect_true!(v2.as_ref().get_ref().valueless_by_exception());
    let v2_cloned = v2.clone();
    expect_true!(v2_cloned.valueless_by_exception());
    expect_eq!(*v2, v2_cloned);
}

#[gtest]
#[should_panic(expected = "std::variant is valueless_by_exception")]
fn test_valueless_by_exception_as_ref_panics() {
    let mut count = 0;
    let mut v = ctor::emplace!(unsafe { return_destructor_variant(&mut count) });
    let _ = v.as_mut().try_emplace_at::<0, _>(FallibleCtor);
    let _ = cpp_std::variant::as_ref(&*v);
}

#[gtest]
#[should_panic(expected = "std::variant is valueless_by_exception")]
fn test_valueless_by_exception_into_enum_panics() {
    let mut v: cpp_std::variant<(i32, cpp_std::vector<i32>)> = cpp_std::variant::new(42);
    let _ = unsafe { core::pin::Pin::new_unchecked(&mut v) }.try_emplace_at::<0, _>(FallibleCtor);
    let _ = v.into_enum();
}

#[gtest]
fn test_macros() {
    let v1: cpp_std::variant![i32, f32, bool] = cpp_std::variant::new_at::<0>(42);
    expect_eq!(v1.get_at::<0>(), Some(&42));

    let v2: cpp_std::trivial_variant![i32, f32, bool] = cpp_std::trivial_variant::new_at::<1>(3.5);
    expect_eq!(v2.get_at::<1>(), Some(&3.5));

    let v3: cpp_std::variant![i32] = cpp_std::variant::new_at::<0>(99);
    expect_eq!(v3.get_at::<0>(), Some(&99));

    // Arity 8 uses the flat `Variant8` tuple representation.
    let v8: cpp_std::trivial_variant![u8, u16, u32, u64, i8, i16, i32, i64] =
        cpp_std::trivial_variant::new_at::<7>(77i64);
    expect_eq!(v8.into_enum(), cpp_std::variant_enums::Variant8::V7(77i64));

    // Arity 9 automatically falls back to nested `Coproduct`.
    let v9: cpp_std::trivial_variant![u8, u16, u32, u64, i8, i16, i32, i64, bool] =
        cpp_std::trivial_variant::new_at::<8>(true);
    expect_eq!(v9.get_at::<8>(), Some(&true));
    expect_eq!(v9.get::<bool, _>(), Some(&true));

    let v4: cpp_std::trivial_variant![
        u8,
        u16,
        u32,
        u64,
        usize,
        i8,
        i16,
        i32,
        i64,
        isize,
        f32,
        f64,
        bool,
        char,
        *const u8,
        *mut u8,
        ()
    ] = cpp_std::trivial_variant::new_at::<16>(());
    expect_eq!(v4.get_at::<16>(), Some(&()));
}

#[gtest]
fn test_ctor_new_default() {
    let v_default =
        ctor::emplace!({ <NonTrivialNotUnpinVariant as ctor::CtorNew<()>>::ctor_new(()) });
    expect_eq!(take_non_trivial_not_unpin_variant_by_const_ref(&v_default), 0);
}

#[gtest]
fn test_unpin() {
    fn is_unpin<T: Unpin>() {}
    is_unpin::<cpp_std::variant![i32, cpp_std::vector<i32>]>();

    // Test mutation via &mut
    let mut v: cpp_std::variant![i32, cpp_std::vector<i32>] = cpp_std::variant::new_at::<0>(10);
    *v.get_at_mut::<0>().unwrap() = 20;
    expect_eq!(*v.get_at::<0>().unwrap(), 20);
}

#[gtest]
fn test_small_alignment_variant() {
    let v = return_small_variant_bool(true);
    expect_eq!(v.get::<bool, _>().copied(), Some(true));
    expect_true!(take_small_variant_by_value(v));
    let v2 = SmallVariant::new_at::<1>(true);
    expect_true!(take_small_variant_by_value(v2));
}
