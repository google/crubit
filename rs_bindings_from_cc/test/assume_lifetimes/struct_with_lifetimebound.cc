// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/assume_lifetimes/struct_with_lifetimebound.h"

namespace {
PlainStruct plain_struct;
}

const PlainStruct StructWithLifetimeboundMemberFunction::f() const {
  return PlainStruct{};
}

const PlainStruct& StructWithLifetimeboundRefMemberFunction::f() const {
  return plain_struct;
}

const PlainStruct DropClassWithLifetimeboundMemberFunction::f() const {
  return PlainStruct{};
}

DropClassWithLifetimeboundMemberFunction::
    ~DropClassWithLifetimeboundMemberFunction() {}

const PlainStruct& DropClassWithLifetimeboundRefMemberFunction::f() const {
  return plain_struct;
}

DropClassWithLifetimeboundRefMemberFunction::
    ~DropClassWithLifetimeboundRefMemberFunction() {}

DropStructWithLifetimeboundCtor::~DropStructWithLifetimeboundCtor() {}

DropStructWithLifetimeboundRefCtor::~DropStructWithLifetimeboundRefCtor() {}

const PlainStruct& DropStructWithRefCtorAndRefMemberFunction::f() const {
  return plain_struct;
}

DropStructWithRefCtorAndRefMemberFunction::
    ~DropStructWithRefCtorAndRefMemberFunction() {}

const PlainStruct DropStructWithCtorAndMemberFunction::f() const {
  return PlainStruct{};
}

DropStructWithCtorAndMemberFunction::~DropStructWithCtorAndMemberFunction() {}

const PlainStruct& DropStructWithCtorAndRefMemberFunction::f() const {
  return plain_struct;
}

DropStructWithCtorAndRefMemberFunction::
    ~DropStructWithCtorAndRefMemberFunction() {}

const PlainStruct DropStructWithRefCtorAndMemberFunction::f() const {
  return PlainStruct{};
}

DropStructWithRefCtorAndMemberFunction::
    ~DropStructWithRefCtorAndMemberFunction() {}
