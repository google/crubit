// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

import {HttpClientTestingModule, HttpTestingController} from '@angular/common/http/testing';
import {TestBed} from '@angular/core/testing';
import {AppComponent} from './app.component';
import {buildFlatSymbolTree, FlatSymbolNode} from './doxygen';

describe('AppComponent', () => {
  let component: AppComponent;
  let httpMock: HttpTestingController;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AppComponent, HttpClientTestingModule],
    }).compileComponents();

    const fixture = TestBed.createComponent(AppComponent);
    component = fixture.componentInstance;
    httpMock = TestBed.inject(HttpTestingController);
  });

  afterEach(() => {
    httpMock.verify();
  });

  it('should create the component', () => {
    expect(component).toBeTruthy();
  });

  it('should initialize with default state', () => {
    expect(component.isCompiling).toBeFalse();
    expect(component.outputFiles).toEqual([]);
    expect(component.selectedOutputFileIndex).toBe(0);
    expect(component.flatDoxygenSymbols).toEqual([]);
    expect(component.selectedSymbol).toBeNull();
    expect(component.doxygenError).toBe('');
    expect(component.isDoxygenCollapsed).toBeFalse();
  });

  it('should toggle doxygen panel state', () => {
    expect(component.isDoxygenCollapsed).toBeFalse();
    component.toggleDoxygenPanel();
    expect(component.isDoxygenCollapsed).toBeTrue();
    component.toggleDoxygenPanel();
    expect(component.isDoxygenCollapsed).toBeFalse();
  });

  it('should toggle symbol node collapse state when node has children', () => {
    const node: FlatSymbolNode = {
      name: 'MyStruct',
      fullName: 'MyStruct',
      kind: 'struct',
      refid: '1',
      depth: 0,
      hasChildren: true,
      collapsed: true,
      visible: true,
    };
    const mockEvent = jasmine.createSpyObj<MouseEvent>('MouseEvent', ['stopPropagation']);

    component.toggleSymbolNode(node, mockEvent);

    expect(mockEvent.stopPropagation).toHaveBeenCalled();
    expect(node.collapsed).toBeFalse();

    component.toggleSymbolNode(node);
    expect(node.collapsed).toBeTrue();
  });

  it('should not toggle symbol node collapse state when node has no children', () => {
    const node: FlatSymbolNode = {
      name: 'my_field',
      fullName: 'MyStruct::my_field',
      kind: 'field',
      refid: '2',
      depth: 1,
      hasChildren: false,
      collapsed: true,
      visible: true,
    };

    component.toggleSymbolNode(node);

    expect(node.collapsed).toBeTrue();
  });

  it('should select a symbol and stop event propagation', () => {
    const node: FlatSymbolNode = {
      name: 'hello',
      fullName: 'hello',
      kind: 'function',
      refid: '3',
      depth: 0,
      hasChildren: false,
      collapsed: false,
      visible: true,
    };
    const mockEvent = jasmine.createSpyObj<MouseEvent>('MouseEvent', ['stopPropagation']);

    component.selectSymbol(node, mockEvent);

    expect(mockEvent.stopPropagation).toHaveBeenCalled();
    expect(component.selectedSymbol).toBe(node);
  });

  it('should handle selectOutputFile with index', () => {
    component.outputFiles = [
      {name: 'file1.cc', content: 'content 1'},
      {name: 'file2.h', content: 'content 2'},
    ];

    component.selectOutputFile(1);
    expect(component.selectedOutputFileIndex).toBe(1);
  });

  it('should send compile request and process output and doxygen responses', () => {
    const rustCode = 'pub fn foo() {}';
    component.compile(rustCode);

    expect(component.isCompiling).toBeTrue();

    const compileReq = httpMock.expectOne('/api/compile');
    expect(compileReq.request.method).toBe('POST');
    expect(compileReq.request.body.pluginName).toBe('cc_bindings_from_rs');

    const mockOutputB64 = btoa('// Generated C++ code');
    compileReq.flush({
      output: {
        files: [{name: 'input_rs_api.h', contentsB64: mockOutputB64}],
      },
    });

    expect(component.isCompiling).toBeFalse();
    expect(component.outputFiles.length).toBe(1);
    expect(component.outputFiles[0].name).toBe('input_rs_api.h');
    expect(component.outputFiles[0].content).toBe('// Generated C++ code');

    const doxygenReq = httpMock.expectOne('/api/doxygen');
    expect(doxygenReq.request.method).toBe('POST');

    doxygenReq.flush({
      fileSymbols: {
        'input_rs_api.h': {
          symbols: [
            {name: 'foo', kind: 'function', refid: 'sym_foo', line: 5},
          ],
        },
      },
    });

    expect(component.flatDoxygenSymbols.length).toBe(1);
    expect(component.flatDoxygenSymbols[0].name).toBe('foo');
  });

  it('should handle compile error response', () => {
    component.compile('invalid rust code');

    expect(component.isCompiling).toBeTrue();

    const compileReq = httpMock.expectOne('/api/compile');
    compileReq.flush({
      error: {
        reason: 'Syntax error in Rust code',
      },
    });

    expect(component.isCompiling).toBeFalse();
    expect(component.outputFiles).toEqual([]);
    expect(component.doxygenError).toBe('Syntax error in Rust code');
  });

  it('should handle compile HTTP error', () => {
    component.compile('pub fn foo() {}');

    const compileReq = httpMock.expectOne('/api/compile');
    compileReq.flush('Server error', {status: 500, statusText: 'Internal Server Error'});

    expect(component.isCompiling).toBeFalse();
    expect(component.outputFiles).toEqual([]);
  });

  it('should handle doxygen error response', () => {
    component.compile('pub fn foo() {}');

    const compileReq = httpMock.expectOne('/api/compile');
    compileReq.flush({
      output: {
        files: [{name: 'out.h', contentsB64: btoa('code')}],
      },
    });

    const doxygenReq = httpMock.expectOne('/api/doxygen');
    doxygenReq.flush({
      error: {
        text: 'Doxygen failed',
        reason: 'Parse error',
      },
    });

    expect(component.doxygenError).toBe('Doxygen failed: Parse error');
  });

  describe('decodeState', () => {
    it('should decode a valid encoded JSON state', () => {
      const state = {
        v: 1,
        tool: 'cc_bindings_from_rs',
        files: [{name: 'input.rs', content: 'pub fn foo() {}'}],
      };
      const encoded = component.encodeState(state);
      expect(component.decodeState(encoded)).toEqual(state);
    });

    it('should encode and decode state with unicode characters', () => {
      const state = {
        v: 1,
        tool: 'cc_bindings_from_rs',
        files: [{name: 'input.rs', content: '// 🚀 Unicode test: こんにちは世界'}],
      };
      const encoded = component.encodeState(state);
      expect(component.decodeState(encoded)).toEqual(state);
    });

    it('should fallback for raw non-JSON code strings', () => {
      const rawCode = 'pub fn foo() {}';
      const decoded = component.decodeState(rawCode);
      expect(decoded).toEqual({
        v: 1,
        tool: 'cc_bindings_from_rs',
        files: [{name: 'input.rs', content: rawCode}],
      });
    });

    it('should fallback for JSON strings that do not contain valid files array', () => {
      const invalidJson = JSON.stringify({foo: 'bar'});
      const decoded = component.decodeState(invalidJson);
      expect(decoded).toEqual({
        v: 1,
        tool: 'cc_bindings_from_rs',
        files: [{name: 'input.rs', content: invalidJson}],
      });
    });
  });

  describe('getStateFromUrl and getCodeFromUrl', () => {
    let originalHash: string;
    beforeEach(() => { originalHash = window.location.hash; });
    afterEach(() => { window.location.hash = originalHash; });

    it('should extract state from location.hash or return null when empty', () => {
      window.location.hash = '';
      expect(component.getStateFromUrl()).toBeNull();
      expect(component.getCodeFromUrl()).toBeNull();

      const state = {v: 1, tool: 'cc_bindings_from_rs', files: [{name: 'input.rs', content: 'fn main() {}'}]};
      const encoded = component.encodeState(state);

      window.location.hash = `#code=${encoded}`;
      expect(component.getStateFromUrl()).toEqual(state);
      expect(component.getCodeFromUrl()).toBe('fn main() {}');

      window.location.hash = `#${encoded}`;
      expect(component.getStateFromUrl()).toEqual(state);
    });
  });

  describe('updateUrl', () => {
    it('should encode state into location.hash via replaceState', () => {
      const replaceSpy = spyOn(window.history, 'replaceState');

      const expectedState1 = {
        v: 1,
        tool: 'cc_bindings_from_rs',
        files: [{name: 'input.rs', content: 'pub fn bar() {}'}],
      };
      const expectedEncoded1 = component.encodeState(expectedState1);
      component.updateUrl('pub fn bar() {}');
      expect(replaceSpy).toHaveBeenCalled();
      const url1 = replaceSpy.calls.mostRecent().args[2] as string;
      expect(url1).toContain(`#code=${expectedEncoded1}`);

      const expectedState2 = {
        v: 1,
        tool: 'custom_tool',
        files: [{name: 'input.rs', content: 'struct S;'}],
      };
      const expectedEncoded2 = component.encodeState(expectedState2);
      component.updateUrl(expectedState2.files, expectedState2.tool);
      expect(replaceSpy).toHaveBeenCalled();
      const url2 = replaceSpy.calls.mostRecent().args[2] as string;
      expect(url2).toContain(`#code=${expectedEncoded2}`);
    });
  });

  describe('copyShareLink and fallbackCopy', () => {
    let originalClipboard: PropertyDescriptor | undefined;
    beforeEach(() => { originalClipboard = Object.getOwnPropertyDescriptor(navigator, 'clipboard'); });
    afterEach(() => {
      if (originalClipboard) {
        Object.defineProperty(navigator, 'clipboard', originalClipboard);
      } else {
        try { delete (navigator as any).clipboard; } catch {}
      }
    });

    it('should copy link via navigator.clipboard or fallbackCopy', async () => {
      spyOn(window.history, 'replaceState');
      const writeTextSpy = jasmine.createSpy('writeText').and.returnValue(Promise.resolve());
      Object.defineProperty(navigator, 'clipboard', {value: {writeText: writeTextSpy}, configurable: true, writable: true});

      component.copyShareLink();
      expect(writeTextSpy).toHaveBeenCalledWith(window.location.href);
      await Promise.resolve();
      expect(component.shareButtonText).toBe('Copied!');

      // Fallback when clipboard fails or is unavailable
      const failWriteText = jasmine.createSpy('writeText').and.returnValue(Promise.reject('error'));
      Object.defineProperty(navigator, 'clipboard', {value: {writeText: failWriteText}, configurable: true, writable: true});
      spyOn(document, 'execCommand').and.returnValue(true);

      component.copyShareLink();
      await Promise.resolve();
      await new Promise((r) => setTimeout(r, 0));
      expect(document.execCommand).toHaveBeenCalledWith('copy');
    });
  });

  describe('doxygen symbol tree and visibility', () => {
    it('should update symbol visibility when toggling nodes', () => {
      component.compile('pub fn foo() {}');
      httpMock.expectOne('/api/compile').flush({output: {files: [{name: 'out.h', contentsB64: btoa('code')}]}});
      httpMock.expectOne('/api/doxygen').flush({
        fileSymbols: {
          'out.h': {
            symbols: [
              {name: 'Outer::Inner', kind: 'struct', refid: 's1', line: 10},
              {name: 'Outer::Inner::field', kind: 'field', refid: 'f1', line: 11},
            ],
          },
        },
      });

      expect(component.flatDoxygenSymbols.length).toBe(3);
      const [outer, inner, field] = component.flatDoxygenSymbols;
      expect(outer.visible).toBeTrue();
      expect(inner.visible).toBeFalse();

      component.toggleSymbolNode(outer);
      expect(inner.visible).toBeTrue();
      expect(field.visible).toBeFalse();

      component.toggleSymbolNode(inner);
      expect(field.visible).toBeTrue();
    });
  });

  describe('selectSymbol', () => {
    it('should select symbol and reveal/select line in editor', () => {
      const revealSpy = jasmine.createSpy('revealLineInCenter');
      const setSelSpy = jasmine.createSpy('setSelection');
      const findMatchesSpy = jasmine.createSpy('findMatches').and.returnValue([{range: {startLineNumber: 8, startColumn: 1, endLineNumber: 8, endColumn: 10}}]);

      component.outputEditor = {
        getModel: () => ({
          getLineCount: () => 20,
          getLineMaxColumn: () => 15,
          findMatches: findMatchesSpy,
        }),
        revealLineInCenter: revealSpy,
        setSelection: setSelSpy,
        focus: jasmine.createSpy('focus'),
        dispose: jasmine.createSpy('dispose'),
      };

      const nodeLine: FlatSymbolNode = {name: 'f', fullName: 'f', kind: 'fn', refid: '1', line: 5, depth: 0, hasChildren: false, collapsed: false, visible: true};
      component.selectSymbol(nodeLine);
      expect(revealSpy).toHaveBeenCalledWith(5);

      const nodeNoLine: FlatSymbolNode = {name: 'f', fullName: 'f', kind: 'fn', refid: '1', depth: 0, hasChildren: false, collapsed: false, visible: true};
      component.selectSymbol(nodeNoLine);
      expect(findMatchesSpy).toHaveBeenCalledWith('f', false, false, true, null, true);
      expect(revealSpy).toHaveBeenCalledWith(8);
    });
  });

  describe('buildFlatSymbolTree', () => {
    it('should build hierarchical tree sorted alphabetically', () => {
      const flatTree = buildFlatSymbolTree([
        {name: 'Z::foo', kind: 'function', refid: 'f1'},
        {name: 'A::bar', kind: 'function', refid: 'f2'},
      ]);
      expect(flatTree.map((n) => n.name)).toEqual(['A', 'bar', 'Z', 'foo']);
    });
  });
});


