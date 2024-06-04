

; keywords
(meta_type_expression "type" @keyword)

[
  "as",
  "break",
  "const",
  "continue",
  "else",
  "enum",
  "extern",
  "false",
  "fn",
  "if",
  "impl",
  "implicits",
  "let",
  "loop",
  "match",
  "mod",
  "mut",
  "nopanic",
  "of",
  "pub",
  "ref",
  "return",
  "struct",
  "trait",
  "true",
  "type",
  "use",
  "while",
] @keyword

; 'loose' keywords
["self", "super"] @keyword.loose

; 'reservered' keywords
[
  "Self",
  "do",
  "dyn",
  "for",
  "hint",
  "in",
  "macro",
  "move",
  "static_assert",
  "static",
  "try",
  "typeof",
  "unsafe",
  "where",
  "with",
  "yield"
] @keyword.reserved

; Built-in functions
[
  "assert",
  "panic"
] @function.builtin

; Operators
[
  "!",
  "~",
  "!=",
  "%",
  "%=",
  "&",
  "&&",
  "*",
  "*=",
  "@",
] @operator
