
halftau
-------

Small lisp interpreter written in Rust.

See the prelude:

```clojure
(def defmacro
  (macro [macro-name macro-args macro-body]
         (def macro-name (macro macro-args macro-body))))

(defmacro defn [fn-name fn-args fn-body]
  (def fn-name (fn fn-args fn-body)))

(defn id [x] x)

; > is compiler builtin
(defn or [a b] (if a a b))
(defn and [a b] (if a b a))
(defn >= [a b] (or (> a b) (= a b)))
(defn < [a b] (not (>= a b)))
(defn <= [a b] (not (> a b)))

(defn map [f elts]
  (if (empty? elts) '()
    (cons (f (car elts)) (map f (cdr elts)))))

(defn filter [pred elts]
  (if (empty? elts) '()
    (if (pred (car elts))
      (cons (car elts) (filter pred (cdr elts)))
      (filter pred (cdr elts)))))

(defn foldl [f acc elts]
  (if (empty? elts) acc
    (foldl f (f acc (car elts)) (cdr elts))))

(defn member [x elts]
  (if (empty? elts) false
    (if (= x (car elts)) true
      (member x (cdr elts)))))

; test equality
(assert (= '(1 2 3) '(1 2 3)))
(assert (= "hello" "hello"))
(assert (= 'a 'a 'a 'a))
(assert (= '(a b c) '(a b c)))
(assert-eq false (= 'a 'a 'b 'a))

; test list ops
(assert-eq false (empty? '(1 2 3)))
(assert (empty? '()))
(assert-eq '(1 2 3) (cons 1 '(2 3)))
(assert-eq '(1) (cons 1 '()))
(assert-eq 1 (car '(1 2 3)))
(assert-eq '(2 3) (cdr '(1 2 3)))
(assert-eq '() (cdr '(1)))

; test arithmetic
(assert-eq 109 (+ 3 6 100))
(assert-eq 109 (+ (+ 3 6) 100))
(assert-eq 11 (- 12 1))
(assert-eq 1 (- 12 1 10))
(assert-eq 32 (* 16 2))
(assert-eq 32 (* 8 2 2))
(assert-eq 0.5 (/ 1 2))
(assert-eq 1.0 (/ 1))
(assert (> 3 2))
(assert (not (> 2 3)))
(assert (not (> 0 5)))
(assert (< 2 3))
(assert (<= 2 3))
(assert (<= 3 3))

; test logic
(assert-eq false (not true))
(assert-eq true (not false))
(assert-eq true (not nil))
(assert-eq false (not '()))
(assert-eq false (not '(1 2)))
(assert-eq false (not "string"))
(assert-eq 'a (if true 'a 'b))
(assert-eq 'b (if false 'a 'b))
(assert-eq nil (if false 'a))
(assert (or true false))
(assert (or false true))
(assert (not (and false true)))
(assert (not (and true false)))

; test stdlib
(assert-eq 109 (foldl + 3 '(6 100)))
(assert-eq 600 (foldl * 1 '(6 100)))
(assert (member 1 '(1 2 3)))
(assert (not (member 4 '(1 2 3))))
(assert-eq '(true false true) (map not '(false true false)))
(assert-eq '(false true false) (map id '(false true false)))
(assert-eq '(3 4 5) (map (fn [x] (+ x 2)) '(1 2 3)))
```


