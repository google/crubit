// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

import {CommonModule} from '@angular/common';
import {HttpClient} from '@angular/common/http';
import {AfterViewInit, ChangeDetectorRef, Component, ElementRef, NgZone, OnDestroy, ViewChild} from '@angular/core';
import {Subject, Subscription} from 'rxjs';
import {debounceTime} from 'rxjs/operators';

import {loadMonaco} from './monaco_loader';
import {getStateFromUrl, updateUrl} from './state';

@Component({
  selector: 'app-embed',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './embed.component.html',
  styleUrls: []
})
export class EmbedComponent implements AfterViewInit, OnDestroy {
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

  selectedTool = 'cc_bindings_from_rs';
  viewMode: 'split' | 'input' | 'output' = 'split';
  isEditable = false;

  constructor(
      private http: HttpClient,
      private cdr: ChangeDetectorRef,
      private zone: NgZone
  ) {}

  ngAfterViewInit() {
    loadMonaco().then((monaco: any) => {
      this.zone.run(() => {
        const prefersDark = window.matchMedia &&
            window.matchMedia('(prefers-color-scheme: dark)').matches;
        const theme = prefersDark ? 'vs-dark' : 'vs';

        const state = getStateFromUrl();
        if (state) {
          if (state.tool) {
            this.selectedTool = state.tool;
          }
          this.isEditable = !!state.editable;
          if (state.view) {
            this.viewMode = state.view;
          }
        }

        const initialCode = (state?.files && state.files.length > 0)
            ? state.files[0].content
            : '// Write Rust code here\npub struct MyStruct {\n    pub a: i32,\n}\n\npub extern "C" fn hello() {}\n';

        this.inputEditor = monaco.editor.create(this.inputContainer.nativeElement, {
          value: initialCode,
          language: 'rust',
          theme: theme,
          readOnly: !this.isEditable,
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

  toggleEditable(): void {
    this.isEditable = !this.isEditable;
    if (this.inputEditor) {
      this.inputEditor.updateOptions({ readOnly: !this.isEditable });
    }
    const code = this.inputEditor ? this.inputEditor.getValue() : '';
    updateUrl(code, this.selectedTool, this.isEditable, this.viewMode);
    this.cdr.detectChanges();
  }

  setViewMode(mode: 'split' | 'input' | 'output'): void {
    this.viewMode = mode;
    const code = this.inputEditor ? this.inputEditor.getValue() : '';
    updateUrl(code, this.selectedTool, this.isEditable, this.viewMode);
    this.cdr.detectChanges();
    setTimeout(() => {
      if (this.inputEditor) {
        this.inputEditor.layout();
      }
      if (this.outputEditor) {
        this.outputEditor.layout();
      }
    }, 50);
  }

  selectOutputFile(index: number) {
    this.selectedOutputFileIndex = index;
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
    } else if (['.h', '.hpp', '.cc', '.cpp'].some(ext => fileName.endsWith(ext))) {
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

    updateUrl(content, this.selectedTool, this.isEditable, this.viewMode);
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
          if (this.outputEditor) {
            this.outputEditor.setValue('// Error:\n' + (res.error.reason || res.error.text));
          }
          return;
        }
        if (!res.output?.files?.length) {
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
        if (!this.outputEditor || !currentFile) {
          return;
        }
        this.outputEditor.setValue(currentFile.content);
        this.updateOutputEditorLanguage(currentFile.name);
        this.cdr.detectChanges();
      },
      error: (err) => {
        this.isCompiling = false;
        this.outputFiles = [];
        const errText = err.error?.error?.reason || err.message || 'Unknown Error';
        if (this.outputEditor) {
          this.outputEditor.setValue('// Error:\n' + errText);
        }
        this.cdr.detectChanges();
      }
    });
  }

  get currentHash(): string {
    if (typeof window === 'undefined') return '';
    return window.location.hash || '';
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
