#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, box_syntax)]

extern crate syntax;
extern crate rustc;

use syntax::ast::{Ident, Mod, MetaItem};
use syntax::ast::Item_::{ItemMod, ItemFn};
use syntax::ast::MetaItem_::{MetaNameValue};
use syntax::ast::Lit_::{LitStr};
use syntax::parse::token::{intern, InternedString};
use syntax::ext::base::{ExtCtxt, MultiDecorator, Annotatable};
use syntax::ext::base::Annotatable::Item;
use syntax::codemap::Span;
use syntax::ptr::P;

use rustc::plugin::Registry;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::cell::RefCell;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("points"), MultiDecorator(box do_it));
}

thread_local!(static FNS_IN_MOD: RefCell<HashMap<Ident, Ident>>
                = RefCell::new(HashMap::new()));
thread_local!(static RESULT_FILE: RefCell<File>
                = RefCell::new(File::create("tmc-points.txt").unwrap()));

fn do_it(_: &mut ExtCtxt,
         _: Span,
         meta: &MetaItem,
         item: &Annotatable,
         _: &mut FnMut(Annotatable)) {
    if let Some(value) = resolve_value(meta) { 
        if let &Item(ref i) = item {
            match i.node {
                ItemMod(Mod{ref items, ..}) => {
                    handle_module(i.ident, value, items);
                }
                ItemFn(..) => {
                    handle_function(&i.ident, value);
                }
                _ => {}
            }
        }
    }
}

fn handle_module(ident: Ident, value: &str, items: &Vec<P<syntax::ast::Item>>) {
    for item in items {
        if let ItemFn(..) = item.node {
            FNS_IN_MOD.with(|fun| {
                fun.borrow_mut()
                   .insert(item.ident, ident);
            });
        }
    }
    RESULT_FILE.with(|file| {
        let mut file_borrow = file.borrow_mut();
        write!(file_borrow, "{} = {}\n", ident, value).unwrap();
    });
}

fn handle_function(ident: &Ident, value: &str) {
    FNS_IN_MOD.with(|fun| {
        RESULT_FILE.with(|file| {
            let mut file_borrow = file.borrow_mut();
            if let Some(s) = fun.borrow().get(ident) {
                write!(file_borrow, "{}.{} = {}\n", s, ident, value).unwrap();
            } else {
                write!(file_borrow, "{} = {}\n", ident, value).unwrap();
            }
        });
    });
}

fn resolve_value(meta: &MetaItem) -> Option<&InternedString> {
    if let MetaNameValue(_, ref lit) = meta.node {
        if let LitStr(ref string, _) = lit.node {
            return Some(string);
        }
    }
    None
}
