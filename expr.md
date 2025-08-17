# Expressions

Based on [order](https://www.cs.bilkent.edu.tr/~guvenir/courses/CS101/op_precedence.html) as references in `Quint` grammar.

## Precedence and assoc

> NOTE: lower level == high precedence

| level | operators                                           | assoc | opcode          |
| ----- | --------------------------------------------------- | ----- | --------------- |
| 0     | Terms                                               | LR    |                 |
| 1     | expr."op", operApp (op=identifer), listapp (op=nth) | LR    |                 |
| 2     | expr ^ expr, -expr                                  | RL    | ipow, iuminus   |
| 3     | *, /, %                                             | LR    | imul,idiv, imod |
| 4     | +, -                                                | LR    | iadd, isub      |
| 5     | <, <=, >, >=                                        | LR    |                 |
| 6     | ==, !=                                              | LR    |                 |
| 7     | and                                                 | LR    |                 |
| 8     | or                                                  | LR    |                 |
| 9     | =                                                   | RL    | Assign          |
| 10    | all else                                            | LR    |                 |




