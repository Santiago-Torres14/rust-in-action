# Data in depth

## Bit patterns and types

Everything we see in our computers (and in
electronic devices) communicate in binary
that means they only understand 1's and 0's
But humans doesn't interpret those values
as good as a computer does, so we need to
translate them into readable format.

That's where types enter the room, one byte
(4 bits)  can represent many things depending 
on its type. Files such as images, CSV, JSON
are all bytes interpreted in different formats.


```rust
// 65,535 is the max value a 16 bit integer can hold
const SIXTYFIVETHOUSAND_535: u16 = 0b1111_1111_1111_1111;
```

## Endianness

Endianness is the way bits are arranged, and
read. Some CPUs use big endian (left to right)
but most current CPUs use little endian (right
to left).

```rust
const BIG_ENDIAN: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
const LITTLE_ENDIAN: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];
```


