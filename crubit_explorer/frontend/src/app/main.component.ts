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
import type * as monaco from 'monaco-editor';

import {decodeBase64ToUtf8, encodeUtf8ToBase64, getCodeFromUrl, getStateFromUrl, updateUrl} from './state';

const SHARE_BUTTON_RESET_DELAY_MS = 2000;

/** File emitted by the compilation tool. */
export interface OutputFile {
  name: string;
  content: string;
}

/** Raw output file from the compile API response. */
export interface CompileOutputFile {
  readonly name: string;
  readonly contentsB64: string;
}

/** Response structure for the compile API. */
export interface CompileResponse {
  readonly output?: {
    readonly files?: CompileOutputFile[];
  };
  readonly error?: {
    readonly reason?: string;
    readonly text?: string;
  };
}

@Component({
  selector: 'app-main',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './main.component.html',
})
export class MainComponent implements AfterViewInit, OnDestroy {
  @ViewChild('inputEditorContainer', {static: true})
  inputContainer!: ElementRef;
  @ViewChild('outputEditorContainer', {static: true})
  outputContainer!: ElementRef;

  inputEditor: monaco.editor.IStandaloneCodeEditor | null = null;
  outputEditor: monaco.editor.IStandaloneCodeEditor | null = null;

  private inputChangeSubject = new Subject<string>();
  private subscription!: Subscription;
  isCompiling = false;

  outputFiles: OutputFile[] = [];
  selectedOutputFileIndex = 0;

  private doxygenSymbols: Record<string, {symbols?: DoxygenSymbol[]}> = {};
  flatDoxygenSymbols: FlatSymbolNode[] = [];
  selectedSymbol: FlatSymbolNode | null = null;
  doxygenError = '';
  isDoxygenCollapsed = false;
  selectedTool = 'cc_bindings_from_rs';

  shareButtonText = 'Share';

  constructor(
      private http: HttpClient,
      private cdr: ChangeDetectorRef,
      private zone: NgZone
  ) {}

  /**
   * Copies the URL containing the current editor state to the clipboard.
   */
  async copyShareLink(): Promise<void> {
    const code = this.inputEditor ? this.inputEditor.getValue() : '';
    if (code) {
      updateUrl(code, this.selectedTool);
    }
    const shareUrl = window.location.href;
    if (navigator.clipboard?.writeText) {
      try {
        await navigator.clipboard.writeText(shareUrl);
        this.shareButtonText = 'Copied!';
        setTimeout(() => {
          this.shareButtonText = 'Share';
          this.cdr.detectChanges();
        }, SHARE_BUTTON_RESET_DELAY_MS);
        this.cdr.detectChanges();
        return;
      } catch {
        // Fall back to fallbackCopy if clipboard write fails.
      }
    }
    this.fallbackCopy(shareUrl);
  }

