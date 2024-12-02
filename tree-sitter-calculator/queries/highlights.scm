; Operators
(binary_expression
  operator: ["+" "*" "-" "/"] @operator)

; Parens
["(" ")"] @punctuation

; Numbers
(number) @number

; Floats
(float) @float

; Errors
(ERROR) @error