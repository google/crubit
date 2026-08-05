// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

export type ViewMode = 'split' | 'input' | 'output';

/**
 * Type predicate to check if a value is a valid ViewMode.
 */
export function isViewMode(v: unknown): v is ViewMode {
  return v === 'split' || v === 'input' || v === 'output';
}

export interface InputFileState {
  name: string;
  content: string;
}

export interface ExplorerState {
  v?: number;
  tool?: string;
  editable?: boolean;
  view?: ViewMode;
  files: InputFileState[];
}

/**
 * Encodes a UTF-8 string into a Base64 string using TextEncoder.
 */
export function encodeUtf8ToBase64(str: string): string {
  const bytes = new TextEncoder().encode(str);
  let binString = '';
  for (let i = 0; i < bytes.length; i++) {
    binString += String.fromCharCode(bytes[i]);
  }
  return btoa(binString);
}

/**
 * Decodes a Base64 string into a UTF-8 string using TextDecoder.
 */
export function decodeBase64ToUtf8(base64: string): string {
  const binString = atob(base64);
  const bytes = new Uint8Array(binString.length);
  for (let i = 0; i < binString.length; i++) {
    bytes[i] = binString.charCodeAt(i);
  }
  return new TextDecoder('utf-8', {fatal: true}).decode(bytes);
}

/**
 * Encodes an explorer state object into a URL-safe Base64 string.
 */
export function encodeState(state: ExplorerState): string {
  const jsonStr = JSON.stringify(state);
  const base64 = encodeUtf8ToBase64(jsonStr);
  return base64.replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}

interface RawParsedState {
  v?: number;
  tool?: string;
  editable?: boolean;
  view?: unknown;
  files?: InputFileState[];
}

export function decodeCode(encoded: string): string {
  let base64 = encoded.replace(/-/g, '+').replace(/_/g, '/');
  while (base64.length % 4 !== 0) {
    base64 += '=';
  }
  try {
    return decodeBase64ToUtf8(base64);
  } catch {
    try {
      return decodeURIComponent(encoded);
    } catch {
      return encoded;
    }
  }
}

/**
 * Decodes a URL-safe Base64 string into an explorer state object.
 */
export function decodeState(encoded: string): ExplorerState {
  if (encoded.includes('code=') || encoded.includes('&tool=') ||
      encoded.includes('&editable=') || encoded.includes('&view=')) {
    const params = new URLSearchParams(encoded);
    const codeParam = params.get('code') || '';
    const decodedText = decodeCode(codeParam);
    const validViews: ViewMode[] = ['split', 'input', 'output'];
    const viewParam = params.get('view') as ViewMode;
    const state: ExplorerState = {
      v: 1,
      tool: params.get('tool') || 'cc_bindings_from_rs',
      editable: params.get('editable') === 'true',
      files: [{name: 'input.rs', content: decodedText}]
    };
    if (validViews.includes(viewParam)) {
      state.view = viewParam;
    }
    return state;
  }

  const decodedText = decodeCode(encoded);

  let parsed: unknown;
  try {
    parsed = JSON.parse(decodedText);
  } catch {
    // Fallback for non-JSON raw code strings
  }

  if (parsed !== null && typeof parsed === 'object') {
    const parsedObj = parsed as RawParsedState;
    if (Array.isArray(parsedObj.files) && parsedObj.files.length > 0) {
      const state: ExplorerState = {
        v: parsedObj.v ?? 1,
        tool: parsedObj.tool ?? 'cc_bindings_from_rs',
        editable: typeof parsedObj.editable === 'boolean' ? parsedObj.editable : false,
        files: parsedObj.files
      };
      if (isViewMode(parsedObj.view)) {
        state.view = parsedObj.view;
      }
      return state;
    }
  }

  return {
    v: 1,
    tool: 'cc_bindings_from_rs',
    editable: false,
    files: [{name: 'input.rs', content: decodedText}]
  };
}

/**
 * Reads and decodes the explorer state from the URL search parameters or hash.
 */
export function getStateFromUrl(): ExplorerState | null {
  if (typeof window === 'undefined' || !window.location) return null;
  let codeParam: string | null = null;
  let hashParams = new URLSearchParams();
  if (window.location.hash) {
    const hash = window.location.hash.startsWith('#')
        ? window.location.hash.slice(1)
        : window.location.hash;
    hashParams = new URLSearchParams(hash);
    codeParam = hashParams.get('code');
    if (!codeParam && !hashParams.has('tool') && !hashParams.has('editable') &&
        !hashParams.has('view')) {
      codeParam = hash;
    }
  }
  const urlParams = new URLSearchParams(window.location.search);
  if (!codeParam) {
    codeParam = urlParams.get('code');
  }
  if (codeParam) {
    const state = decodeState(codeParam);
    const tool = hashParams.get('tool') || urlParams.get('tool');
    if (tool) {
      state.tool = tool;
    }
    const editableParam =
        hashParams.get('editable') ?? urlParams.get('editable');
    if (editableParam !== null) {
      state.editable = editableParam === 'true';
    }
    const viewParam = hashParams.get('view') || urlParams.get('view');
    if (isViewMode(viewParam)) {
      state.view = viewParam;
    }
    return state;
  }
  return null;
}

/**
 * Extracts the primary Rust code string from the URL state.
 */
export function getCodeFromUrl(): string | null {
  const state = getStateFromUrl();
  if (state && state.files && state.files.length > 0) {
    return state.files[0].content;
  }
  return null;
}

/**
 * Updates the window location hash with the encoded explorer state.
 */
export function updateUrl(
    filesOrContent: InputFileState[] | string,
    tool = 'cc_bindings_from_rs',
    editable?: boolean,
    view?: ViewMode
): void {
  if (typeof window === 'undefined' || !window.location) return;
  let files: InputFileState[];
  if (typeof filesOrContent === 'string') {
    files = [{name: 'input.rs', content: filesOrContent}];
  } else {
    files = filesOrContent;
  }
  const state: ExplorerState = {v: 1, tool, editable, view, files};
  const encoded = encodeState(state);
  const url = new URL(window.location.href);
  url.searchParams.delete('code');
  url.searchParams.delete('tool');
  url.searchParams.delete('editable');
  url.searchParams.delete('view');
  url.hash = `code=${encoded}`;
  window.history.replaceState(null, '', url.toString());
}
