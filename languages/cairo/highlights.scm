;; ═══════════════════════════════════════════════════════════════════
;;                         CAIRO SYNTAX HIGHLIGHTING
;; ═══════════════════════════════════════════════════════════════════

;; ─────────────── KEYWORDS & LANGUAGE CONSTRUCTS ───────────────────
;; Core language keywords
(
  (identifier) @keyword
  (#match? @keyword "^(enum|trait|type|const|extern|match|if|else|for|while|loop|break|continue|return|as)$")
)

;; Function-specific keywords
("fn") @keyword.function
("impl") @keyword

;; Declaration keywords
("let") @keyword
("ref") @keyword
("mod") @namespace.keyword
("use") @use.keyword
("of") @keyword

;; Mutability modifiers
(mutable_specifier) @keyword.modifier

;; ─────────────── MODULES & NAMESPACES ──────────────────────────────
;; Module definitions
(mod_item
  name: (identifier) @namespace.definition)

;; Use declarations
(use_declaration
  argument: [
    (identifier)         @namespace
    (scoped_identifier)  @namespace
  ]
)

;; Use paths - intermediate segments
(scoped_identifier
  path: (identifier) @namespace)

;; Use paths - final segment
(scoped_identifier
  name: [
    (identifier)        @type
    (type_identifier)   @type
  ])

;; Brace-group imports: use foo::{Bar, baz::Qux, *}
(use_list
  (identifier) @type)

(use_list
  (scoped_identifier
    name: [
      (identifier)       @type
      (type_identifier)  @type
    ]))

;; ─────────────── TYPES & TYPE DEFINITIONS ──────────────────────────
;; Primitive and built-in types
(primitive_type) @type.builtin
(type_identifier) @type
(scoped_type_identifier) @type

;; Struct definitions
("struct") @type.keyword
(struct_item
  name: (type_identifier) @type.definition)

;; Enum definitions
("enum") @type.keyword
(enum_item
  name: (type_identifier) @type.definition)

;; Implementation blocks
(impl_item
  (identifier) @type.definition
  .)

;; ─────────────── FUNCTIONS ─────────────────────────────────────────
;; Function definitions
(function
  name: (identifier) @function.definition)

;; Function calls - simple
(call_expression
  function: (identifier) @function.call)

;; Function calls - path-qualified
(call_expression
  function: (scoped_identifier
    name: (identifier) @function.call))

;; Generic function calls
(generic_function
  function: (identifier) @function.call)

;; Method calls
(call_expression
  function: (field_expression
              field: (field_identifier) @function.call))

;; ─────────────── VARIABLES & PARAMETERS ────────────────────────────
;; Variable bindings in let declarations
(let_declaration
  pattern: (identifier) @variable)

;; Function parameters
(parameter
  pattern: (identifier) @variable.parameter)

;; Built-in variables (self)
((identifier) @variable.builtin
 (#eq? @variable.builtin "self"))

;; Field access and properties
(field_expression
  field: [
    (field_identifier) @property
    (identifier)       @property
  ])

;; Struct field identifiers
(field_identifier) @property

;; ─────────────── LITERALS ──────────────────────────────────────────
(boolean_literal) @boolean
(string_literal) @string
(shortstring_literal) @string
(numeric_literal) @number
(negative_literal
  (numeric_literal) @number)

;; ─────────────── COMMENTS ──────────────────────────────────────────
(line_comment) @comment

;; ─────────────── OPERATORS ─────────────────────────────────────────
;; Arrow operators
("=>" @operator)
("->" @operator)

;; Assignment and comparison
("=" @operator)
("==" @operator)
("!=" @operator)
(">" @operator)
("<" @operator)

;; Arithmetic operators
("+" @operator)
("-" @operator)
("*" @operator)
("/" @operator)
("%" @operator)

;; Logical operators
("&&" @operator)
("||" @operator)
("!" @operator)

;; ─────────────── PUNCTUATION ───────────────────────────────────────
;; Special punctuation
("@" @punctuation.special)   ; snapshot marker
("#" @punctuation.special)   ; attribute marker

;; Delimiters
("::" @punctuation.delimiter)
("." @punctuation.delimiter)
("," @punctuation.delimiter)
(";" @punctuation.delimiter)
(":" @punctuation.delimiter)

;; Brackets
("(" @punctuation.bracket)
(")" @punctuation.bracket)
("{" @punctuation.bracket)
("}" @punctuation.bracket)
("[" @punctuation.bracket)
("]" @punctuation.bracket)

;; ─────────────── MACROS ────────────────────────────────────────────
(macro_invocation
  macro: (identifier) @macro)

;; ─────────────── ATTRIBUTES ────────────────────────────────────────
;; Generic attributes
(attribute
  (identifier) @attribute)

;; Cairo/Starknet-specific attributes
(attribute
  (identifier) @attribute
  (#match? @attribute "^(storage|event|constructor|external|view|l1_handler|derive|starknet)$"))

;; ─────────────── CAIRO-SPECIFIC PATTERNS ───────────────────────────
;; Felt252 type (Cairo's primary type)
((type_identifier) @type.builtin
 (#eq? @type.builtin "felt252"))

;; Contract interface keyword
((identifier) @keyword
 (#eq? @keyword "contract"))

;; Storage variables pattern
(attribute
  (scoped_identifier
    path: (identifier) @attribute
    name: (identifier) @attribute)
  (#eq? @attribute "storage"))

;; Event definitions
(attribute
  (identifier) @attribute
  (#eq? @attribute "event"))

;; External function markers
(attribute
  (call_expression
    function: (identifier) @attribute
    arguments: (arguments
      (identifier) @attribute))
  (#match? @attribute "^(external|view)$"))