  /**
   * Fallback clipboard copy mechanism using a temporary DOM element.
   */
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
  }

  /**
   * Initializes Monaco editor instances and sets up automatic compilation on input changes.
   */
  ngAfterViewInit(): void {
    loadMonaco().then((monacoInstance) => {
      this.zone.run(() => {
        const prefersDark = window.matchMedia &&
            window.matchMedia('(prefers-color-scheme: dark)').matches;
        const theme = prefersDark ? 'vs-dark' : 'vs';

        const state = getStateFromUrl();
        if (state?.tool) {
          this.selectedTool = state.tool;
        }

        const initialCode = getCodeFromUrl() ||
            '// Write Rust code here\npub struct MyStruct {\n    pub a: i32,\n}\n\npub extern "C" fn hello() {}\n';

        this.inputEditor = monacoInstance.editor.create(this.inputContainer.nativeElement, {
          value: initialCode,
          language: 'rust',
          theme,
          minimap: {enabled: false},
          automaticLayout: true
        });

        this.outputEditor =
            monacoInstance.editor.create(this.outputContainer.nativeElement, {
              value: '// Output will appear here',
              language: 'cpp',
              theme,
              readOnly: true,
              minimap: {enabled: false},
              automaticLayout: true
            });

        this.inputEditor?.onDidChangeModelContent(() => {
          if (this.inputEditor) {
            this.inputChangeSubject.next(this.inputEditor.getValue());
          }
        });

        this.compile();
      });
    });

    this.subscription =
        this.inputChangeSubject.pipe(debounceTime(600)).subscribe(content => {
          this.compile(content);
        });
  }

  /**
   * Switches the active output file displayed in the editor and updates Doxygen symbols.
   */
  selectOutputFile(index: number): void {
    this.selectedOutputFileIndex = index;
    this.updateFilteredDoxygenSymbols();

    if (!this.outputEditor || !this.outputFiles[index]) {
      return;
    }

    this.outputEditor.setValue(this.outputFiles[index].content);
    this.updateOutputEditorLanguage(this.outputFiles[index].name);
  }

  /**
   * Configures the syntax highlighting language for the output editor based on file extension.
   */
  private updateOutputEditorLanguage(fileName: string): void {
    const monacoGlobal = (window as unknown as {monaco?: typeof monaco}).monaco;
    if (!monacoGlobal?.editor || !this.outputEditor) {
      return;
    }

    let lang = '';
    if (fileName.endsWith('.rs')) {
      lang = 'rust';
    } else if (['.h', '.hpp', '.cc', '.cpp'].some(ext => fileName.endsWith(ext))) {
      lang = 'cpp';
    }

    const model = this.outputEditor.getModel();
    if (lang && model) {
      monacoGlobal.editor.setModelLanguage(model, lang);
    }
  }

  /**
   * Compiles the Rust input code via the backend API and updates the output editor and Doxygen view.
   */
  compile(content?: string): void {
    if (!content && this.inputEditor) {
      content = this.inputEditor.getValue();
    }
    if (!content) return;

    updateUrl(content, this.selectedTool);

    this.isCompiling = true;

    const payload = {
      pluginName: this.selectedTool,
      input: {
        files: [{
          name: 'input.rs',
          contentsB64: encodeUtf8ToBase64(content)
        }]
      }
    };

    this.http.post<CompileResponse>('/api/compile', payload).subscribe({
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
            (file) => ({
              name: file.name,
              content: decodeBase64ToUtf8(file.contentsB64)
            }));

        if (this.selectedOutputFileIndex >= this.outputFiles.length) {
          this.selectedOutputFileIndex = 0;
        }

        const currentFile = this.outputFiles[this.selectedOutputFileIndex];
        if (currentFile && this.outputEditor) {
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

  /**
   * Sets the current Doxygen symbol dataset and optional error message.
   */
  private setDoxygen(
      symbols: Record<string, {symbols?: DoxygenSymbol[]}> = {},
      error = ''
  ): void {
    this.doxygenSymbols = symbols;
    this.doxygenError = error;
    this.updateFilteredDoxygenSymbols();
  }

  /**
   * Rebuilds the symbol tree filtered for the currently selected output file.
   */
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

  /**
   * Selects a symbol in the Doxygen tree and highlights its position in the output editor.
   */
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

  /**
   * Toggles the collapsed/expanded state of a symbol tree node.
   */
  toggleSymbolNode(node: FlatSymbolNode, event?: MouseEvent): void {
    if (event) {
      event.stopPropagation();
    }
    if (!node.hasChildren) return;
    node.collapsed = !node.collapsed;
    this.updateSymbolVisibility();
  }

  /**
   * Toggles the visibility of the Doxygen symbol panel.
   */
  toggleDoxygenPanel(): void {
    this.isDoxygenCollapsed = !this.isDoxygenCollapsed;
  }

  /**
   * Recalculates node visibility within the flattened symbol tree based on parent collapse state.
   */
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

  /**
   * Cleans up subscriptions and disposes Monaco editor instances on component destruction.
   */
  ngOnDestroy(): void {
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
