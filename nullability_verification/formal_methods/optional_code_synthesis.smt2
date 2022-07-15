;; Part of the Crubit project, under the Apache License v2.0 with LLVM
;; Exceptions. See /LICENSE for license information.
;; SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

;; Run: cvc5 --lang smt --incremental --fmf-bound optional_code_synthesis.smt2

(set-option :produce-models true)
(set-option :produce-assertions true)
(set-option :produce-assignments true)
(set-option :produce-unsat-cores true)
(set-logic HO_ALL)

(declare-datatype FlowConditions
  ((make-flow-conditions
    (get-fc-1 Bool)
    (get-fc-2 Bool)
    (get-fc-3 Bool)
    (get-fc-4 Bool)
    (get-fc-5 Bool))))

(define-fun join-fc ((c Bool) (fc-then Bool) (fc-else Bool)) Bool
  (or
    (and c fc-then)
    (and (not c) fc-else)))

(declare-datatype OptionalValue
  ((make-optional-value (get-x0 Bool))))

(declare-fun get-has-value (OptionalValue) Bool)

;; Args: flow-condition, opt.
(declare-fun is-unsafe-to-unwrap (Bool OptionalValue) Bool)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example RegularCheck
;;
;; ```
;; void foo(optional<int> x) {
;;   // (1)
;;   x.value();
;;   if (x.has_value()) {
;;     // (2)
;;     x.value();
;;   }
;; }
;; ```

(define-fun run-RegularCheck
  ((x OptionalValue) (body (-> FlowConditions Bool))) Bool
  (let ((fc-1 true))
    (let ((fc-2 (and fc-1 (get-has-value x))))
      (body (make-flow-conditions fc-1 fc-2 false false false)))))

;; Every program point is reachable.
(assert
  (exists ((x OptionalValue))
    (run-RegularCheck x (lambda ((fcs FlowConditions)) (get-fc-1 fcs)))))

(assert
  (exists ((x OptionalValue))
    (run-RegularCheck x (lambda ((fcs FlowConditions)) (get-fc-2 fcs)))))

;; Unwrap at (1) is unsafe.
(assert
  (exists ((x OptionalValue))
    (run-RegularCheck
      x
      (lambda ((fcs FlowConditions))
        (is-unsafe-to-unwrap (get-fc-1 fcs) x)))))

;; Unwrap at (2) is safe.
(assert
  (forall ((x OptionalValue))
    (run-RegularCheck
      x
      (lambda ((fcs FlowConditions))
      (not (is-unsafe-to-unwrap (get-fc-2 fcs) x))))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example MixedValues
;;
;; ```
;; void foo(optional<int> x, bool b) {
;;   // (1)
;;   if (b) {
;;     // (2)
;;     x = 42;
;;     // (3)
;;   }
;;   // (4)
;;   x.value();
;;   if (b) {
;;     // (5)
;;     x.value();
;;   }
;; }
;; ```

(define-fun run-MixedValues
  ((x OptionalValue) (b Bool) (body (-> FlowConditions Bool))) Bool
  (let ((fc-1 true))
    (let ((fc-2 (and fc-1 b)))
      (let ((fc-3 (and fc-2 (get-has-value x))))
        (let ((fc-4 (join-fc b fc-3 fc-1)))
          (let ((fc-5 (and fc-4 b)))
            (body (make-flow-conditions fc-1 fc-2 fc-3 fc-4 fc-5))))))))

;; Every program point is reachable.
(assert
  (exists ((x OptionalValue) (b Bool))
    (run-MixedValues x b (lambda ((fcs FlowConditions)) (get-fc-1 fcs)))))

(assert
  (exists ((x OptionalValue) (b Bool))
    (run-MixedValues x b (lambda ((fcs FlowConditions)) (get-fc-2 fcs)))))

(assert
  (exists ((x OptionalValue) (b Bool))
    (run-MixedValues x b (lambda ((fcs FlowConditions)) (get-fc-3 fcs)))))

(assert
  (exists ((x OptionalValue) (b Bool))
    (run-MixedValues x b (lambda ((fcs FlowConditions)) (get-fc-4 fcs)))))

(assert
  (exists ((x OptionalValue) (b Bool))
    (run-MixedValues x b (lambda ((fcs FlowConditions)) (get-fc-5 fcs)))))

;; Unwrap at (4) is unsafe.
(assert
  (exists ((x OptionalValue) (b Bool))
    (run-MixedValues
      x b
      (lambda ((fcs FlowConditions))
        (is-unsafe-to-unwrap (get-fc-4 fcs) x)))))

;; Unwrap at (5) is safe.
(assert
  (forall ((x OptionalValue) (b Bool))
    (run-MixedValues
      x b
      (lambda ((fcs FlowConditions))
        (not (is-unsafe-to-unwrap (get-fc-5 fcs) x))))))

(check-sat)
(get-value (get-has-value))
(get-value (is-unsafe-to-unwrap))

; vim: set syntax=scheme:
