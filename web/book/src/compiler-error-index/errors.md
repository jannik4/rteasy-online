# Errors

An overview of all possible errors that can occur during compilation.

## E001

This error indicates that a symbol is declared more than once.

### Examples

```rteasy,compile_fail(E001)
declare register X(3:0)
declare register X # error: duplicate symbol "X"
```

```rteasy,compile_fail(E001)
declare register X(3:0)
declare bus X # error: duplicate symbol "X"
```

## E002

This error indicates that the length of a register array is not a power of two. The length must always be a power of two.

### Examples

```rteasy,compile_fail(E002)
# error: length of register array "ARR" must be a power of two
declare register array ARR(7:0)[3]
```

```rteasy,compile_fail(E002)
# error: length of register array "ARR" must be a power of two
declare register array ARR(7:0)[0]
```

## E003

This error occurs when a register array is used without an index expression.

### Examples

```rteasy,compile_fail(E003)
~declare register X(7:0)
~declare register array ARR(7:0)[4]
~
X <- ARR[0] + 1; # ok
X <- ARR + 1;    # error: register array "ARR" is missing index [...]
```

```rteasy,compile_fail(E003)
~declare register array ARR(7:0)[4]
~
ARR[3] <- 1 + 1; # ok
ARR <- 1 + 1;    # error: register array "ARR" is missing index [...]
```

## E004

This error indicates that a label is declared more than once. Labels are used as goto marks and must therefore be unique.

### Examples

```rteasy,compile_fail(E004)
~declare register X(3:0), Y(3:0)
~
MY_LABEL: X <- Y;
MY_LABEL: X <- X + 1; # error: duplicate label "MY_LABEL"
```

## E005

This error occurs when a symbol can not be found.

### Examples

```rteasy,compile_fail(E005)
declare register AR(3:0)
declare memory MEM(AR, DR) # error: no register named "DR" found
```

```rteasy,compile_fail(E005)
X <- 42 + 2; # error: no register or bus named "X" found
```

```rteasy,compile_fail(E005)
declare register X(3:0)
X <- ARR[0]; # error: no register array named "ARR" found
```

```rteasy,compile_fail(E005)
read MEM; # error: no memory named "MEM" found
```

## E006

This error occurs when a label can not be found.

### Examples

```rteasy,compile_fail(E006)
LABEL_A: goto LABEL_B; # error: no label named "LABEL_B" found
```

## E007

This error occurs when an expression without a fixed size is used in a switch operation. This requirement is necessary to have a well defined size in which to evaluate. Fixed size expression are: comparisons, concatenations, registers, buses, register arrays and bit strings.

### Examples

```rteasy,compile_fail(E007)
~declare register X(3:0), Y(3:0)
~
switch X + Y { # error: expected fixed size expression
    case 1: nop
    default: nop
};
```

```rteasy,compile_fail(E007)
switch 12 { # error: expected fixed size expression
    case 1: nop
    default: nop
};
```

```rteasy
~declare register X(3:0), Y(3:0)
~
switch X = Y { # ok
    case 1: nop
    default: nop
};
```

```rteasy
switch "1100" { # ok
    case 1: nop
    default: nop
};
```

## E008

This error occurs when a non-constant expression used in a case clause. Constant expression are: literals, concatenations only containing constants and terms only containing constants.

### Examples

```rteasy,compile_fail(E008)
~declare register X(3:0), Y(3:0)
~
switch "0101" {
    case X + Y: nop # error: expected constant expression
    default: nop
};
```

```rteasy
switch "0101" {
    case 7: nop # ok
    default: nop
};
```

```rteasy
switch "0101" {
    case 3 + 4: nop # ok
    default: nop
};
```

## E009

This error indicates a switch operation with zero or more than one default clause. Switch operations must always have exactly one default clause.

### Examples

```rteasy,compile_fail(E009)
# error: expected exactly one default clause
switch "0101" {
    case 1: nop
};
```

```rteasy,compile_fail(E009)
# error: expected exactly one default clause
switch "0101" {
    case 1: nop
    default: nop
    default: nop
};
```

## E010

This error occurs when a literal other than the bit string is used in a concatenation. Concatenations may only contain elements of fixed size, thus only registers, buses, register arrays and bit strings.

### Examples

```rteasy,compile_fail(E010)
~declare register X(7:0), Y(3:0)
~
X <- Y."101".Y(0); # ok
X <- Y.5.Y(0);     # error: concat must not contain numbers other than bit strings
```

## E011

TODO: ...

## E012

TODO: ...

## E013

TODO: ...

## E014

TODO: ...

## E015

TODO: ...

## E016

TODO: ...

## E017

TODO: ...

## E018

TODO: ...

## E019

TODO: ...

## E020

TODO: ...

## E021

TODO: ...

## E022

TODO: ...

## E023

TODO: ...

## E024

TODO: ...

## E025

This error indicates that a register, bus, register array or memory is assigned more than once in a cycle. Only one assignment to an item may be executed per execution path and cycle.

### Examples

```rteasy,compile_fail(E025)
~declare register X(3:0)
~
X <- 2, X <- 1; # error: register "X" is assigned more than once
```

```rteasy,compile_fail(E025)
~declare register AR(3:0), DR(3:0)
~declare memory MEM(AR, DR)
~
read MEM, read MEM; # error: register "DR" is assigned more than once
write MEM, write MEM; # error: memory "MEM" is assigned more than once
```

```rteasy
~declare register X(3:0), COND
~
# ok, because always only one of the two assignments is executed in one cycle.
if COND then X <- 2 else X <- 1 fi;
```

## E026

TODO: ...

## E027

TODO: ...

## E028

TODO: ...
