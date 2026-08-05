// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

import {HttpClientTestingModule, HttpTestingController} from '@angular/common/http/testing';
import {TestBed} from '@angular/core/testing';
import {buildFlatSymbolTree, FlatSymbolNode} from './doxygen';
import {MainComponent} from './main.component';
import type * as monaco from 'monaco-editor';

describe('MainComponent', () => {
  let component: MainComponent;
  let httpMock: HttpTestingController;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MainComponent, HttpClientTestingModule],
    }).compileComponents();

    const fixture = TestBed.createComponent(MainComponent);
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

  describe('copyDirectLink', () => {
    let originalClipboardDesc: PropertyDescriptor | undefined;

    beforeEach(() => {
      originalClipboardDesc = Object.getOwnPropertyDescriptor(Navigator.prototype, 'clipboard') ||
                              Object.getOwnPropertyDescriptor(navigator, 'clipboard');
    });

    afterEach(() => {
      if (originalClipboardDesc) {
        try {
          Object.defineProperty(Navigator.prototype, 'clipboard', originalClipboardDesc);
        } catch {
          // Ignore if unable to redefine
        }
      } else {
        try {
          delete (navigator as unknown as Record<string, unknown>)['clipboard'];
        } catch {
          // Ignore if unable to delete
        }
      }
    });

    it('should copy share link using navigator.clipboard.writeText when available', async () => {
      const writeTextSpy = jasmine.createSpy('writeText').and.resolveTo();
      Object.defineProperty(navigator, 'clipboard', {
        value: { writeText: writeTextSpy },
        configurable: true,
        writable: true,
      });

      await component.copyDirectLink();

      expect(writeTextSpy).toHaveBeenCalledWith(window.location.href);
      expect(component.directLinkCopyText).toBe('Copied!');
    });

    it('should fallback to execCommand when writeText fails', async () => {
      const writeTextSpy = jasmine.createSpy('writeText').and.returnValue(Promise.reject(new Error('Denied')));
      Object.defineProperty(navigator, 'clipboard', {
        value: { writeText: writeTextSpy },
        configurable: true,
        writable: true,
      });
      const execSpy = spyOn(document, 'execCommand').and.returnValue(true);

      await component.copyDirectLink();

      expect(writeTextSpy).toHaveBeenCalledWith(window.location.href);
      expect(execSpy).toHaveBeenCalledWith('copy');
      expect(component.directLinkCopyText).toBe('Copied!');
    });

    it('should fallback to execCommand when navigator.clipboard is absent', async () => {
      Object.defineProperty(navigator, 'clipboard', {
        value: undefined,
        configurable: true,
        writable: true,
      });
      const execSpy = spyOn(document, 'execCommand').and.returnValue(true);

      await component.copyDirectLink();

      expect(execSpy).toHaveBeenCalledWith('copy');
      expect(component.directLinkCopyText).toBe('Copied!');
    });

    it('should copy embed code using copyEmbedCode', async () => {
      const writeTextSpy = jasmine.createSpy('writeText').and.resolveTo();
      Object.defineProperty(navigator, 'clipboard', {
        value: { writeText: writeTextSpy },
        configurable: true,
        writable: true,
      });

      await component.copyEmbedCode();

      expect(writeTextSpy).toHaveBeenCalledWith(component.embedIframeCode);
      expect(component.embedCodeCopyText).toBe('Copied!');
    });
  });

  describe('share modal', () => {
    it('should open and close share modal', () => {
      expect(component.isShareModalOpen).toBeFalse();
      component.openShareModal();
      expect(component.isShareModalOpen).toBeTrue();
      component.closeShareModal();
      expect(component.isShareModalOpen).toBeFalse();
    });

    it('should update URL when opening share modal if inputEditor exists', () => {
      const replaceSpy = spyOn(window.history, 'replaceState');
      component.inputEditor = {
        getValue: () => 'pub fn test() {}',
      } as unknown as monaco.editor.IStandaloneCodeEditor;
      component.openShareModal();
      expect(replaceSpy).toHaveBeenCalled();
      expect(component.isShareModalOpen).toBeTrue();
    });

    it('should provide shareDirectUrl', () => {
      expect(component.shareDirectUrl).toBe(window.location.href);
    });

    it('should construct embedUrl and embedIframeCode', () => {
      component.selectedTool = 'cc_bindings_from_rs';
      component.isEmbedEditable = true;
      component.embedViewMode = 'split';
      expect(component.embedUrl).toContain('/embed#');
      expect(component.embedIframeCode).toContain(`<iframe src="${component.embedUrl}"`);
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
        } as unknown as monaco.editor.ITextModel),
        revealLineInCenter: revealSpy,
        setSelection: setSelSpy,
        focus: jasmine.createSpy('focus'),
        dispose: jasmine.createSpy('dispose'),
      } as unknown as monaco.editor.IStandaloneCodeEditor;

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


