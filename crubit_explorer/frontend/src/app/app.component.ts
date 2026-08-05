// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

import {CommonModule} from '@angular/common';
import {HttpClient} from '@angular/common/http';
import {AfterViewInit, ChangeDetectorRef, Component, ElementRef, NgZone, OnDestroy, ViewChild} from '@angular/core';
import {Subject, Subscription} from 'rxjs';
import {debounceTime} from 'rxjs/operators';

import {buildFlatSymbolTree, DoxygenResponse, DoxygenSymbol, FlatSymbolNode} from './doxygen';
import {loadMonaco} from './monaco_loader';

export interface InputFileState {
  name: string;
  content: string;
}

export interface ExplorerState {
  v?: number;
  tool?: string;
  files: InputFileState[];
}

const SHARE_BUTTON_RESET_DELAY_MS = 2000;

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './app.component.html',
  styleUrls: []
})
export class AppComponent implements AfterViewInit, OnDestroy {
  @ViewChild('inputEditorContainer', {static: true})
  inputContainer!: ElementRef;
  @ViewChild('outputEditorContainer', {static: true})
  outputContainer!: ElementRef;

  inputEditor: any;
  outputEditor: any;

  private inputChangeSubject = new Subject<string>();
  private subscription!: Subscription;
  isCompiling = false;

  outputFiles: Array<{name: string, content: string}> = [];
  selectedOutputFileIndex = 0;

  private doxygenSymbols: Record<string, {symbols?: DoxygenSymbol[]}> = {};
  flatDoxygenSymbols: FlatSymbolNode[] = [];
  selectedSymbol: FlatSymbolNode | null = null;
  doxygenError = '';
  isDoxygenCollapsed = false;
  shareButtonText = 'Share';
  selectedTool = 'cc_bindings_from_rs';

  constructor(
      private http: HttpClient,
      private cdr: ChangeDetectorRef,
      private zone: NgZone
  ) {}

  encodeState(state: ExplorerState): string {
    const jsonStr = JSON.stringify(state);
    // encodeURIComponent encodes multibyte Unicode characters into
    // percent sequences. `unescape` converts those into an 8 bit string,
    // so we can safely encode non-ASCII characters to base64
    const base64 = btoa(unescape(encodeURIComponent(jsonStr)));
    return base64.replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
  }

  decodeState(encoded: string): ExplorerState {
    let base64 = encoded.replace(/-/g, '+').replace(/_/g, '/');
    while (base64.length % 4 !== 0) {
      base64 += '=';
    }
    let decodedText = '';
    try {
      // This is the reverse of the encoding, turning base64 into the
      // percent-encoded sequence, and then decoding that back into the
      // non-ASCII string
      decodedText = decodeURIComponent(escape(atob(base64)));
    } catch {
      try {
        decodedText = decodeURIComponent(encoded);
      } catch {
        decodedText = encoded;
      }
    }

    let parsed: any;
    try {
      parsed = JSON.parse(decodedText);
    } catch {
      // Fallback for non-JSON raw code strings
    }

    if (parsed && typeof parsed === 'object' && Array.isArray(parsed.files) && parsed.files.length > 0) {
      return {
        v: parsed.v || 1,
        tool: parsed.tool || 'cc_bindings_from_rs',
        files: parsed.files
      };
    }

    return {
      v: 1,
      tool: 'cc_bindings_from_rs',
      files: [{ name: 'input.rs', content: decodedText }]
    };
  }

  getStateFromUrl(): ExplorerState | null {
    if (typeof window === 'undefined' || !window.location) return null;
    let codeParam: string | null = null;
    if (window.location.hash) {
      const hash = window.location.hash.startsWith('#')
          ? window.location.hash.slice(1)
          : window.location.hash;
      const hashParams = new URLSearchParams(hash);
      codeParam = hashParams.get('code') || hash;
    }
    if (!codeParam) {
      const urlParams = new URLSearchParams(window.location.search);
      codeParam = urlParams.get('code');
    }
    if (codeParam) {
      return this.decodeState(codeParam);
    }
    return null;
  }

  getCodeFromUrl(): string | null {
    const state = this.getStateFromUrl();
    if (state && state.files && state.files.length > 0) {
      return state.files[0].content;
    }
    return null;
  }

  updateUrl(filesOrContent: InputFileState[] | string, tool = this.selectedTool): void {
    if (typeof window === 'undefined' || !window.location) return;
    let files: InputFileState[];
    if (typeof filesOrContent === 'string') {
      files = [{ name: 'input.rs', content: filesOrContent }];
    } else {
      files = filesOrContent;
    }
    const state: ExplorerState = { v: 1, tool, files };
    const encoded = this.encodeState(state);
    const url = new URL(window.location.href);
    url.searchParams.delete('code');
    url.hash = `code=${encoded}`;
    window.history.replaceState(null, '', url.toString());
  }

