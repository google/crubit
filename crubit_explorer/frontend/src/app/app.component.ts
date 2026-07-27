// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

import {CommonModule} from '@angular/common';
import {HttpClient} from '@angular/common/http';
import {AfterViewInit, Component, CUSTOM_ELEMENTS_SCHEMA, ElementRef, OnDestroy, ViewChild} from '@angular/core';
import {Subject, Subscription, debounceTime} from 'rxjs';
import loader from '@monaco-editor/loader';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule],
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
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
  monaco: any;

  private inputChangeSubject = new Subject<string>();
  private subscription!: Subscription;
  isCompiling = false;

  constructor(private http: HttpClient) {}

  ngAfterViewInit() {
    loader.init().then(monaco => {
      this.monaco = monaco;
      const prefersDark = window.matchMedia &&
          window.matchMedia('(prefers-color-scheme: dark)').matches;
      const theme = prefersDark ? 'vs-dark' : 'vs';

      this.inputEditor = monaco.editor.create(this.inputContainer.nativeElement, {
        value:
            '// Write Rust code here\npub struct MyStruct {\n    pub a: i32,\n}\n\npub extern "C" fn hello() {}\n',
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
    });

    this.subscription =
        this.inputChangeSubject.pipe(debounceTime(600)).subscribe(content => {
          this.compile(content);
        });
  }

  compile(content?: string) {
    if (!content && this.inputEditor) {
      content = this.inputEditor.getValue();
    }
    if (!content) return;

    this.isCompiling = true;

    const payload = {
      pluginName: 'cc_bindings_from_rs',
      enableCodegenTracing: false,
      pluginFlags: [],
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
          if (this.outputEditor) {
            this.outputEditor.setValue(
                '// Error:\n' + (res.error.reason || res.error.text));
          }
        } else if (
            res.output && res.output.files && res.output.files.length > 0) {
          // Combine all output files for display
          let combinedOutput = '';
          for (let file of res.output.files) {
            const decoded = decodeURIComponent(escape(atob(file.contentsB64)));
            combinedOutput += '// File: ' + file.name + '\n' + decoded + '\n\n';
          }
          if (this.outputEditor) {
            this.outputEditor.setValue(combinedOutput);
          }
        }
      },
      error: (err) => {
        this.isCompiling = false;
        const errText =
            err.error?.error?.reason || err.message || 'Unknown Error';
        if (this.outputEditor) {
          this.outputEditor.setValue('// Error:\n' + errText);
        }
      }
    });
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
