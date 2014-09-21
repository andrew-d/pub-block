# pub-block

pub-block is a simple syntax extension for Rust that helps when making things
public.  It provides two primitives: the `public!` macro - which makes all
top-level items within it public - and the `#[pub_fields]` attribute, which
makes all fields of the decorated structure public.

For a full usage example,
[see here](https://github.com/andrew-d/pub-block/blob/master/examples/test.rs).
