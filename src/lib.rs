#![crate_name="pub_block"]
#![crate_type="dylib"]

#![feature(quote, phase, plugin_registrar, macro_rules)]

extern crate syntax;
extern crate rustc;

#[phase(plugin, link)]
extern crate log;

use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{ExtCtxt, MacResult, MacItems, Modifier};
use syntax::fold::{Folder, noop_fold_struct_field};
use syntax::parse;
use syntax::parse::token::intern;
use syntax::ptr::P;

use rustc::plugin::Registry;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(registrar: &mut Registry) {
    registrar.register_macro("public", pub_block);
    registrar.register_syntax_extension(intern("pub_fields"), Modifier(box pub_fields));
}

fn pub_block(cx: &mut ExtCtxt, _sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult+'static> {
    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), tts.to_vec());

    let mut returned_items = vec![];

    loop {
        let i = match parser.parse_item(vec![]) {
            Some(i) => {
                debug!("Item ident: {}", i.ident.as_str());

                P(ast::Item {
                    ident: i.ident,
                    attrs: i.attrs.clone(),
                    id:    i.id,
                    node:  i.node.clone(),
                    vis:   ast::Public,
                    span:  i.span,
                })
            },
            None => break,
        };

        returned_items.push(i);
    }

    MacItems::new(returned_items.into_iter())
}

fn pub_fields(_cx: &mut ExtCtxt, _sp: codemap::Span, _meta: &ast::MetaItem, it: P<ast::Item>) -> P<ast::Item> {
    let mut folder = StructFieldFolder::new();
    let res = folder.fold_item(it);

    res.expect_one("expected StructFieldFolder to return a single Item")
}

struct StructFieldFolder;

impl StructFieldFolder {
    fn new() -> StructFieldFolder {
        StructFieldFolder
    }
}

impl Folder for StructFieldFolder {
    fn fold_struct_field(&mut self, sf: ast::StructField) -> ast::StructField {
        let ret = noop_fold_struct_field(sf, self);

        // Modify the field to be public.
        let new_kind = match ret.node.kind {
            ast::NamedField(id, _) => ast::NamedField(id, ast::Public),
            ast::UnnamedField(_)   => ast::UnnamedField(ast::Public),
        };

        let new_field = ast::StructField_ {
            kind:  new_kind,
            id:    ret.node.id,
            ty:    ret.node.ty,
            attrs: ret.node.attrs,
        };

        let new_ret = ast::StructField {
            node: new_field,
            span: ret.span,
        };

        new_ret
    }
}
