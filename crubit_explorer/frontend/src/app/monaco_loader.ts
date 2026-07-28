// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

import loader from '@monaco-editor/loader';

export async function loadMonaco(): Promise<any> {
  const monaco = await loader.init();
  return monaco;
}
