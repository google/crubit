// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

import loader from '@monaco-editor/loader';
import type * as monaco from 'monaco-editor';

/**
 * Asynchronously loads the Monaco editor scripts and returns the global `monaco` object.
 */
export async function loadMonaco(): Promise<typeof monaco> {
  const monaco = await loader.init();
  return monaco;
}