  copyShareLink(): void {
    const code = this.inputEditor ? this.inputEditor.getValue() : '';
    if (code) {
      this.updateUrl(code);
    }
    const shareUrl = window.location.href;
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(shareUrl).then(() => {
        this.shareButtonText = 'Copied!';
        setTimeout(() => {
          this.shareButtonText = 'Share';
          this.cdr.detectChanges();
        }, SHARE_BUTTON_RESET_DELAY_MS);
        this.cdr.detectChanges();
      }).catch(() => {
        this.fallbackCopy(shareUrl);
      });
    } else {
      this.fallbackCopy(shareUrl);
    }
  }

  private fallbackCopy(text: string): void {
    const el = document.createElement('textarea');
    el.value = text;
    document.body.appendChild(el);
    el.select();
    try {
      document.execCommand('copy');
      this.shareButtonText = 'Copied!';
    } catch {
      this.shareButtonText = 'Failed';
    }
    document.body.removeChild(el);
    setTimeout(() => {
      this.shareButtonText = 'Share';
      this.cdr.detectChanges();
    }, SHARE_BUTTON_RESET_DELAY_MS);
    this.cdr.detectChanges();
  }

  ngAfterViewInit() {
    loadMonaco().then((monaco: any) => {
      this.zone.run(() => {
        const prefersDark = window.matchMedia &&
            window.matchMedia('(prefers-color-scheme: dark)').matches;
      const theme = prefersDark ? 'vs-dark' : 'vs';

      const state = this.getStateFromUrl();
      if (state?.tool) {
        this.selectedTool = state.tool;
      }

      const initialCode = (state?.files && state.files.length > 0)
          ? state.files[0].content
          : '// Write Rust code here\npub struct MyStruct {\n    pub a: i32,\n}\n\npub extern "C" fn hello() {}\n';

      this.inputEditor = monaco.editor.create(this.inputContainer.nativeElement, {
        value: initialCode,
        language: 'rust',
        theme: theme,
        minimap: {enabled: false},
        automaticLayout: true
      });

      this.outputEditor =
          monaco.editor.create(this.outputContainer.nativeElement, {
            value: '// Output will appear here',
            language: 'cpp',
            theme: theme,
            readOnly: true,
            minimap: {enabled: false},
            automaticLayout: true
          });

      this.inputEditor.onDidChangeModelContent(() => {
        this.inputChangeSubject.next(this.inputEditor.getValue());
      });

      this.compile();
      });
    });

    this.subscription =
        this.inputChangeSubject.pipe(debounceTime(600)).subscribe(content => {
          this.compile(content);
        });
  }

  selectOutputFile(index: number) {
    this.selectedOutputFileIndex = index;
    this.updateFilteredDoxygenSymbols();

    if (!this.outputEditor || !this.outputFiles[index]) {
      return;
    }

    this.outputEditor.setValue(this.outputFiles[index].content);
    this.updateOutputEditorLanguage(this.outputFiles[index].name);
  }

  private updateOutputEditorLanguage(fileName: string) {
    const monaco = (window as any).monaco;
    if (!monaco?.editor || !this.outputEditor) {
      return;
    }

    let lang = '';
    if (fileName.endsWith('.rs')) {
      lang = 'rust';
    } else if ([
                 '.h', '.hpp', '.cc', '.cpp'
               ].some(ext => fileName.endsWith(ext))) {
      lang = 'cpp';
    }

    if (lang) {
      monaco.editor.setModelLanguage(this.outputEditor.getModel(), lang);
    }
  }

  compile(content?: string) {
    if (!content && this.inputEditor) {
      content = this.inputEditor.getValue();
    }
    if (!content) return;

    this.updateUrl(content);

    this.isCompiling = true;

    const payload = {
      pluginName: this.selectedTool,
      input: {
        files: [{
          name: 'input.rs',
          contentsB64: btoa(unescape(encodeURIComponent(content)))
        }]
      }
    };

    this.http.post<any>('/api/compile', payload).subscribe({
      next: (res) => {
        this.isCompiling = false;
        if (res.error) {
          this.outputFiles = [];
          this.setDoxygen({}, res.error.reason || res.error.text);
          if (this.outputEditor) {
            this.outputEditor.setValue(
                '// Error:\n' + (res.error.reason || res.error.text));
          }
          return;
        }

        if (!res.output?.files?.length) {
          this.setDoxygen();
          return;
        }

        this.outputFiles = res.output.files.map(
            (file: {name: string, contentsB64: string}) => ({
              name: file.name,
              content: decodeURIComponent(escape(atob(file.contentsB64)))
            }));

        if (this.selectedOutputFileIndex >= this.outputFiles.length) {
          this.selectedOutputFileIndex = 0;
        }

        const currentFile = this.outputFiles[this.selectedOutputFileIndex];
        if (!currentFile) {
          return;
        }

        if (this.outputEditor) {
          this.outputEditor.setValue(currentFile.content);
          this.updateOutputEditorLanguage(currentFile.name);
        }

        const doxygenPayload = {
          input: {
            files: res.output.files
          }
        };

        this.http.post<DoxygenResponse>('/api/doxygen', doxygenPayload).subscribe({
          next: (doxyRes) => {
            if (doxyRes.error) {
              this.setDoxygen({}, `${doxyRes.error.text}: ${doxyRes.error.reason}`);
            } else {
              this.setDoxygen(doxyRes.fileSymbols ?? {});
            }
            this.cdr.detectChanges();
          },
          error: (err) => {
            this.setDoxygen({}, err.message || 'Doxygen request failed');
            this.cdr.detectChanges();
          }
        });

        this.cdr.detectChanges();
      },
      error: (err) => {
        this.isCompiling = false;
        this.outputFiles = [];
        this.setDoxygen();
        const errText =
            err.error?.error?.reason || err.message || 'Unknown Error';
        if (this.outputEditor) {
          this.outputEditor.setValue('// Error:\n' + errText);
        }
        this.cdr.detectChanges();
      }
    });
  }

  private setDoxygen(
      symbols: Record<string, {symbols?: DoxygenSymbol[]}> = {},
      error = ''
  ): void {
    this.doxygenSymbols = symbols;
    this.doxygenError = error;
    this.updateFilteredDoxygenSymbols();
  }

  private updateFilteredDoxygenSymbols(): void {
    this.selectedSymbol = null;
    if (this.selectedOutputFileIndex < 0 ||
        !this.outputFiles[this.selectedOutputFileIndex]) {
      this.flatDoxygenSymbols = [];
      return;
    }
    const selectedName = this.outputFiles[this.selectedOutputFileIndex].name;
    const fileSymbols = this.doxygenSymbols[selectedName]?.symbols ?? [];
    this.flatDoxygenSymbols = buildFlatSymbolTree(fileSymbols);
    this.updateSymbolVisibility();
  }

  selectSymbol(node: FlatSymbolNode, event?: MouseEvent): void {
    if (event) {
      event.stopPropagation();
    }
    this.selectedSymbol = node;

    if (!this.outputEditor) {
      return;
    }

    const model = this.outputEditor.getModel();
    if (!model) {
      return;
    }

    let targetLine: number | null = null;

    if (node.line && node.line > 0 && node.line <= model.getLineCount()) {
      targetLine = node.line;
    } else {
      const searchName = node.name;
      const matches = model.findMatches(searchName, false, false, true, null, true);
      if (matches && matches.length > 0) {
        targetLine = matches[0].range.startLineNumber;
      }
    }

    if (targetLine !== null) {
      const maxCol = model.getLineMaxColumn(targetLine);
      this.outputEditor.revealLineInCenter(targetLine);
      this.outputEditor.setSelection({
        startLineNumber: targetLine,
        startColumn: 1,
        endLineNumber: targetLine,
        endColumn: maxCol
      });
      this.outputEditor.focus();
    }
  }

  toggleSymbolNode(node: FlatSymbolNode, event?: MouseEvent): void {
    if (event) {
      event.stopPropagation();
    }
    if (!node.hasChildren) return;
    node.collapsed = !node.collapsed;
    this.updateSymbolVisibility();
  }

  toggleDoxygenPanel(): void {
    this.isDoxygenCollapsed = !this.isDoxygenCollapsed;
  }

  private updateSymbolVisibility(): void {
    let hiddenDepth = Infinity;
    for (const node of this.flatDoxygenSymbols) {
      if (node.depth >= hiddenDepth) {
        node.visible = false;
      } else {
        node.visible = true;
        if (node.collapsed) {
          hiddenDepth = node.depth + 1;
        } else {
          hiddenDepth = Infinity;
        }
      }
    }
  }

  ngOnDestroy() {
    if (this.subscription) {
      this.subscription.unsubscribe();
    }
    if (this.inputEditor) {
      this.inputEditor.dispose();
    }
    if (this.outputEditor) {
      this.outputEditor.dispose();
    }
  }
}
