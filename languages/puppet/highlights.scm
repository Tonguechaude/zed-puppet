; Variables

(identifier) @variable

(include_statement (identifier) @type)

(include_statement (class_identifier (identifier) @type . ))

; Keywords

[
  "class"
  "inherits"
  "include"
  "node"
  "type"
  "tag"
] @keyword

[
  "if"
  "elsif"
  "else"
  "unless"
  "case"
] @conditional

[
  "define"
  "function"
] @keyword.function

(default_case "default" @conditional)

; Properties

(attribute name: (identifier) @property)
(attribute name: (variable (identifier) @property))

; Parameters

(lambda (variable (identifier) @parameter))

(parameter (variable (identifier) @parameter))

(function_call (identifier) @parameter)

; Functions

(function_declaration
  "function" . (identifier) @function)

(function_call
  (identifier) @function.call "(")

(function_call
  (field_expression "." (identifier) @method.call) "(")

(defined_resource_type
  "define" . (identifier) @function)

; Methods

(function_declaration
  "function" . (class_identifier (identifier) @method . ))

(function_call
  (class_identifier (identifier) @method.call . ))

(defined_resource_type
  "define" . (class_identifier (identifier) @method . ))

; Types

(type) @type

(builtin_type) @type.builtin

(class_definition
  (identifier) @type)
(class_definition
  (class_identifier (identifier) @type . ))

(class_inherits (identifier) @type)
(class_inherits (class_identifier (identifier) @type . ))

(node_definition (node_name (identifier) @type))

((identifier) @type
  (#lua-match? @type "^[A-Z]"))

((identifier) @type.builtin
  (#any-of? @type.builtin "Boolean" "Integer" "Float" "String" "Array" "Hash" "Regexp" "Variant" "Data" "Undef" "Default" "File" "Enum" "Optional"))

((identifier) @function.builtin
  (#any-of? @function.builtin "contain"))

; Handle resource highlighting
(resource_declaration
    type: (identifier) @keyword)

; Handle Types reference highlighting
(resource_reference
  (identifier) @variant)


; "Namespaces"

(class_identifier . (identifier) @namespace)

; Operators

[
  "or"
  "and"
  "in"
] @keyword.operator

[
  "="
  "+="
  "->"
  "~>"
  "<<|"
  "<|"
  "|>"
  "|>>"
  "?"
  ">"
  ">="
  "<="
  "<"
  "=="
  "!="
  "<<"
  ">>"
  "+"
  "-"
  "*"
  "/"
  "%"
  "=~"
  "!~"
] @operator

; Punctuation

[
  "|"
  "."
  ","
  ";"
  ":"
  "::"
  "=>"
] @punctuation.delimiter

[ "{" "}" ] @punctuation.bracket

[ "[" "]" ] @punctuation.bracket

[ "(" ")" ] @punctuation.bracket

(interpolation [ "${" "}" ] @punctuation.special)

[
  "$"
  "@"
  "@@"
] @punctuation.special

; Literals

(number) @number

(float) @float

(string) @string

(escape_sequence) @string.escape

(regex) @string.regex

(boolean) @boolean

[
  (undef)
  (default)
] @variable.builtin

; Comments

(comment) @comment @spell

; Errors

(ERROR) @error
