# Grammar
### The purpose of this document is to explain the grammar of the assembly language that is assembled by the assembler.

- All text following a `;` is ignored.
- Lines are whitespace terminated.
- Each line may have one operation (`mov`, `adi`, `str`, etc) and zero to two arguments.
- Numbers prefixed with `0x` will be interpreted as hexadecimal.
- Unless otherwise specified, numbers will be interpreted as decimal.
