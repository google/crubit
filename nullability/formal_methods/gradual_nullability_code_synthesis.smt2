;; Part of the Crubit project, under the Apache License v2.0 with LLVM
;; Exceptions. See /LICENSE for license information.
;; SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

;; Run: cvc5 --lang smt --incremental --fmf-bound gradual_nullability_code_synthesis.smt2

(set-option :produce-models true)
(set-option :produce-assertions true)
(set-option :produce-assignments true)
(set-option :produce-unsat-cores true)
(set-logic HO_ALL)

(declare-datatype FlowConditions
  ((make-FlowConditions
    (_get-fc-1 Bool)
    (_get-fc-2 Bool)
    (_get-fc-3 Bool)
    (_get-fc-4 Bool)
    (_get-fc-5 Bool)
    (_get-fc-6 Bool))))

;; Redefine selectors as regular functions, because predefined ones
;; can't be converted to values of function type for some reason.
(define-fun get-fc-1 ((fcs FlowConditions)) Bool
  (_get-fc-1 fcs))

(define-fun get-fc-2 ((fcs FlowConditions)) Bool
  (_get-fc-2 fcs))

(define-fun get-fc-3 ((fcs FlowConditions)) Bool
  (_get-fc-3 fcs))

(define-fun get-fc-4 ((fcs FlowConditions)) Bool
  (_get-fc-4 fcs))

(define-fun get-fc-5 ((fcs FlowConditions)) Bool
  (_get-fc-5 fcs))

(define-fun get-fc-6 ((fcs FlowConditions)) Bool
  (_get-fc-6 fcs))

(define-fun join-fc ((c Bool) (fc-then Bool) (fc-else Bool)) Bool
  (or (and c fc-then)
      (and (not c) fc-else)))

(assert
  (forall ((c Bool) (fc-then Bool) (fc-else Bool))
    (= (join-fc c fc-then fc-else)
       (join-fc (not c) fc-else fc-then))))

