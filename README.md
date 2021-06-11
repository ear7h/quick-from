# `quick_from`

A derive macro for quickly implementing `From` on on enum variants that wrap
other types.

## Example

```rust
#[macro_use]
extern crate quick_from;

use std::{io, fs};

#[derive(QuickFrom)]
enum Error {
    InvalidInput,

    #[quick_from]
    Io(io::Error),
}

fn my_read(s : &str) -> Result<Vec<u8>, Error> {
    if s.len() == 0 {
        return Err(Error::InvalidInput)
    }

    Ok(fs::read(s)?)
}
```

