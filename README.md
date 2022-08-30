# four-arithmetic-ops-parser
Simple parser for 4 operation(+, -, *, /)

This makes expression syntax tree.

## examples

Addition
```
❯ cargo run 1 + 2
(Add  1  2)
```

Subtraction
```
❯ cargo run 1 - 2
(Sub  1  2)
```

Multiplication
```
❯ cargo run "1 * 2"
(Mul  1  2)
```

Division
```
❯ cargo run 1 / 2
(Div  1  2)
```

Default is left associative
```
❯ cargo run 1 + 2 + 3 + 4
(Add(Add(Add  1  2)  3)  4)
```

If you use parens(`(`,`)`) or multiplication(`*`), you need to use quotaion(`'` or `"`)
```
❯ cargo run "(1 + 2)"
(Add  1  2)

❯ cargo run "1 * 2"
(Mul  1  2)
```

By using parens, you can change priority for operation
```
❯ cargo run "1 + (2 + 3)"
(Add  1(Add  2  3))
```

```
❯ cargo run "(1 + 2) + (3 + 4)"
(Add(Add  1  2)(Add  3  4))
```

Multiplication and division take precedence over addition and multiplication
```
❯ cargo run "1 + 2 * 3"
(Add  1(Mul  2  3))

❯ cargo run "1 - 2 / 3"
(Sub  1(Div  2  3))
```

## not implementation
- Error handling for zero division
- Evaluate expression
