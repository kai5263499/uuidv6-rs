![test](https://github.com/kai5263499/uuidv6-rs/actions/workflows/tests.yml/badge.svg)

# uuidv6-rs

Reference implementation of UUIDv6 expressed in Rust 

=======================

## Draft proposal

The draft proposal of the UUIDv6 spec along with examples and rationale is available [here](https://datatracker.ietf.org/doc/html/draft-peabody-dispatch-new-uuid-format).

Another informative source which contains reference implementations for generating UUIDv6's in go and converting UUIv1 to UUIDv6 in Python is available [here](http://gh.peabody.io/uuidv6/).

# Byte layout

Bytes 0-7: (each digit shown is hex, 4 bits)
```
    00 00 00 00  00 00 00 00
    |                | ||  |
     ----------------  ||  |
    timestamp          ||  |
    bits 59-12         ||  |
                 version|  |
                         --
                  timestamp
                  bits 11-0
```

Bytes 8-15: (same as RFC 4122)
```
    00 00 00 00  00 00 00 00
    ||  | |                |
    ||  |  ________________
    ||  |       node
    | --
    | clock seq bits 11-0
    2 bits variant, 2 bits
    are 13-12 of clock seq
```