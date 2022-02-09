# Errors

An overview of all possible errors that can occur during compilation.

## E001

This error indicates that a register, bus or memory is assigned more than once in a cycle. Only one assignment to an item may be executed per execution path and cycle.

### Examples

```rteasy
declare register X(3:0)

X <- 2, X <- 1; # error: register "X" is assigned more than once
```

```rteasy
declare register AR(3:0), DR(3:0)
declare memory MEM(AR, DR)

read MEM, read MEM; # error: register "DR" is assigned more than once
write MEM, write MEM; # error: memory "MEM" is assigned more than once
```

## E002

TODO: ...

## E003

TODO: ...

## E004

TODO: ...

## E005

TODO: ...

## E006

TODO: ...

...

TODO: ...
