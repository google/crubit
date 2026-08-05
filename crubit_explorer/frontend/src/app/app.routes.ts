// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

import {Routes} from '@angular/router';
import {MainComponent} from './main.component';

export const routes: Routes = [
  {path: '', component: MainComponent},
  {path: '**', redirectTo: ''}
];
