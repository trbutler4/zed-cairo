;; ─────────────── keywords / punctuation ───────────────
;; ── `use` paths ──────────────────────────────────────────────────
;; intermediate segments (alexandria_merkle_tree, merkle_tree, …)
(scoped_identifier
  path: (identifier) @namespace)
;; last segment – could be identifier **or** type_identifier
(scoped_identifier
  name: [
    (identifier)        @type
    (type_identifier)   @type
  ])

;; ── brace-group imports  use foo::{Bar, baz::Qux, *} ─────────────
;; 1) plain identifiers inside the braces
(use_list
  (identifier) @type)

;; 2) scoped identifiers inside the braces – colour the last segment
(use_list
  (scoped_identifier
    name: [
      (identifier)       @type
      (type_identifier)  @type
    ]))

(
  (identifier) @keyword
  (#match? @keyword "^(enum|trait|type|const|extern|match|if|else|for|while|loop|break|continue|return|as)$")
)
;; function keyword and snapshot sigil
("fn") @keyword.function          ; coloured differently from generic @keyword
("@")  @punctuation.special       ; snapshot marker in signatures
("of")   @keyword
("impl") @keyword
;; declaration + mutability keywords
("let") @keyword                 ; often coloured blue / red
(mutable_specifier) @keyword.modifier

;; let merkle_tree = …  – colour the binding name
(let_declaration
  pattern: (identifier) @variable)

("ref") @keyword

("mod") @namespace.keyword
(mod_item
  name: (identifier) @namespace.definition)

;; ── struct definitions  (keyword + name share colour) ──────────────
("struct") @type.keyword ; the keyword
(struct_item
  name: (type_identifier) @type.definition) ; the name

("use") @use.keyword

(use_declaration
  argument: [
    (identifier)         @namespace
    (scoped_identifier)  @namespace
  ]
)

;; ───────────────── parameters & `self` ─────────────────────────────
;; function-parameter names  fn foo(owner: Address, …)
(parameter
  pattern: (identifier) @variable.parameter)

;; the built-in receiver `self`
((identifier) @variable.builtin
 (#eq? @variable.builtin "self"))

;; impl Foo { … }   – highlight the `Foo`
(impl_item
  ;; the first bare identifier that appears after `impl`
  (identifier) @type.definition
  .            ; stop after the first one so we don’t capture items inside the body
)


;; leading hash in attributes  #[derive]  #[storage]
("#" @punctuation.special)


("::") @punctuation.delimiter

("=>" @operator)
("->" @operator)
("="  @operator)
("%"  @operator)
("+"  @operator)
("-"  @operator)
("*"  @operator)
("/"  @operator)
("&&" @operator)
("||" @operator)
("!"  @operator)
("==" @operator)
("!=" @operator)
(">"  @operator)
("<"  @operator)

;; ── method call  self.foo()  ───────────────────────────
(call_expression
  function: (field_expression
              field: (field_identifier) @function.call))

;; ── field access  self.owner  map.len  ─────────────────────────────
(field_expression
  field: [
    (field_identifier)  @property           ; struct / storage fields
    (identifier)        @property           ; module-level variables
  ])

;; ─────────────── literals ───────────────
(boolean_literal)  @boolean
(string_literal)   @string
(shortstring_literal) @string
(numeric_literal)  @number
(negative_literal
  (numeric_literal) @number)

;; ─────────────── comments ───────────────
(line_comment) @comment

;; ─────────────── types ───────────────
(primitive_type)          @type.builtin
(type_identifier)         @type
(scoped_type_identifier)  @type

;; ─────────────── functions ───────────────
;; definition
(function
  name: (identifier) @function.definition)

;; call – simple or path-qualified
(call_expression
  function: [
    (identifier) @function.call
    (scoped_identifier
      name: (identifier) @function.call)
  ])

(generic_function
  function: (identifier) @function.call)

;; ─────────────── macros ───────────────
(macro_invocation
  macro: (identifier) @macro)

;; ─────────────── struct / enum names ───────────────
(struct_item name: (type_identifier) @type.definition)
(enum_item   name: (type_identifier) @type.definition)

;; fields inside struct expressions & patterns
(field_identifier) @property

;; attributes (`#[derive]`, `#[constructor]`, …)
(attribute
  (identifier) @attribute)

;; ─────────────── module / path separators ───────────────
("::" @punctuation.delimiter)
("."  @punctuation.delimiter)
(","  @punctuation.delimiter)
(";"  @punctuation.delimiter)
(":"  @punctuation.delimiter)
("("  @punctuation.bracket)
(")"  @punctuation.bracket)
("{"  @punctuation.bracket)
("}"  @punctuation.bracket)
("["  @punctuation.bracket)
("]"  @punctuation.bracket)
