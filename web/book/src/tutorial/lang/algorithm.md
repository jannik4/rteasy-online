# Algorithm

The declarations are followed by the actual algorithm. The algorithm consists of a list of states that are executed sequentially. A state represents a clock cycle in which several _micro operations_ are executed in parallel. Each state is terminated by a semicolon.

```rteasy
~declare register A(3:0), B(3:0), REG(3:0)
~
# A simple state with two micro operations executed in parallel.
A <- A + 1, B <- B - 1;

# Another state with only one micro operation.
REG <- A and B;
```

## Label

In addition to the micro operations, a state can receive a label to which it can be jumped to:

```rteasy
~declare register A(3:0)
~
# A state with the label MY_LABEL.
MY_LABEL: A <- A + 1;

# ...

# Resume execution at MY_LABEL
goto MY_LABEL;
```

## Conditional Branch

Since registers are clocked, it is not possible to jump in one cycle depending on the result of a register assignment. Instead two cycles are needed. For this purpose, a state can have a conditional branch separated by the pipe symbol. After the pipe, all assignments have already taken place. The conditional branch may only contain if, switch and goto operations. With this, it is possible in some cases to save a clock cycle by writing:

```rteasy
~declare register COUNTER(7:0)
~
LOOP:  nop; # do something
CHECK: COUNTER <- COUNTER + 1 | if COUNTER < 20 then goto LOOP fi;
```

instead of:

```rteasy
~declare register COUNTER(7:0)
~
LOOP:  nop; # do something
INC:   COUNTER <- COUNTER + 1;
CHECK: if COUNTER < 20 then goto LOOP fi;
```
