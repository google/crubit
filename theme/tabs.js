/**
 * Part of the Crubit project, under the Apache License v2.0 with LLVM
 * Exceptions. See /LICENSE for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 * @license Apache-2.0 WITH LLVM-exception
 */

/**
 * Change active tab of tabs.
 *
 * @param {Element} container
 * @param {string} name
 */
function changeTab(container, name) {
  for (const child of container.children) {
    if (!(child instanceof HTMLElement)) {
      continue;
    }

    if (child.classList.contains('mdbook-tabs')) {
      for (const tab of child.children) {
        if (!(tab instanceof HTMLElement)) {
          continue;
        }

        if (tab.dataset.tabname === name) {
          tab.classList.add('active');
        } else {
          tab.classList.remove('active');
        }
      }
    } else if (child.classList.contains('mdbook-tab-content')) {
      if (child.dataset.tabname === name) {
        child.classList.remove('hidden');
      } else {
        child.classList.add('hidden');
      }
    }
  }
}

document.addEventListener('DOMContentLoaded', () => {
  const tabs = document.querySelectorAll('.mdbook-tab');
  for (const tab of tabs) {
    tab.addEventListener('click', () => {
      if (!(tab instanceof HTMLElement)) {
        return;
      }

      if (!tab.parentElement || !tab.parentElement.parentElement) {
        return;
      }

      const container = tab.parentElement.parentElement;
      const name = tab.dataset.tabname;
      const globalTabId = container.dataset.tabglobal;

      changeTab(container, name);

      if (globalTabId) {
        localStorage.setItem(`mdbook-tabs-${globalTabId}`, name);

        const globalContainers = document.querySelectorAll(
            `.mdbook-tabs-container[data-tabglobal="${globalTabId}"]`);
        for (const globalContainer of globalContainers) {
          changeTab(globalContainer, name);
        }
      }
    });
  }

  const containers =
      document.querySelectorAll('.mdbook-tabs-container[data-tabglobal]');
  for (const container of containers) {
    const globalTabId = container.dataset.tabglobal;

    const name = localStorage.getItem(`mdbook-tabs-${globalTabId}`);
    if (name && document.querySelector(`.mdbook-tab[data-tabname=${name}]`)) {
      changeTab(container, name);
    }
  }
});