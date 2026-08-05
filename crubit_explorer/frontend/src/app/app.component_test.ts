// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

import {HttpClientTestingModule, HttpTestingController} from '@angular/common/http/testing';
import {TestBed} from '@angular/core/testing';
import {AppComponent} from './app.component';
import {FlatSymbolNode} from './doxygen';

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

    component.toggleSymbolNode(node);
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

  it('should handle selectOutputFile with index', () => {
    component.outputFiles = [
      {name: 'file1.cc', content: 'content 1'},
      {name: 'file2.h', content: 'content 2'},
    ];

    component.selectOutputFile(1);
    expect(component.selectedOutputFileIndex).toBe(1);
  });

  it('should send compile request and process output and doxygen responses', () => {
    component.outputEditor = {
      setValue: () => {},
      getModel: () => ({}),
    };
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
            {name: 'foo', kind: 'function', refid: 'sym_foo'},
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
    component.outputEditor = {
      setValue: () => {},
      getModel: () => ({}),
    };
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
});
