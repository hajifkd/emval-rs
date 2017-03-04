# emval-rs

A rust wrapper for emscripten [val.h](https://kripken.github.io/emscripten-site/docs/api_reference/val.h.html)

Using val.h, the Javascript code can be called from rust.

# Usage

```rust
extern crate emval;
use emval::*;

let window = JSObj::global("window");
let name: String = window.call_prop("prompt", args!("What is your name?"));
window.call_prop::<()>("alert", args!(format!("Hi, {} from rust!", name)));
```

# build

```sh
$ EMMAKEN_CFLAGS=--bind cargo build --target=asmjs-unknown-emscripten
```

