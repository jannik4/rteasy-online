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

...

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

...
