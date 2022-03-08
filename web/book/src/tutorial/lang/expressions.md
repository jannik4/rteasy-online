# Expressions

Expressions appear in assignments, asserts, and as conditions.
Besides literals, registers and buses there is the possibility to form concatenations and terms.

When evaluating expressions, expressions of different sizes are automatically adjusted by zero extending by default.
However, if you want for example a register to be interpreted as signed, you can use the sign extend operator (see further below) to extend with the sign bit instead.
What the exact rules are when evaluating expressions can be seen in [Evaluation of Expressions](../advanced/eval-expressions.md).

## Literals

### Decimal

```rteasy
~declare register X(7:0)
~
X <- 12;
X <- 00000012; # leading zeros are stripped
```

### Binary

Binary literals are prefixed with `0b`/`0B` (or with `%` for compatibility with older versions).

```rteasy
~declare register X(7:0)
~
X <-     0b110011;
X <-     0B110011;
X <-      %110011;
X <- 0b0000110011; # leading zeros are stripped
```

### Hexadecimal

Hexadecimal literals are prefixed with `0x`/`0X` (or with `$` for compatibility with older versions).

```rteasy
~declare register X(7:0)
~
X <-     0xfa;
X <-     0XfA;
X <-      $fa;
X <- 0x0000fa; # leading zeros are stripped
```

### Bit Strings

Bit strings are a sequence of zeros and ones delimited by double quotes. They work pretty much the same as normal binary numbers. The only difference is that they have a fixed size unlike other literals. All other literals ignore leading zeros and are always equivalent to the shortest binary representation of themselves.

The main use case for bit strings is for use in concatenations (see further below) where all parts must have a well defined size.

```rteasy
~declare register X(7:0)
~
X <- "01010";
```

## Registers/Buses

Registers and buses can be used simply by name. Individual bits can be accessed by specifying a bit range.

```rteasy
~declare register X(7:0)
declare register REG(7:0)
declare bus BUS(7:0)

X <- REG;
X <- BUS;

X <- REG(4:2); # Access a subrange
X <- BUS(4);   # Access a single bit
```

## Register Arrays

Using register arrays works similiar to using registers. In addition to the name, an index must be specified, which is itself an expression.
Accessing individual bits is not possible with register arrays.

In addition, care must be taken that register arrays may be read no more than twice once per execution path and cycle.

```rteasy
~declare register X(7:0)
declare register IDX(1:0)
declare register array ARR(7:0)[4]

X <- ARR[0];       # Read at index 0
X <- ARR[IDX + 1]; # Use an expression as the index
```

If it is necessary to access individual bits, this can be done with the help of a temporary bus:

```rteasy
~declare register X(7:0)
declare bus TEMP(7:0)
declare register array ARR(7:0)[4]

TEMP <- ARR[0], X <- TEMP(4:2); # Access a subrange at index 0
```

## Concatenations

Concatenations can be composed of registers, buses, register arrays and bit strings. The individual values are simply concatenated bit by bit.

```rteasy
~declare register X(31:0)
declare register REG(7:0)
declare bus BUS(7:0)
declare register array ARR(7:0)[4]

X <- REG."001100".BUS(2);
X <- "01".ARR[3].REG(3:1);
```

## Terms

Unary terms are written as `OPERATOR EXPRESSION` and binary terms are written as `EXPRESSION OPERATOR EXPRESSION`.

```rteasy
~declare register X(15:0)
declare register REG(7:0)
declare bus BUS(7:0)

X <- neg REG;
X <- REG + 1;
X <- not BUS."1" + 42;
```

The following table gives an overview of all operators. Operators with higher precedence are executed before operators with lower precedence. Precedence comes before associativity.

| Precedence | Operator              | Associativity | Individual operators |
| ---------- | --------------------- | ------------- | -------------------- |
| 10         | Unary Sign            | right-to-left | `- ... `             |
| 10         | Unary Negation        | right-to-left | `neg ... `           |
| 9          | Sign Extend           | right-to-left | `sxt ... `           |
| 8          | Addition              | left-to-right | `... + ...`          |
| 8          | Subtraction           | left-to-right | `... - ...`          |
| 7          | Less Than             | left-to-right | `... < ...`          |
| 7          | Less Than Or Equal    | left-to-right | `... <= ...`         |
| 7          | Greater Than          | left-to-right | `... > ...`          |
| 7          | Greater Than Or Equal | left-to-right | `... >= ...`         |
| 6          | Equality              | left-to-right | `... = ...`          |
| 6          | Inequality            | left-to-right | `... <> ...`         |
| 5          | Bitwise NOT           | right-to-left | `not ...`            |
| 4          | Bitwise NAND          | left-to-right | `... nand ...`       |
| 3          | Bitwise AND           | left-to-right | `... and ...`        |
| 2          | Bitwise NOR           | left-to-right | `... nor ...`        |
| 1          | Bitwise OR            | left-to-right | `... or ...`         |
| 0          | Bitwise XOR           | left-to-right | `... xor ...`        |
