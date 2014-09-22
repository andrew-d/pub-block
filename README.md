# pub_block

pub-block is a simple syntax extension for Rust that helps when making things
public.  It provides two primitives: the `public!` macro - which makes all
top-level items within it public - and the `#[pub_fields]` attribute, which
makes all fields of the decorated structure public.

## Usage

To use `pub_block`, put this in your `Cargo.toml`:

```toml
[dependencies.pub_block]
git = "https://github.com/andrew-d/pub-block"
```

And add this to your crate root:

```rust
#![feature(phase)]

#[phase(plugin, link)]
extern crate pub_block;
```

After a `cargo update pub_block`, you're good to go! For a full example of
how to use the `public!` macro in your code, [see
here](https://github.com/andrew-d/pub-block/blob/master/examples/test.rs).
