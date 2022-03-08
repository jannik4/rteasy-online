# Micro Operations

The micro operations are the building blocks of the algorithm. Besides the unconditional operations (assignment, no operation and read/write) there are jumps (goto) and conditional operations (if and switch).

## Assignment

...

## No Operation

The `nop` operation will not trigger any operation and can be used to have an empty state.

```rteasy
nop;
```

## Read/Write

The read and write operations are available for operating on memories. Both operations take the name of the memory as an argument, e.g. `read MEM`. When reading, the value currently stored in the memory at the position of the address register is written to the data register. In the case of writing, the process is exactly the opposite: The value from the data register is written into the memory.

```rteasy
~declare register AR(3:0), DR(3:0)
~declare memory MEM(AR, DR)
~
# Read value at position 4
AR <- 4;
read MEM;

# Write (value + 1) back to the memory
DR <- DR + 1;
write MEM;
```

## Assert

The assert operation checks if an expression (with a size of one bit) is one or zero. If the expression evaluates to zero, the assert fails and the simulator will stop immediately and highlight the failed assert.

The assert operation is intended as a tool for development and is therefore only executed in the simulator. It will not execute if the program gets compiled to VHDL.

```rteasy
~declare bus BUS(3:0)
~
assert 5 > 2; # passes
assert BUS = 2, BUS <- 2; # passes
```

```rteasy,should_fail
assert 2 = 3; # fails
```

## Goto

The goto operation takes a label and can be used to resume the execution at a different state than the following one.

```rteasy
~declare register A, B
~
START: A <- 0, B <- 1, goto SKIP;

A <- B; # <-- This is never executed

SKIP: nop; # do something
END: goto START;
```

## If

The if operation checks a single condition, which can be either one or zero, and executes the if or else branch accordingly. Syntactically, the else branch is optional. If it is missing a simple `nop` is used instead. Both branches can contain any number of micro operations, just like a state. Thus, it is also possible to nest if operations.

```rteasy
~declare register A(3:0), B(3:0), C(3:0), D(3:0), COUNT(3:0)
~
# If/else
if COUNT = 0 then
    A <- 0, B <- 0
else
    A <- 1
fi;

# Nested if
if A = 0 and B = 0 then
    C <- 0, if D > 1 then D <- 0 fi
fi;
```

## Switch

The switch operation checks an expression against various values. The expression must have a fixed size. This requirement is necessary to have a well defined size in which to evaluate. Fixed size expression are: comparisons, concatenations, registers, buses, register arrays and bit strings.

The values used in the case clauses can be literals or constant expressions. Constant expression are: literals, concatenations only containing constants and terms only containing constants.

In addition to the case clauses, there must always be exactly one default clause.

```rteasy
~declare register A(3:0), B(3:0), C(3:0), D(3:0)
~
switch A.D(2) {            # match against a fixed size expression
    case 0: B <- 2, C <- 2 # case clause 0
    case 1: nop            # case clause 1
    case 1 + 1: C <- 3     # case clause 2 (1 + 1)
    default: goto END      # default clause
};
~
~END:
```
