// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

import {decodeBase64ToUtf8, decodeState, encodeState, encodeUtf8ToBase64, getCodeFromUrl, getStateFromUrl, isViewMode, updateUrl} from './state';

describe('state', () => {
  describe('isViewMode', () => {
    it('should return true for valid ViewMode values', () => {
      expect(isViewMode('split')).toBeTrue();
      expect(isViewMode('input')).toBeTrue();
      expect(isViewMode('output')).toBeTrue();
    });

    it('should return false for invalid values', () => {
      expect(isViewMode('invalid')).toBeFalse();
      expect(isViewMode('')).toBeFalse();
      expect(isViewMode(null)).toBeFalse();
      expect(isViewMode(undefined)).toBeFalse();
      expect(isViewMode(123)).toBeFalse();
    });
  });

  describe('encodeUtf8ToBase64 / decodeBase64ToUtf8', () => {
    it('should correctly encode and decode UTF-8 strings', () => {
      const original = 'Hello, World! 🦀';
      const encoded = encodeUtf8ToBase64(original);
      expect(decodeBase64ToUtf8(encoded)).toBe(original);
    });
  });

  describe('decodeState', () => {
    it('should decode a valid encoded JSON state', () => {
      const state = {
        v: 1,
        tool: 'cc_bindings_from_rs',
        editable: false,
        files: [{name: 'input.rs', content: 'pub fn foo() {}'}],
      };
      const encoded = encodeState(state);
      expect(decodeState(encoded)).toEqual(state);
    });

    it('should encode and decode state with unicode characters', () => {
      const state = {
        v: 1,
        tool: 'cc_bindings_from_rs',
        editable: false,
        files: [{name: 'input.rs', content: '// 🚀 Unicode test: こんにちは世界'}],
      };
      const encoded = encodeState(state);
      expect(decodeState(encoded)).toEqual(state);
    });

    it('should preserve falsy valid values like v: 0 or tool: ""', () => {
      const state = {
        v: 0,
        tool: '',
        editable: true,
        view: 'input' as const,
        files: [{name: 'input.rs', content: 'fn main() {}'}],
      };
      const encoded = encodeState(state);
      expect(decodeState(encoded)).toEqual(state);
    });

    it('should fallback for raw non-JSON code strings', () => {
      const rawCode = 'pub fn foo() {}';
      const decoded = decodeState(rawCode);
      expect(decoded).toEqual({
        v: 1,
        tool: 'cc_bindings_from_rs',
        editable: false,
        files: [{name: 'input.rs', content: rawCode}],
      });
    });

    it('should fallback for JSON strings that do not contain valid files array', () => {
      const invalidJson = JSON.stringify({foo: 'bar'});
      const decoded = decodeState(invalidJson);
      expect(decoded).toEqual({
        v: 1,
        tool: 'cc_bindings_from_rs',
        editable: false,
        files: [{name: 'input.rs', content: invalidJson}],
      });
    });
  });

  describe('getStateFromUrl and getCodeFromUrl', () => {
    let originalHash: string;
    beforeEach(() => {
      originalHash = window.location.hash;
    });
    afterEach(() => {
      window.location.hash = originalHash;
    });

    it('should extract state from location.hash or return null when empty', () => {
      window.location.hash = '';
      expect(getStateFromUrl()).toBeNull();
      expect(getCodeFromUrl()).toBeNull();

      const state = {
        v: 1,
        tool: 'cc_bindings_from_rs',
        editable: false,
        files: [{name: 'input.rs', content: 'fn main() {}'}],
      };
      const encoded = encodeState(state);

      window.location.hash = `#code=${encoded}`;
      expect(getStateFromUrl()).toEqual(state);
      expect(getCodeFromUrl()).toBe('fn main() {}');

      window.location.hash = `#${encoded}`;
      expect(getStateFromUrl()).toEqual(state);
    });
  });

  describe('updateUrl', () => {
    it('should encode state into location.hash via replaceState', () => {
      const replaceSpy = spyOn(window.history, 'replaceState');

      const expectedState1 = {
        v: 1,
        tool: 'cc_bindings_from_rs',
        editable: undefined,
        view: undefined,
        files: [{name: 'input.rs', content: 'pub fn bar() {}'}],
      };
      const expectedEncoded1 = encodeState(expectedState1);
      updateUrl('pub fn bar() {}');
      expect(replaceSpy).toHaveBeenCalled();
      const url1 = replaceSpy.calls.mostRecent().args[2] as string;
      expect(url1).toContain(`#code=${expectedEncoded1}`);

      const expectedState2 = {
        v: 1,
        tool: 'custom_tool',
        editable: undefined,
        view: undefined,
        files: [{name: 'input.rs', content: 'struct S;'}],
      };
      const expectedEncoded2 = encodeState(expectedState2);
      updateUrl(expectedState2.files, expectedState2.tool);
      expect(replaceSpy).toHaveBeenCalled();
      const url2 = replaceSpy.calls.mostRecent().args[2] as string;
      expect(url2).toContain(`#code=${expectedEncoded2}`);
    });
  });
});
