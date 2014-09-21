#![feature(phase)]

#[phase(plugin, link)]
extern crate pub_block;


mod submod {
    public! {
        fn a_function() -> int {
            1234
        }

        fn another_func() -> &'static str {
            "test 123"
        }
    }

    #[pub_fields]
    pub struct HasFields {
        foo: uint,
        bar: bool,
    }

    pub fn get_struct() -> HasFields {
        HasFields {
            foo: 5678,
            bar: true,
        }
    }
}


fn main() {
    println!("Running");
    println!("functions: value1 = {}, value2 = {}",
        submod::a_function(),
        submod::another_func());

    let s = submod::get_struct();
    println!("struct: foo = {}, bar = {}", s.foo, s.bar);
}
