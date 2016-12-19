
(asdf:load-system :cffi)

;; Constants
(defconstant +spec-max-draw+ 16)
(defconstant +spec-max-x+ 10)
(defconstant +spec-max-y+ 5)
(defconstant +spec-max-xy+ (* +spec-max-x+ +spec-max-y+))
(defconstant +spec-max-pre-xy+ (- +spec-max-xy+ 1))
(defconstant +spec-character-max+ (* +spec-max-y+ 16))

;; Character Color Attributes
(defvar +black+ (make-array '(3)))
(setf (aref +black+ 0) 0)
(setf (aref +black+ 1) 0)
(setf (aref +black+ 2) 0)
(defvar +red+ (make-array '(3)))
(setf (aref +red+ 0) 255)
(setf (aref +red+ 1) 0)
(setf (aref +red+ 2) 0)
(defvar +yellow+ (make-array '(3)))
(setf (aref +yellow+ 0) 255)
(setf (aref +yellow+ 1) 255)
(setf (aref +yellow+ 2) 0)
(defvar +green+ (make-array '(3)))
(setf (aref +green+ 0) 0)
(setf (aref +green+ 1) 255)
(setf (aref +green+ 2) 0)
(defvar +cyan+ (make-array '(3)))
(setf (aref +cyan+ 0) 0)
(setf (aref +cyan+ 1) 255)
(setf (aref +cyan+ 2) 255)
(defvar +blue+ (make-array '(3)))
(setf (aref +blue+ 0) 0)
(setf (aref +blue+ 1) 0)
(setf (aref +blue+ 2) 255)
(defvar +magenta+ (make-array '(3)))
(setf (aref +magenta+ 0) 255)
(setf (aref +magenta+ 1) 0)
(setf (aref +magenta+ 2) 255)
(defvar +white+ (make-array '(3)))
(setf (aref +white+ 0) 255)
(setf (aref +white+ 1) 255)
(setf (aref +white+ 2) 255)
(defvar +default-foreground+ (make-array '(3)))
(setf (aref +default-foreground+ 0) 0)
(setf (aref +default-foreground+ 1) 0)
(setf (aref +default-foreground+ 2) 0)
(defvar +default-background+ (make-array '(3)))
(setf (aref +default-background+ 0) 255)
(setf (aref +default-background+ 1) 255)
(setf (aref +default-background+ 2) 255)

;; Characters Style Attributes
(cffi:defcenum attribute
  (:none #x00)
  (:bold #x01)
  (:dim #x02)
  (:italic #x04)
  (:underline #x08)
  (:blink #x10)
  (:reverse #x20)
  (:hidden #x40))

;; Neko Default Placements into the display screen
(cffi:defcenum cardinal
  :upper-left
  :upper-middle
  :upper-right
  :middle-left
  :middle-central
  :middle-right
  :lower-left
  :lower-middle
  :lower-right)

;; Neko's Postures
(cffi:defcenum sheet
  (:none 95)
  (:bust 98))

;; Neko's Body Parts and Accessories
(cffi:defcenum part
  (:none 95)
  (:arm-left 97)
  (:arm-right 65)
  (:boobs 44)
  (:clavicle 99)
  (:ear-left 101)
  (:ear-right 69)
  (:eye-left 121)
  (:eye-right 89)
  (:hair-top 111)
  (:hair-left 114)
  (:hair-right 82)
  (:hand-left 100)
  (:hand-right 68)
  (:mouth 109)
  (:tail 116)
  (:bell 108)
  (:exclamation-mark 120)
  (:exclamation-marks 88)
  (:heart 104)
  (:hearts 72)
  (:lantern 110)
  (:question-mark 113)
  (:question-marks 81)
  (:wool-ball 119))

;; Neko's Emotions
(cffi:defcenum emotion
  (:none 95)
  (:angry 97)
  (:happy 104)
  (:love 108)
  (:malicious 109)
  (:misunderstanding 105)
  (:shocked 111)
  (:sleepy 115)
  (:speechless 101))

;; Neko Placement Selection
(cffi:defcstruct _position_
  (:cardinal cardinal)
  (cartesian (:array :unsigned-short 2)))

;; Neko's Texels Definition
(cffi:defcstruct _tuple_
  (:part part)
  (:emotion emotion))

;; Display screen's Characters
(cffi:defcstruct _character_
  (attribute :unsigned-char)
  (foreground (:array :unsigned-char 3))
  (background (:array :unsigned-char 3))
  (glyph :int))

(cffi:defcstruct _library-state_
  (:sheet sheet)
  (emotion (:array (:array (:pointer (:struct _tuple_)) 50) 16))
 ;;;;; (emotion (:array (:array (:pointer (:struct _tuple_)) +spec-max-xy+) +spec-max-draw+))
  (:pointer (:struct _position_))
  (message (:array (:struct _character_) 80))
 ;;;;; (message (:array (:struct _character_) +spec-character-max+))
  (unmount :unsigned-char))

;;(sb-ext:exit)
