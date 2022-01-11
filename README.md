After using `cargo expand`:

```rust
[hello-impl/src/lib.rs:34] &tokens = TokenStream [
    Ident {
        ident: "let",
        span: #0 bytes(114..117),
    },
    Ident {
        ident: "_failed",
        span: #0 bytes(118..125),
    },
    Punct {
        ch: '=',
        spacing: Alone,
        span: #0 bytes(126..127),
    },
    Literal {
        kind: Integer,
        symbol: "0",
        suffix: None,
        span: #0 bytes(128..129),
    },
    Punct {
        ch: ';',
        spacing: Alone,
        span: #0 bytes(129..130),
    },
]
error: unexpected token
 --> src/main.rs:7:9
  |
7 |         let _failed = 0;
  |         ^^^

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    let _succeeded = 1;
    ();
}
```
