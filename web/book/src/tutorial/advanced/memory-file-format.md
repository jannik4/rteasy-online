# Memory File Format

The memory file format is a simple line-based file format.

## Header

The first line is the header, which indicates in which base the numbers are stored and how large the memory is. The header must always be in the following shape:

```rteasy,ignore
[B|b|H|h] <ADDRESS_SIZE> <DATA_SIZE>
```

`B` or `b` means binary, `H` or `h` means hexadecimal. `ADDRESS_SIZE` specifies the bit width of the address space and `DATA_SIZE` specifies the bit width of the data. For example, a memory that is stored in a binary base, with 65536 (= 2^16) entries and a width of 1 byte, has the following header:

```rteasy,ignore
B 16 8
```

## Data

After the header, the data is stored line by line. The first line, unless otherwise specified, is at address 0. Subsequent lines are always located at the next address. For example, the following describes a memory with the numbers `0x1`, `0x7` and `0xF1` at address `0x0`, `0x1` and `0x2`:

```rteasy,ignore
H 4 16

1
7
F1
```

Additionally, it is possible to store data at a specific address. With `<ADDRESS>:` the address for the next line can be specified. For example, in the following, the values `0xFF` and `0xC` are stored at the addresses `0x9` and `0xA`:

```rteasy,ignore
H 4 16

9:
FF
C
```

Of course, this can be combined in any way:

```rteasy,ignore
H 4 16

3
4

C:
2
1

3:
25
```

## Comments

The memory format allows simple line comments starting with the hash (#) character. Comments are allowed in all lines including the header. Example:

```rteasy,ignore
H 8 32 # Memory in hexadecimal base

# Hello World

4: # Store some data, starting at address 4
FF # (addr = 4)
0  # (addr = 5)
1  # (addr = 6)
2  # (addr = 7)
```
