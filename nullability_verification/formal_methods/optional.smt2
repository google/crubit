;; Part of the Crubit project, under the Apache License v2.0 with LLVM
;; Exceptions. See /LICENSE for license information.
;; SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

;; Run: cvc5 --lang smt --incremental optional.smt2

(set-option :produce-models true)
(set-option :produce-assertions true)
(set-option :produce-assignments true)
(set-option :produce-unsat-cores true)
(set-logic HO_ALL)

(define-fun join-fc ((c Bool) (fc-then Bool) (fc-else Bool)) Bool
  (or
    (and c fc-then)
    (and (not c) fc-else)))

(declare-datatype OptionalValue
  ((make-optional-value (get-x0 Bool))))

(define-fun get-has-value ((opt OptionalValue)) Bool
  (get-x0 opt))

(define-fun is-unsafe-to-unwrap ((fc Bool) (opt OptionalValue)) Bool
  (and fc (not (get-has-value opt))))

(echo "=============================")
(echo "=== Example Regular-Check ===")
(push 1)

; void foo(optional<int> x) {
;   // (1)
;   x.value();
;   if (x.has_value()) {
;     // (2)
;     x.value();
;   }
; }

(declare-fun x () OptionalValue)

(define-fun fc-1 () Bool
  true)

(define-fun fc-2 () Bool
  (get-has-value x))

(push 1)
  (assert (is-unsafe-to-unwrap fc-1 x))
  (echo "Expected: sat")
  (echo "Actual:")
  (check-sat)
  (get-value (fc-1 fc-2 x))
(pop 1)

(push 1)
  (assert (is-unsafe-to-unwrap fc-2 x))
  (echo "Expected: unsat")
  (echo "Actual:")
  (check-sat)
  (get-value (fc-1 fc-2 x))
(pop 1)

(pop 1)

(echo "============================")
(echo "=== Example Mixed-Values ===")
(push 1)

; void foo(optional<int> x, bool b) {
;   // (1)
;   if (b) {
;     // (2)
;     x = 42;
;     // (3)
;   }
;   // (4)
;   x.value();
;   if (b) {
;     // (5)
;     x.value();
;   }
; }

(declare-fun x () OptionalValue)
(declare-fun b () Bool)

(define-fun fc-1 () Bool
  true)

(define-fun fc-2 () Bool
  (and fc-1 b))

(define-fun fc-3 () Bool
  (and fc-2 (get-has-value x)))

(define-fun fc-4 () Bool
  (join-fc b fc-3 fc-1))

(define-fun fc-5 () Bool
  (and fc-4 b))

(push 1)
  (assert (is-unsafe-to-unwrap fc-4 x))
  (echo "Expected: sat")
  (echo "Actual:")
  (check-sat)
  (get-value (fc-1 fc-2 fc-3 fc-4 fc-5 x b))
(pop 1)

(push 1)
  (assert (is-unsafe-to-unwrap fc-5 x))
  (echo "Expected: unsat")
  (echo "Actual:")
  (check-sat)
  (get-value (fc-1 fc-2 fc-3 fc-4 fc-5 x b))
(pop 1)

(pop 1)

; vim: set syntax=scheme:
