; allow Markdown fenced blocks inside doc-comments (example)
((line_comment) @comment
 (#match? @comment "^///```\\w+$")
 (#set! injection.language "markdown"))