(declare-datatype PointerValue
  ((make-PointerValue
    (get-x0 Bool)
    (get-x1 Bool)
)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Solution: unconstrained declarations.

;; Flow condition conjuncts that constrain a pointer according to its
;; annotation.
(declare-fun fc-conj--ptr-is-null (PointerValue) Bool)
(declare-fun fc-conj--ptr-is-unknown (PointerValue) Bool)
(declare-fun fc-conj--ptr-is-nonnull (PointerValue) Bool)
(declare-fun fc-conj--ptr-is-nullable (PointerValue) Bool)

;; Flow condition conjunct that constrains the result of comparing two
;; pointers for equality.
;;
;; Args: lhs, rhs, are-equal.
(declare-fun fc-conj--ptrs-were-compared (PointerValue PointerValue Bool) Bool)

;; Flow condition conjunct that constrains the new joined pointer,
;; by combining the pointers coming from the "then" and the "else" branches
;; of an "if" statement.
;;
;; Args: condition, ptr-then, ptr-else, ptr-joined.
(declare-fun fc-conj--join-ptr (Bool PointerValue PointerValue PointerValue) Bool)

;; Safety criteria, tells us whether it is safe to dereference a given
;; pointer at a given program point. The program point is identified
;; by its flow condition.
;;
;; Args: flow-condition, pointer.
(declare-fun is-unsafe-to-deref (Bool PointerValue) Bool)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Solution S1: two bits, no quantifiers
;; - Bit 1 represents if the pointer's nullability is known
;; - Bit 2 represents if the pointer is not null

(define-fun enable-solution-s1 () Bool false)

(assert (=> enable-solution-s1
  (forall ((p PointerValue))
    (= (fc-conj--ptr-is-null p)
       (= p (make-PointerValue true false))))))

(assert (=> enable-solution-s1
  (forall ((p PointerValue))
    (= (fc-conj--ptr-is-unknown p)
        (or
          (= p (make-PointerValue false false))
          (= p (make-PointerValue false true)))))))

(assert (=> enable-solution-s1
  (forall ((p PointerValue))
    (= (fc-conj--ptr-is-nonnull p)
       (= p (make-PointerValue true true))))))

(assert (=> enable-solution-s1
  (forall ((p PointerValue))
    (= (fc-conj--ptr-is-nullable p)
        (or
          (= p (make-PointerValue true false))
          (= p (make-PointerValue true true)))))))

(assert (=> enable-solution-s1
  (forall ((lhs PointerValue) (rhs PointerValue) (eq Bool))
    (= (fc-conj--ptrs-were-compared lhs rhs eq)
       (and
         ;; nullptr == nullptr
         (=> (and (fc-conj--ptr-is-null lhs) (fc-conj--ptr-is-null rhs))
             eq)

         ;; nullptr != nonnull
         (=> (and (fc-conj--ptr-is-null lhs) (fc-conj--ptr-is-nonnull rhs))
             (not eq))

         ;; nonnull != nullptr
         (=> (and (fc-conj--ptr-is-nonnull lhs) (fc-conj--ptr-is-null rhs))
             (not eq)))))))

(assert (=> enable-solution-s1
  (forall ((c Bool)
           (ptr-then PointerValue)
           (ptr-else PointerValue)
           (ptr-joined PointerValue))
    (= (fc-conj--join-ptr c ptr-then ptr-else ptr-joined)
       (or (and c (= ptr-joined ptr-then))
           (and (not c) (= ptr-joined ptr-else)))))))

(assert (=> enable-solution-s1
  (forall ((fc Bool) (p PointerValue))
    (= (is-unsafe-to-deref fc p)
       (and fc (fc-conj--ptr-is-null p))))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Solution S2: ??? bits, with quantifiers.
;; TODO: Implement.

(define-fun enable-solution-s2 () Bool false)

;(assert (=> enable-solution-s2
;  (forall ((p PointerValue))
;    (= (fc-conj--ptr-is-null p)
;       TODO))))

;(assert (=> enable-solution-s2
;  (forall ((p PointerValue))
;    (= (fc-conj--ptr-is-unknown p)
;       TODO))))

;(assert (=> enable-solution-s2
;  (forall ((p PointerValue))
;    (= (fc-conj--ptr-is-nonnull p)
;       TODO))))

;(assert (=> enable-solution-s2
;  (forall ((p PointerValue))
;    (= (fc-conj--ptr-is-nullable p)
;        TODO))))

;(assert (=> enable-solution-s2
;  (forall ((lhs PointerValue) (rhs PointerValue) (eq Bool))
;    (= (fc-conj--ptrs-were-compared lhs rhs eq)
;       TODO))))

;(assert (=> enable-solution-s2
;  (forall ((c Bool)
;           (ptr-then PointerValue)
;           (ptr-else PointerValue)
;           (ptr-joined PointerValue))
;    (= (fc-conj--join-ptr c ptr-then ptr-else ptr-joined)
;       TODO))))

;(assert (=> enable-solution-s2
;  (forall ((fc Bool) (p PointerValue))
;    (= (is-unsafe-to-deref fc p)
;       TODO))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Check the solution.

(echo "Checking satisfiability of the selected solution.")
(check-sat)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Properties that the solution must satisfy.

(define-fun is-valid-pointer ((p PointerValue)) Bool
  (or (fc-conj--ptr-is-null p)
      (fc-conj--ptr-is-unknown p)
      (fc-conj--ptr-is-nonnull p)
      (fc-conj--ptr-is-nullable p)))

;; Reflexivity for fc-conj--ptrs-were-compared.
(assert
  (forall ((p PointerValue))
    (=> (is-valid-pointer p)
        (fc-conj--ptrs-were-compared p p true))))

;; Symmetry for fc-conj--ptrs-were-compared.
(assert
  (forall ((p1 PointerValue) (p2 PointerValue) (are-equal Bool))
    (=> (and (is-valid-pointer p1) (is-valid-pointer p2))
        (= (fc-conj--ptrs-were-compared p1 p2 are-equal)
           (fc-conj--ptrs-were-compared p2 p1 are-equal)))))

;; Transitivity for fc-conj--ptrs-were-compared.
(assert
  (=>
    ;; Transitivity does not hold in S1.
    ;; Consider (nonnull == unknown), (unknown == null).
    ;; However (nonnull == null) never holds.
    ;; ```
    ;; void target(int * _NonNull x, int *y) {
    ;;   if (x == y && y == nullptr) {
    ;;     // dead code that we can't detect
    ;;   }
    ;; }
    ;; ```
    ;; TODO: How big of a problem is it?
    (not enable-solution-s1)
    (forall ((p1 PointerValue) (p2 PointerValue) (p3 PointerValue))
      (=> (and (is-valid-pointer p1)
               (is-valid-pointer p2)
               (is-valid-pointer p3)
               (fc-conj--ptrs-were-compared p1 p2 true)
               (fc-conj--ptrs-were-compared p2 p3 true))
          (fc-conj--ptrs-were-compared p1 p3 true)))))

;; Symmetry for fc-conj--join-ptr with regards to pointers.
(assert
  (forall ((b Bool) (p1 PointerValue) (p2 PointerValue) (p3 PointerValue))
    (= (fc-conj--join-ptr b p1 p2 p3)
       (fc-conj--join-ptr (not b) p2 p1 p3))))

(echo "Checking whether the selected solution satisfies the properties.")
(check-sat)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example DerefAllUnchecked
;;
;; ```
;; void target(
;;     int *ptr_unknown,
;;     int * _NonNull ptr_nonnull,
;;     int * _Nullable ptr_nullable) {
;;   int *ptr_nullptr = nullptr;
;;   // (1)
;;   *ptr_unknown; // safe
;;   *ptr_nonnull; // safe
;;   *ptr_nullable; // unsafe
;;   *ptr_nullptr; // unsafe
;; }
;; ```

(declare-datatype State-DerefAllUnchecked
  ((make-State-DerefAllUnchecked
    (_get-ptr_unknown PointerValue)
    (_get-ptr_nonnull PointerValue)
    (_get-ptr_nullable PointerValue)
    (_get-ptr_nullptr PointerValue))))

(define-fun get-ptr_unknown-DerefAllUnchecked
  ((state State-DerefAllUnchecked)) PointerValue
  (_get-ptr_unknown state))

(define-fun get-ptr_nonnull-DerefAllUnchecked
  ((state State-DerefAllUnchecked)) PointerValue
  (_get-ptr_nonnull state))

(define-fun get-ptr_nullable-DerefAllUnchecked
  ((state State-DerefAllUnchecked)) PointerValue
  (_get-ptr_nullable state))

(define-fun get-ptr_nullptr-DerefAllUnchecked
  ((state State-DerefAllUnchecked)) PointerValue
  (_get-ptr_nullptr state))

(define-fun run-DerefAllUnchecked
  ((state State-DerefAllUnchecked))
  FlowConditions
  (match state
    (((make-State-DerefAllUnchecked
           ptr_unknown ptr_nonnull ptr_nullable ptr_nullptr)
      (let ((fc-1 (and (fc-conj--ptr-is-unknown ptr_unknown)
                       (fc-conj--ptr-is-nonnull ptr_nonnull)
                       (fc-conj--ptr-is-nullable ptr_nullable)
                       (fc-conj--ptr-is-null ptr_nullptr))))
        (make-FlowConditions fc-1 false false false false false))))))

(define-fun is-reachable-DerefAllUnchecked
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-DerefAllUnchecked))
    (let ((fcs (run-DerefAllUnchecked state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-DerefAllUnchecked
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-DerefAllUnchecked PointerValue)))
  Bool
  (exists ((state State-DerefAllUnchecked))
    (let ((fcs (run-DerefAllUnchecked state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert
  (! (is-reachable-DerefAllUnchecked get-fc-1)
     :named DerefAllUnchecked-reachable-1))

(assert
  (! (not (is-unsafe-deref-DerefAllUnchecked get-fc-1 get-ptr_unknown-DerefAllUnchecked))
     :named DerefAllUnchecked-deref-unknown))

(assert
  (! (not (is-unsafe-deref-DerefAllUnchecked get-fc-1 get-ptr_nonnull-DerefAllUnchecked))
     :named DerefAllUnchecked-deref-nonnull))

(assert
  (! (is-unsafe-deref-DerefAllUnchecked get-fc-1 get-ptr_nullable-DerefAllUnchecked)
     :named DerefAllUnchecked-deref-nullable))

(assert
  (! (is-unsafe-deref-DerefAllUnchecked get-fc-1 get-ptr_nullptr-DerefAllUnchecked)
     :named DerefAllUnchecked-deref-nullptr))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example MixedNullableAndNonNull
;;
;; ```
;; void target(int * _Nullable x, bool b) {
;;   // (1)
;;   int i;
;;   *x; // unsafe
;;   if (b) {
;;     // (2)
;;     *x; // unsafe
;;     x = &i;
;;     // (3)
;;     *x; // safe
;;   }
;;   // (4)
;;   *x; // unsafe
;;   if (b) {
;;     // (5)
;;     *x; // safe
;;   } else {
;;     // (6)
;;     *x; // unsafe
;;   }
;; }
;; ```

(declare-datatype State-MixedNullableAndNonNull
  ((make-State-MixedNullableAndNonNull
    (_get-x-1 PointerValue)
    (_get-x-3 PointerValue)
    (_get-x-4 PointerValue)
    (b Bool))))

(define-fun get-x-1-MixedNullableAndNonNull
  ((state State-MixedNullableAndNonNull)) PointerValue
  (_get-x-1 state))

(define-fun get-x-3-MixedNullableAndNonNull
  ((state State-MixedNullableAndNonNull)) PointerValue
  (_get-x-3 state))

(define-fun get-x-4-MixedNullableAndNonNull
  ((state State-MixedNullableAndNonNull)) PointerValue
  (_get-x-4 state))

(define-fun run-MixedNullableAndNonNull
  ((state State-MixedNullableAndNonNull))
  FlowConditions
  (match state
    (((make-State-MixedNullableAndNonNull x-1 x-3 x-4 b)
      (let ((fc-1 (fc-conj--ptr-is-nullable x-1)))
      (let ((fc-2 (and fc-1 b)))
      (let ((fc-3 (and fc-2 (fc-conj--ptr-is-nonnull x-3))))
      (let ((fc-4 (and (join-fc b fc-3 fc-1)
                       (fc-conj--join-ptr b x-3 x-1 x-4))))
      (let ((fc-5 (and fc-4 b)))
      (let ((fc-6 (and fc-4 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 fc-5 fc-6)))))))))))

(define-fun is-reachable-MixedNullableAndNonNull
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-MixedNullableAndNonNull))
    (let ((fcs (run-MixedNullableAndNonNull state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-MixedNullableAndNonNull
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-MixedNullableAndNonNull PointerValue)))
  Bool
  (exists ((state State-MixedNullableAndNonNull))
    (let ((fcs (run-MixedNullableAndNonNull state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert
  (! (is-reachable-MixedNullableAndNonNull get-fc-1)
     :named MixedNullableAndNonNull-reachable-1))

(assert
  (! (is-reachable-MixedNullableAndNonNull get-fc-2)
     :named MixedNullableAndNonNull-reachable-2))

(assert
  (! (is-reachable-MixedNullableAndNonNull get-fc-3)
     :named MixedNullableAndNonNull-reachable-3))

(assert
  (! (is-reachable-MixedNullableAndNonNull get-fc-4)
     :named MixedNullableAndNonNull-reachable-4))

(assert
  (! (is-reachable-MixedNullableAndNonNull get-fc-5)
     :named MixedNullableAndNonNull-reachable-5))

(assert
  (! (is-reachable-MixedNullableAndNonNull get-fc-6)
     :named MixedNullableAndNonNull-reachable-6))

(assert
  (! (is-unsafe-deref-MixedNullableAndNonNull get-fc-1 get-x-1-MixedNullableAndNonNull)
     :named MixedNullableAndNonNull-deref-1))

(assert
  (! (is-unsafe-deref-MixedNullableAndNonNull get-fc-2 get-x-1-MixedNullableAndNonNull)
     :named MixedNullableAndNonNull-deref-2))

(assert
  (! (not (is-unsafe-deref-MixedNullableAndNonNull get-fc-3 get-x-3-MixedNullableAndNonNull))
     :named MixedNullableAndNonNull-deref-3))

(assert
  (! (is-unsafe-deref-MixedNullableAndNonNull get-fc-4 get-x-4-MixedNullableAndNonNull)
     :named MixedNullableAndNonNull-deref-4))

(assert
  (! (not (is-unsafe-deref-MixedNullableAndNonNull get-fc-5 get-x-4-MixedNullableAndNonNull))
     :named MixedNullableAndNonNull-deref-5))

(assert
  (! (is-unsafe-deref-MixedNullableAndNonNull get-fc-6 get-x-4-MixedNullableAndNonNull)
     :named MixedNullableAndNonNull-deref-6))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example MixedNullableAndUnknown
;;
;; ```
;; void target(int * _Nullable x, bool b) {
;;   // (1)
;;   *x; // unsafe
;;   if (b) {
;;     // (2)
;;     *x; // unsafe
;;     x = MakeUnknown();
;;     // (3)
;;     *x; // safe
;;   }
;;   // (4)
;;   *x; // unsafe
;;   if (b) {
;;     // (5)
;;     *x; // safe
;;   } else {
;;     // (6)
;;     *x; // unsafe
;;   }
;; }
;; ```

(declare-datatype State-MixedNullableAndUnknown
  ((make-State-MixedNullableAndUnknown
    (_get-x-1 PointerValue)
    (_get-x-3 PointerValue)
    (_get-x-4 PointerValue)
    (b Bool))))

(define-fun get-x-1-MixedNullableAndUnknown
  ((state State-MixedNullableAndUnknown)) PointerValue
  (_get-x-1 state))

(define-fun get-x-3-MixedNullableAndUnknown
  ((state State-MixedNullableAndUnknown)) PointerValue
  (_get-x-3 state))

(define-fun get-x-4-MixedNullableAndUnknown
  ((state State-MixedNullableAndUnknown)) PointerValue
  (_get-x-4 state))

(define-fun run-MixedNullableAndUnknown
  ((state State-MixedNullableAndUnknown))
  FlowConditions
  (match state
    (((make-State-MixedNullableAndUnknown x-1 x-3 x-4 b)
      (let ((fc-1 (fc-conj--ptr-is-nullable x-1)))
      (let ((fc-2 (and fc-1 b)))
      (let ((fc-3 (and fc-2 (fc-conj--ptr-is-unknown x-3))))
      (let ((fc-4 (and (join-fc b fc-3 fc-1)
                       (fc-conj--join-ptr b x-3 x-1 x-4))))
      (let ((fc-5 (and fc-4 b)))
      (let ((fc-6 (and fc-4 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 fc-5 fc-6)))))))))))

(define-fun is-reachable-MixedNullableAndUnknown
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-MixedNullableAndUnknown))
    (let ((fcs (run-MixedNullableAndUnknown state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-MixedNullableAndUnknown
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-MixedNullableAndUnknown PointerValue)))
  Bool
  (exists ((state State-MixedNullableAndUnknown))
    (let ((fcs (run-MixedNullableAndUnknown state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert
  (! (is-reachable-MixedNullableAndUnknown get-fc-1)
     :named MixedNullableAndUnknown-reachable-1))

(assert
  (! (is-reachable-MixedNullableAndUnknown get-fc-2)
     :named MixedNullableAndUnknown-reachable-2))

(assert
  (! (is-reachable-MixedNullableAndUnknown get-fc-3)
     :named MixedNullableAndUnknown-reachable-3))

(assert
  (! (is-reachable-MixedNullableAndUnknown get-fc-4)
     :named MixedNullableAndUnknown-reachable-4))

(assert
  (! (is-reachable-MixedNullableAndUnknown get-fc-5)
     :named MixedNullableAndUnknown-reachable-5))

(assert
  (! (is-reachable-MixedNullableAndUnknown get-fc-6)
     :named MixedNullableAndUnknown-reachable-6))

(assert
  (! (is-unsafe-deref-MixedNullableAndUnknown get-fc-1 get-x-1-MixedNullableAndUnknown)
     :named MixedNullableAndUnknown-deref-1))

(assert
  (! (is-unsafe-deref-MixedNullableAndUnknown get-fc-2 get-x-1-MixedNullableAndUnknown)
     :named MixedNullableAndUnknown-deref-2))

(assert
  (! (not (is-unsafe-deref-MixedNullableAndUnknown get-fc-3 get-x-3-MixedNullableAndUnknown))
     :named MixedNullableAndUnknown-deref-3))

(assert
  (! (is-unsafe-deref-MixedNullableAndUnknown get-fc-4 get-x-4-MixedNullableAndUnknown)
     :named MixedNullableAndUnknown-deref-4))

(assert
  (! (not (is-unsafe-deref-MixedNullableAndUnknown get-fc-5 get-x-4-MixedNullableAndUnknown))
     :named MixedNullableAndUnknown-deref-5))

(assert
  (! (is-unsafe-deref-MixedNullableAndUnknown get-fc-6 get-x-4-MixedNullableAndUnknown)
     :named MixedNullableAndUnknown-deref-6))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example MixedUnknownAndNull
;;
;; ```
;; void target(int *x, bool b) {
;;   // (1)
;;   *x; // safe
;;   if (b) {
;;     // (2)
;;     *x; // safe
;;     x = nullptr;
;;     // (3)
;;     *x; // unsafe
;;   }
;;   // (4)
;;   *x; // unsafe
;;   if (b) {
;;     // (5)
;;     *x; // unsafe
;;   } else {
;;     // (6)
;;     *x; // safe
;;   }
;; }
;; ```

(declare-datatype State-MixedUnknownAndNull
  ((make-State-MixedUnknownAndNull
    (_get-x-1 PointerValue)
    (_get-x-3 PointerValue)
    (_get-x-4 PointerValue)
    (b Bool))))

(define-fun get-x-1-MixedUnknownAndNull
  ((state State-MixedUnknownAndNull)) PointerValue
  (_get-x-1 state))

(define-fun get-x-3-MixedUnknownAndNull
  ((state State-MixedUnknownAndNull)) PointerValue
  (_get-x-3 state))

(define-fun get-x-4-MixedUnknownAndNull
  ((state State-MixedUnknownAndNull)) PointerValue
  (_get-x-4 state))

(define-fun run-MixedUnknownAndNull
  ((state State-MixedUnknownAndNull))
  FlowConditions
  (match state
    (((make-State-MixedUnknownAndNull x-1 x-3 x-4 b)
      (let ((fc-1 (fc-conj--ptr-is-unknown x-1)))
      (let ((fc-2 (and fc-1 b)))
      (let ((fc-3 (and fc-2 (fc-conj--ptr-is-null x-3))))
      (let ((fc-4 (and (join-fc b fc-3 fc-1)
                       (fc-conj--join-ptr b x-3 x-1 x-4))))
      (let ((fc-5 (and fc-4 b)))
      (let ((fc-6 (and fc-4 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 fc-5 fc-6)))))))))))

(define-fun is-reachable-MixedUnknownAndNull
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-MixedUnknownAndNull))
    (let ((fcs (run-MixedUnknownAndNull state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-MixedUnknownAndNull
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-MixedUnknownAndNull PointerValue)))
  Bool
  (exists ((state State-MixedUnknownAndNull))
    (let ((fcs (run-MixedUnknownAndNull state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert
  (! (is-reachable-MixedUnknownAndNull get-fc-1)
     :named MixedUnknownAndNull-reachable-1))

(assert
  (! (is-reachable-MixedUnknownAndNull get-fc-2)
     :named MixedUnknownAndNull-reachable-2))

(assert
  (! (is-reachable-MixedUnknownAndNull get-fc-3)
     :named MixedUnknownAndNull-reachable-3))

(assert
  (! (is-reachable-MixedUnknownAndNull get-fc-4)
     :named MixedUnknownAndNull-reachable-4))

(assert
  (! (is-reachable-MixedUnknownAndNull get-fc-5)
     :named MixedUnknownAndNull-reachable-5))

(assert
  (! (is-reachable-MixedUnknownAndNull get-fc-6)
     :named MixedUnknownAndNull-reachable-6))

(assert
  (! (not (is-unsafe-deref-MixedUnknownAndNull get-fc-1 get-x-1-MixedUnknownAndNull))
     :named MixedUnknownAndNull-deref-1))

(assert
  (! (not (is-unsafe-deref-MixedUnknownAndNull get-fc-2 get-x-1-MixedUnknownAndNull))
     :named MixedUnknownAndNull-deref-2))

(assert
  (! (is-unsafe-deref-MixedUnknownAndNull get-fc-3 get-x-3-MixedUnknownAndNull)
     :named MixedUnknownAndNull-deref-3))

(assert
  (! (is-unsafe-deref-MixedUnknownAndNull get-fc-4 get-x-4-MixedUnknownAndNull)
     :named MixedUnknownAndNull-deref-4))

(assert
  (! (is-unsafe-deref-MixedUnknownAndNull get-fc-5 get-x-4-MixedUnknownAndNull)
     :named MixedUnknownAndNull-deref-5))

(assert
  (! (not (is-unsafe-deref-MixedUnknownAndNull get-fc-6 get-x-4-MixedUnknownAndNull))
     :named MixedUnknownAndNull-deref-6))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example MixedUnknownAndNonNull
;;
;; ```
;; void target(int *x, bool b) {
;;   // (1)
;;   *x; // safe
;;   if (b) {
;;     // (2)
;;     *x; // safe
;;     x = MakeNonNull();
;;     // (3)
;;     *x; // safe
;;   }
;;   // (4)
;;   *x; // safe
;;   if (b) {
;;     // (5)
;;     *x; // safe
;;   } else {
;;     // (6)
;;     *x; // safe
;;   }
;; }
;; ```

(declare-datatype State-MixedUnknownAndNonNull
  ((make-State-MixedUnknownAndNonNull
    (_get-x-1 PointerValue)
    (_get-x-3 PointerValue)
    (_get-x-4 PointerValue)
    (b Bool))))

(define-fun get-x-1-MixedUnknownAndNonNull
  ((state State-MixedUnknownAndNonNull)) PointerValue
  (_get-x-1 state))

(define-fun get-x-3-MixedUnknownAndNonNull
  ((state State-MixedUnknownAndNonNull)) PointerValue
  (_get-x-3 state))

(define-fun get-x-4-MixedUnknownAndNonNull
  ((state State-MixedUnknownAndNonNull)) PointerValue
  (_get-x-4 state))

(define-fun run-MixedUnknownAndNonNull
  ((state State-MixedUnknownAndNonNull))
  FlowConditions
  (match state
    (((make-State-MixedUnknownAndNonNull x-1 x-3 x-4 b)
      (let ((fc-1 (fc-conj--ptr-is-unknown x-1)))
      (let ((fc-2 (and fc-1 b)))
      (let ((fc-3 (and fc-2 (fc-conj--ptr-is-nonnull x-3))))
      (let ((fc-4 (and (join-fc b fc-3 fc-1)
                       (fc-conj--join-ptr b x-3 x-1 x-4))))
      (let ((fc-5 (and fc-4 b)))
      (let ((fc-6 (and fc-4 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 fc-5 fc-6)))))))))))

(define-fun is-reachable-MixedUnknownAndNonNull
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-MixedUnknownAndNonNull))
    (let ((fcs (run-MixedUnknownAndNonNull state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-MixedUnknownAndNonNull
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-MixedUnknownAndNonNull PointerValue)))
  Bool
  (exists ((state State-MixedUnknownAndNonNull))
    (let ((fcs (run-MixedUnknownAndNonNull state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert
  (! (is-reachable-MixedUnknownAndNonNull get-fc-1)
     :named MixedUnknownAndNonNull-reachable-1))

(assert
  (! (is-reachable-MixedUnknownAndNonNull get-fc-2)
     :named MixedUnknownAndNonNull-reachable-2))

(assert
  (! (is-reachable-MixedUnknownAndNonNull get-fc-3)
     :named MixedUnknownAndNonNull-reachable-3))

(assert
  (! (is-reachable-MixedUnknownAndNonNull get-fc-4)
     :named MixedUnknownAndNonNull-reachable-4))

(assert
  (! (is-reachable-MixedUnknownAndNonNull get-fc-5)
     :named MixedUnknownAndNonNull-reachable-5))

(assert
  (! (is-reachable-MixedUnknownAndNonNull get-fc-6)
     :named MixedUnknownAndNonNull-reachable-6))

(assert
  (! (not (is-unsafe-deref-MixedUnknownAndNonNull get-fc-1 get-x-1-MixedUnknownAndNonNull))
     :named MixedUnknownAndNonNull-deref-1))

(assert
  (! (not (is-unsafe-deref-MixedUnknownAndNonNull get-fc-2 get-x-1-MixedUnknownAndNonNull))
     :named MixedUnknownAndNonNull-deref-2))

(assert
  (! (not (is-unsafe-deref-MixedUnknownAndNonNull get-fc-3 get-x-3-MixedUnknownAndNonNull))
     :named MixedUnknownAndNonNull-deref-3))

(assert
  (! (not (is-unsafe-deref-MixedUnknownAndNonNull get-fc-4 get-x-4-MixedUnknownAndNonNull))
     :named MixedUnknownAndNonNull-deref-4))

(assert
  (! (not (is-unsafe-deref-MixedUnknownAndNonNull get-fc-5 get-x-4-MixedUnknownAndNonNull))
     :named MixedUnknownAndNonNull-deref-5))

(assert
  (! (not (is-unsafe-deref-MixedUnknownAndNonNull get-fc-6 get-x-4-MixedUnknownAndNonNull))
     :named MixedUnknownAndNonNull-deref-6))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example CompareNullAndNull
;;
;; ```
;; void target() {
;;   int *x = nullptr;
;;   int *y = nullptr;
;;   // (1)
;;   bool b = x == y;
;;   // (2)
;;   if (b) {
;;     // (3)
;;     *x; // unsafe
;;     *y; // unsafe
;;   } else {
;;     // (4) - unreachable
;;   }
;; }
;; ```

(declare-datatype State-CompareNullAndNull
  ((make-State-CompareNullAndNull
    (_get-x PointerValue)
    (_get-y PointerValue)
    (_get-b Bool))))

(define-fun get-x-CompareNullAndNull
  ((state State-CompareNullAndNull)) PointerValue
  (_get-x state))

(define-fun get-y-CompareNullAndNull
  ((state State-CompareNullAndNull)) PointerValue
  (_get-y state))

(define-fun run-CompareNullAndNull
  ((state State-CompareNullAndNull))
  FlowConditions
  (match state
    (((make-State-CompareNullAndNull x y b)
      (let ((fc-1 (and (fc-conj--ptr-is-null x)
                       (fc-conj--ptr-is-null y))))
      (let ((fc-2 (and fc-1 (fc-conj--ptrs-were-compared x y b))))
      (let ((fc-3 (and fc-2 b)))
      (let ((fc-4 (and fc-2 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 false false)))))))))

(define-fun is-reachable-CompareNullAndNull
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-CompareNullAndNull))
    (let ((fcs (run-CompareNullAndNull state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-CompareNullAndNull
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-CompareNullAndNull PointerValue)))
  Bool
  (exists ((state State-CompareNullAndNull))
    (let ((fcs (run-CompareNullAndNull state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert
  (! (is-reachable-CompareNullAndNull get-fc-1)
     :named CompareNullAndNull-reachable-1))

(assert
  (! (is-reachable-CompareNullAndNull get-fc-2)
     :named CompareNullAndNull-reachable-2))

(assert
  (! (is-reachable-CompareNullAndNull get-fc-3)
     :named CompareNullAndNull-reachable-3))

(assert
  (! (not (is-reachable-CompareNullAndNull get-fc-4))
     :named CompareNullAndNull-reachable-4))

(assert
  (! (is-unsafe-deref-CompareNullAndNull get-fc-3 get-x-CompareNullAndNull)
     :named CompareNullAndNull-deref-3-x))

(assert
  (! (is-unsafe-deref-CompareNullAndNull get-fc-3 get-y-CompareNullAndNull)
     :named CompareNullAndNull-deref-3-y))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example CompareUnknownAndUnknown
;;
;; ```
;; void target(int *x, int *y) {
;;   // (1)
;;   bool b = x == y;
;;   // (2)
;;   if (b) {
;;     // (3)
;;     *x; // safe
;;     *y; // safe
;;   } else {
;;     // (4)
;;     *x; // safe
;;     *y; // safe
;;   }
;; }
;; ```

(declare-datatype State-CompareUnknownAndUnknown
  ((make-State-CompareUnknownAndUnknown
    (_get-x PointerValue)
    (_get-y PointerValue)
    (_get-b Bool))))

(define-fun get-x-CompareUnknownAndUnknown
  ((state State-CompareUnknownAndUnknown)) PointerValue
  (_get-x state))

(define-fun get-y-CompareUnknownAndUnknown
  ((state State-CompareUnknownAndUnknown)) PointerValue
  (_get-y state))

(define-fun run-CompareUnknownAndUnknown
  ((state State-CompareUnknownAndUnknown))
  FlowConditions
  (match state
    (((make-State-CompareUnknownAndUnknown x y b)
      (let ((fc-1 (and (fc-conj--ptr-is-unknown x)
                       (fc-conj--ptr-is-unknown y))))
      (let ((fc-2 (and fc-1 (fc-conj--ptrs-were-compared x y b))))
      (let ((fc-3 (and fc-2 b)))
      (let ((fc-4 (and fc-2 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 false false)))))))))

(define-fun is-reachable-CompareUnknownAndUnknown
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-CompareUnknownAndUnknown))
    (let ((fcs (run-CompareUnknownAndUnknown state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-CompareUnknownAndUnknown
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-CompareUnknownAndUnknown PointerValue)))
  Bool
  (exists ((state State-CompareUnknownAndUnknown))
    (let ((fcs (run-CompareUnknownAndUnknown state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert
  (! (is-reachable-CompareUnknownAndUnknown get-fc-1)
     :named CompareUnknownAndUnknown-reachable-1))

(assert
  (! (is-reachable-CompareUnknownAndUnknown get-fc-2)
     :named CompareUnknownAndUnknown-reachable-2))

(assert
  (! (is-reachable-CompareUnknownAndUnknown get-fc-3)
     :named CompareUnknownAndUnknown-reachable-3))

(assert
  (! (is-reachable-CompareUnknownAndUnknown get-fc-4)
     :named CompareUnknownAndUnknown-reachable-4))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndUnknown get-fc-3 get-x-CompareUnknownAndUnknown))
     :named CompareUnknownAndUnknown-deref-3-x))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndUnknown get-fc-3 get-y-CompareUnknownAndUnknown))
     :named CompareUnknownAndUnknown-deref-3-y))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndUnknown get-fc-4 get-x-CompareUnknownAndUnknown))
     :named CompareUnknownAndUnknown-deref-4-x))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndUnknown get-fc-4 get-y-CompareUnknownAndUnknown))
     :named CompareUnknownAndUnknown-deref-4-y))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example CompareNonNullAndNonNull
;;
;; ```
;; void target(int * _NonNull x, int * _NonNull y) {
;;   // (1)
;;   bool b = x == y;
;;   // (2)
;;   if (b) {
;;     // (3)
;;     *x; // safe
;;     *y; // safe
;;   } else {
;;     // (4)
;;     *x; // safe
;;     *y; // safe
;;   }
;; }
;; ```

(declare-datatype State-CompareNonNullAndNonNull
  ((make-State-CompareNonNullAndNonNull
    (_get-x PointerValue)
    (_get-y PointerValue)
    (_get-b Bool))))

(define-fun get-x-CompareNonNullAndNonNull
  ((state State-CompareNonNullAndNonNull)) PointerValue
  (_get-x state))

(define-fun get-y-CompareNonNullAndNonNull
  ((state State-CompareNonNullAndNonNull)) PointerValue
  (_get-y state))

(define-fun run-CompareNonNullAndNonNull
  ((state State-CompareNonNullAndNonNull))
  FlowConditions
  (match state
    (((make-State-CompareNonNullAndNonNull x y b)
      (let ((fc-1 (and (fc-conj--ptr-is-nonnull x)
                       (fc-conj--ptr-is-nonnull y))))
      (let ((fc-2 (and fc-1 (fc-conj--ptrs-were-compared x y b))))
      (let ((fc-3 (and fc-2 b)))
      (let ((fc-4 (and fc-2 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 false false)))))))))

(define-fun is-reachable-CompareNonNullAndNonNull
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-CompareNonNullAndNonNull))
    (let ((fcs (run-CompareNonNullAndNonNull state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-CompareNonNullAndNonNull
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-CompareNonNullAndNonNull PointerValue)))
  Bool
  (exists ((state State-CompareNonNullAndNonNull))
    (let ((fcs (run-CompareNonNullAndNonNull state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert
  (! (is-reachable-CompareNonNullAndNonNull get-fc-1)
     :named CompareNonNullAndNonNull-reachable-1))

(assert
  (! (is-reachable-CompareNonNullAndNonNull get-fc-2)
     :named CompareNonNullAndNonNull-reachable-2))

(assert
  (! (is-reachable-CompareNonNullAndNonNull get-fc-3)
     :named CompareNonNullAndNonNull-reachable-3))

(assert
  (! (is-reachable-CompareNonNullAndNonNull get-fc-4)
     :named CompareNonNullAndNonNull-reachable-4))

(assert
  (! (not (is-unsafe-deref-CompareNonNullAndNonNull get-fc-3 get-x-CompareNonNullAndNonNull))
     :named CompareNonNullAndNonNull-deref-3-x))

(assert
  (! (not (is-unsafe-deref-CompareNonNullAndNonNull get-fc-3 get-y-CompareNonNullAndNonNull))
     :named CompareNonNullAndNonNull-deref-3-y))

(assert
  (! (not (is-unsafe-deref-CompareNonNullAndNonNull get-fc-4 get-x-CompareNonNullAndNonNull))
     :named CompareNonNullAndNonNull-deref-4-x))

(assert
  (! (not (is-unsafe-deref-CompareNonNullAndNonNull get-fc-4 get-y-CompareNonNullAndNonNull))
     :named CompareNonNullAndNonNull-deref-4-y))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example CompareNonNullAndNull
;;
;; ```
;; void target(int * _NonNull x) {
;;   // (1)
;;   *x; // safe
;;   bool b = x == nullptr;
;;   // (2)
;;   *x; // safe
;;   if (b) {
;;     // (3) - unreachable
;;   } else {
;;     // (4)
;;     *x; // safe
;;   }
;; }
;; ```

(declare-datatype State-CompareNonNullAndNull
  ((make-State-CompareNonNullAndNull
    (_get-the-nullptr PointerValue)
    (_get-x PointerValue)
    (_get-b Bool))))

(define-fun get-x-CompareNonNullAndNull
  ((state State-CompareNonNullAndNull)) PointerValue
  (_get-x state))

(define-fun run-CompareNonNullAndNull
  ((state State-CompareNonNullAndNull))
  FlowConditions
  (match state
    (((make-State-CompareNonNullAndNull the-nullptr x b)
      (let ((fc-1 (and (fc-conj--ptr-is-null the-nullptr)
                       (fc-conj--ptr-is-nonnull x))))
      (let ((fc-2 (and fc-1 (fc-conj--ptrs-were-compared x the-nullptr b))))
      (let ((fc-3 (and fc-2 b)))
      (let ((fc-4 (and fc-2 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 false false)))))))))

(define-fun is-reachable-CompareNonNullAndNull
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-CompareNonNullAndNull))
    (let ((fcs (run-CompareNonNullAndNull state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-CompareNonNullAndNull
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-CompareNonNullAndNull PointerValue)))
  Bool
  (exists ((state State-CompareNonNullAndNull))
    (let ((fcs (run-CompareNonNullAndNull state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert
  (! (is-reachable-CompareNonNullAndNull get-fc-1)
     :named CompareNonNullAndNull-reachable-1))

(assert
  (! (is-reachable-CompareNonNullAndNull get-fc-2)
     :named CompareNonNullAndNull-reachable-2))

(assert
  (! (not (is-reachable-CompareNonNullAndNull get-fc-3))
     :named CompareNonNullAndNull-reachable-3))

(assert
  (! (is-reachable-CompareNonNullAndNull get-fc-4)
     :named CompareNonNullAndNull-reachable-4))

(assert
  (! (not (is-unsafe-deref-CompareNonNullAndNull get-fc-1 get-x-CompareNonNullAndNull))
     :named CompareNonNullAndNull-deref-1))

(assert
  (! (not (is-unsafe-deref-CompareNonNullAndNull get-fc-2 get-x-CompareNonNullAndNull))
     :named CompareNonNullAndNull-deref-2))

(assert
  (! (not (is-unsafe-deref-CompareNonNullAndNull get-fc-4 get-x-CompareNonNullAndNull))
     :named CompareNonNullAndNull-deref-4))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example CompareNullableAndNull
;;
;; ```
;; void target(int * _Nullable x) {
;;   // (1)
;;   *x; // unsafe
;;   bool b = x == nullptr;
;;   // (2)
;;   *x; // unsafe
;;   if (b) {
;;     // (3)
;;     *x; // unsafe
;;   } else {
;;     // (4)
;;     *x; // safe
;;   }
;; }
;; ```

(declare-datatype State-CompareNullableAndNull
  ((make-State-CompareNullableAndNull
    (_get-the-nullptr PointerValue)
    (_get-x PointerValue)
    (_get-b Bool))))

(define-fun get-x-CompareNullableAndNull
  ((state State-CompareNullableAndNull)) PointerValue
  (_get-x state))

(define-fun run-CompareNullableAndNull
  ((state State-CompareNullableAndNull))
  FlowConditions
  (match state
    (((make-State-CompareNullableAndNull the-nullptr x b)
      (let ((fc-1 (and (fc-conj--ptr-is-null the-nullptr)
                       (fc-conj--ptr-is-nullable x))))
      (let ((fc-2 (and fc-1 (fc-conj--ptrs-were-compared x the-nullptr b))))
      (let ((fc-3 (and fc-2 b)))
      (let ((fc-4 (and fc-2 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 false false)))))))))

(define-fun is-reachable-CompareNullableAndNull
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-CompareNullableAndNull))
    (let ((fcs (run-CompareNullableAndNull state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-CompareNullableAndNull
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-CompareNullableAndNull PointerValue)))
  Bool
  (exists ((state State-CompareNullableAndNull))
    (let ((fcs (run-CompareNullableAndNull state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert
  (! (is-reachable-CompareNullableAndNull get-fc-1)
     :named CompareNullableAndNull-reachable-1))

(assert
  (! (is-reachable-CompareNullableAndNull get-fc-2)
     :named CompareNullableAndNull-reachable-2))

(assert
  (! (is-reachable-CompareNullableAndNull get-fc-3)
     :named CompareNullableAndNull-reachable-3))

(assert
  (! (is-reachable-CompareNullableAndNull get-fc-4)
     :named CompareNullableAndNull-reachable-4))

(assert
  (! (is-unsafe-deref-CompareNullableAndNull get-fc-1 get-x-CompareNullableAndNull)
     :named CompareNullableAndNull-deref-1))

(assert
  (! (is-unsafe-deref-CompareNullableAndNull get-fc-2 get-x-CompareNullableAndNull)
     :named CompareNullableAndNull-deref-2))

(assert
  (! (is-unsafe-deref-CompareNullableAndNull get-fc-3 get-x-CompareNullableAndNull)
     :named CompareNullableAndNull-deref-3))

(assert
  (! (not (is-unsafe-deref-CompareNullableAndNull get-fc-4 get-x-CompareNullableAndNull))
     :named CompareNullableAndNull-deref-4))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example CompareUnknownAndNullSimple
;;
;; ```
;; void target(int *x) {
;;   // (1)
;;   *x; // safe - false negative
;;   bool b = x == nullptr;
;;   // (2)
;;   *x; // safe - false negative
;;   if (b) {
;;     // (3)
;;     *x; // safe - false negative
;;   } else {
;;     // (4)
;;     *x; // safe
;;   }
;; }
;; ```

(declare-datatype State-CompareUnknownAndNullSimple
  ((make-State-CompareUnknownAndNullSimple
    (_get-the-nullptr PointerValue)
    (_get-x PointerValue)
    (_get-b Bool))))

(define-fun get-x-CompareUnknownAndNullSimple
  ((state State-CompareUnknownAndNullSimple)) PointerValue
  (_get-x state))

(define-fun run-CompareUnknownAndNullSimple
  ((state State-CompareUnknownAndNullSimple))
  FlowConditions
  (match state
    (((make-State-CompareUnknownAndNullSimple the-nullptr x b)
      (let ((fc-1 (and (fc-conj--ptr-is-null the-nullptr)
                       (fc-conj--ptr-is-unknown x))))
      (let ((fc-2 (and fc-1 (fc-conj--ptrs-were-compared x the-nullptr b))))
      (let ((fc-3 (and fc-2 b)))
      (let ((fc-4 (and fc-2 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 false false)))))))))

(define-fun is-reachable-CompareUnknownAndNullSimple
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-CompareUnknownAndNullSimple))
    (let ((fcs (run-CompareUnknownAndNullSimple state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-CompareUnknownAndNullSimple
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-CompareUnknownAndNullSimple PointerValue)))
  Bool
  (exists ((state State-CompareUnknownAndNullSimple))
    (let ((fcs (run-CompareUnknownAndNullSimple state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert (=> enable-solution-s1
  (! (is-reachable-CompareUnknownAndNullSimple get-fc-1)
     :named CompareUnknownAndNullSimple-reachable-1)))

(assert (=> enable-solution-s1
  (! (is-reachable-CompareUnknownAndNullSimple get-fc-2)
     :named CompareUnknownAndNullSimple-reachable-2)))

(assert (=> enable-solution-s1
  (! (is-reachable-CompareUnknownAndNullSimple get-fc-3)
     :named CompareUnknownAndNullSimple-reachable-3)))

(assert (=> enable-solution-s1
  (! (is-reachable-CompareUnknownAndNullSimple get-fc-4)
     :named CompareUnknownAndNullSimple-reachable-4)))

(assert (=> enable-solution-s1
  (! (not (is-unsafe-deref-CompareUnknownAndNullSimple get-fc-1 get-x-CompareUnknownAndNullSimple))
     :named CompareUnknownAndNullSimple-deref-1)))

(assert (=> enable-solution-s1
  (! (not (is-unsafe-deref-CompareUnknownAndNullSimple get-fc-2 get-x-CompareUnknownAndNullSimple))
     :named CompareUnknownAndNullSimple-deref-2)))

(assert (=> enable-solution-s1
  (! (not (is-unsafe-deref-CompareUnknownAndNullSimple get-fc-3 get-x-CompareUnknownAndNullSimple))
     :named CompareUnknownAndNullSimple-deref-3)))

(assert (=> enable-solution-s1
  (! (not (is-unsafe-deref-CompareUnknownAndNullSimple get-fc-4 get-x-CompareUnknownAndNullSimple))
     :named CompareUnknownAndNullSimple-deref-4)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example CompareUnknownAndNonNull
;;
;; ```
;; void target(int *x, int * _NonNull y) {
;;   // (1)
;;   *x; // safe
;;   *y; // safe
;;   bool b = x == y;
;;   // (2)
;;   *x; // safe
;;   *y; // safe
;;   if (b) {
;;     // (3)
;;     *x; // safe
;;     *y; // safe
;;   } else {
;;     // (4)
;;     *x; // safe
;;     *y; // safe
;;   }
;; }
;; ```

(declare-datatype State-CompareUnknownAndNonNull
  ((make-State-CompareUnknownAndNonNull
    (_get-x PointerValue)
    (_get-y PointerValue)
    (_get-b Bool))))

(define-fun get-x-CompareUnknownAndNonNull
  ((state State-CompareUnknownAndNonNull)) PointerValue
  (_get-x state))

(define-fun get-y-CompareUnknownAndNonNull
  ((state State-CompareUnknownAndNonNull)) PointerValue
  (_get-y state))

(define-fun run-CompareUnknownAndNonNull
  ((state State-CompareUnknownAndNonNull))
  FlowConditions
  (match state
    (((make-State-CompareUnknownAndNonNull x y b)
      (let ((fc-1 (and (fc-conj--ptr-is-unknown x)
                       (fc-conj--ptr-is-nonnull y))))
      (let ((fc-2 (and fc-1 (fc-conj--ptrs-were-compared x y b))))
      (let ((fc-3 (and fc-2 b)))
      (let ((fc-4 (and fc-2 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 false false)))))))))

(define-fun is-reachable-CompareUnknownAndNonNull
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-CompareUnknownAndNonNull))
    (let ((fcs (run-CompareUnknownAndNonNull state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-CompareUnknownAndNonNull
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-CompareUnknownAndNonNull PointerValue)))
  Bool
  (exists ((state State-CompareUnknownAndNonNull))
    (let ((fcs (run-CompareUnknownAndNonNull state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert
  (! (is-reachable-CompareUnknownAndNonNull get-fc-1)
     :named CompareUnknownAndNonNull-reachable-1))

(assert
  (! (is-reachable-CompareUnknownAndNonNull get-fc-2)
     :named CompareUnknownAndNonNull-reachable-2))

(assert
  (! (is-reachable-CompareUnknownAndNonNull get-fc-3)
     :named CompareUnknownAndNonNull-reachable-3))

(assert
  (! (is-reachable-CompareUnknownAndNonNull get-fc-4)
     :named CompareUnknownAndNonNull-reachable-4))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndNonNull get-fc-1 get-x-CompareUnknownAndNonNull))
     :named CompareUnknownAndNonNull-deref-1-x))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndNonNull get-fc-1 get-y-CompareUnknownAndNonNull))
     :named CompareUnknownAndNonNull-deref-1-y))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndNonNull get-fc-2 get-x-CompareUnknownAndNonNull))
     :named CompareUnknownAndNonNull-deref-2-x))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndNonNull get-fc-2 get-y-CompareUnknownAndNonNull))
     :named CompareUnknownAndNonNull-deref-2-y))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndNonNull get-fc-3 get-x-CompareUnknownAndNonNull))
     :named CompareUnknownAndNonNull-deref-3-x))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndNonNull get-fc-3 get-y-CompareUnknownAndNonNull))
     :named CompareUnknownAndNonNull-deref-3-y))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndNonNull get-fc-4 get-x-CompareUnknownAndNonNull))
     :named CompareUnknownAndNonNull-deref-4-x))

(assert
  (! (not (is-unsafe-deref-CompareUnknownAndNonNull get-fc-4 get-y-CompareUnknownAndNonNull))
     :named CompareUnknownAndNonNull-deref-4-y))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Example CompareUnknownAndNullAdvanced
;;
;; ```
;; void target(int *x) {
;;   // (1)
;;   *x; // unsafe
;;   bool b = x == nullptr;
;;   // (2)
;;   *x; // unsafe
;;   if (b) {
;;     // (3)
;;     *x; // unsafe
;;   } else {
;;     // (4)
;;     *x; // safe
;;   }
;; }
;; ```

(declare-datatype State-CompareUnknownAndNullAdvanced
  ((make-State-CompareUnknownAndNullAdvanced
    (_get-the-nullptr PointerValue)
    (_get-x PointerValue)
    (_get-b Bool))))

(define-fun get-x-CompareUnknownAndNullAdvanced
  ((state State-CompareUnknownAndNullAdvanced)) PointerValue
  (_get-x state))

(define-fun run-CompareUnknownAndNullAdvanced
  ((state State-CompareUnknownAndNullAdvanced))
  FlowConditions
  (match state
    (((make-State-CompareUnknownAndNullAdvanced the-nullptr x b)
      (let ((fc-1 (and (fc-conj--ptr-is-null the-nullptr)
                       (fc-conj--ptr-is-unknown x))))
      (let ((fc-2 (and fc-1 (fc-conj--ptrs-were-compared x the-nullptr b))))
      (let ((fc-3 (and fc-2 b)))
      (let ((fc-4 (and fc-2 (not b))))
        (make-FlowConditions fc-1 fc-2 fc-3 fc-4 false false)))))))))

(define-fun is-reachable-CompareUnknownAndNullAdvanced
  ((fc-getter (-> FlowConditions Bool)))
  Bool
  (exists ((state State-CompareUnknownAndNullAdvanced))
    (let ((fcs (run-CompareUnknownAndNullAdvanced state)))
      (fc-getter fcs))))

(define-fun is-unsafe-deref-CompareUnknownAndNullAdvanced
  ((fc-getter (-> FlowConditions Bool))
   (ptr-getter (-> State-CompareUnknownAndNullAdvanced PointerValue)))
  Bool
  (exists ((state State-CompareUnknownAndNullAdvanced))
    (let ((fcs (run-CompareUnknownAndNullAdvanced state)))
      (is-unsafe-to-deref (fc-getter fcs) (ptr-getter state)))))

(assert (=> (not enable-solution-s1)
  (! (is-reachable-CompareUnknownAndNullAdvanced get-fc-1)
     :named CompareUnknownAndNullAdvanced-reachable-1)))

(assert (=> (not enable-solution-s1)
  (! (is-reachable-CompareUnknownAndNullAdvanced get-fc-2)
     :named CompareUnknownAndNullAdvanced-reachable-2)))

(assert (=> (not enable-solution-s1)
  (! (is-reachable-CompareUnknownAndNullAdvanced get-fc-3)
     :named CompareUnknownAndNullAdvanced-reachable-3)))

(assert (=> (not enable-solution-s1)
  (! (is-reachable-CompareUnknownAndNullAdvanced get-fc-4)
     :named CompareUnknownAndNullAdvanced-reachable-4)))

(assert (=> (not enable-solution-s1)
  ;; The dereference at (1) is actually unsafe, but the structure of the
  ;; dataflow analysis defined in this file, can't detect that. The issue is
  ;; that the flow condition at (1) does not have information about what happens
  ;; later.
  (! (not (is-unsafe-deref-CompareUnknownAndNullAdvanced get-fc-1 get-x-CompareUnknownAndNullAdvanced))
     :named CompareUnknownAndNullAdvanced-deref-1)))

(assert (=> (not enable-solution-s1)
  (! (is-unsafe-deref-CompareUnknownAndNullAdvanced get-fc-2 get-x-CompareUnknownAndNullAdvanced)
     :named CompareUnknownAndNullAdvanced-deref-2)))

(assert (=> (not enable-solution-s1)
  (! (is-unsafe-deref-CompareUnknownAndNullAdvanced get-fc-3 get-x-CompareUnknownAndNullAdvanced)
     :named CompareUnknownAndNullAdvanced-deref-3)))

(assert (=> (not enable-solution-s1)
  (! (not (is-unsafe-deref-CompareUnknownAndNullAdvanced get-fc-4 get-x-CompareUnknownAndNullAdvanced))
     :named CompareUnknownAndNullAdvanced-deref-4)))

(echo "Verifying the selected solution against test cases.")
(check-sat)
(get-unsat-core)
(get-value (fc-conj--ptr-is-null))
(get-value (fc-conj--ptr-is-unknown))
(get-value (fc-conj--ptr-is-nonnull))
(get-value (fc-conj--ptr-is-nullable))
(get-value (fc-conj--ptrs-were-compared))
(get-value (fc-conj--join-ptr))
(get-value (is-unsafe-to-deref))

; vim: set syntax=scheme:
