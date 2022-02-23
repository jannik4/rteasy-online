# Declarations

Before the actual algorithm, all items must first be defined. A declaration is composed of a `declare` followed by the type of the item, e.g. `register`, and finally the list of items you want to declare, separated by commas. All names may only contain uppercase letters, digits or underscores, whereby the first character may not be a digit.

## Registers

Declare two registers `R` and `C`. Register `R` is 8 bits wide and ranges from 7 (MSB) to 0 (LSB). The register `C` is 1 bit wide, with the bit at position 0.

```rteasy
declare register R(7:0), C
```

## Buses

Declare two buses `B` and `SECOND_BUS`. Bus `B` is 8 bits wide and ranges from 7 (MSB) to 0 (LSB). The bus `SECOND_BUS` is 1 bit wide, with the bit at position 5.

```rteasy
declare bus B(7:0), SECOND_BUS(5)
```

## Register Arrays

Declare a register array named `ARR`. As with registers and buses, a bit range can be specified. The length of the register array is specified in brackets. The length must be a power of two.

```rteasy
declare register array ARR(7:0)[4]
```

## Memories

Declare a memory named `MEM`. Memories require two registers, whereby the first is the address register and the second is the data register. So in this case `AR` is the address register and `DR` is the data register. Thus the memory is of size 2^16 = 64 KByte and 1 byte wide.

```rteasy
declare register AR(15:0), DR(7:0)
declare memory MEM(AR, DR)
```

## Inputs/Outputs

...
