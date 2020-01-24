# f469

Learning Rust hard(ware) way...

To run & debug:

`openocd` in one terminal window (install it if you don't have it)

`cargo run --example <example_name>` in another one

then in gdb:
```
(gdb) continue
(gdb) next
// or whatever
```

Examples for the board:

- [x] `hello` - hello world to the debugger, 
- [x] `blinky` - blink with 4 LEDs on the board, 
- [x] `serialecho` - echo back everything received to serial, 
- [x] `hashes` - bitcoin_hashes crate, sha256 of a fixed sentence, 
- [ ] `ecc` - rust-secp256k1 crate, create privkey, derive pubkey, sign and verify a message.

