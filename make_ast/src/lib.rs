// #![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use quote::quote;
use syn::{Expr, Token, Ident, Result, Lit};

macro_rules! for_all_unops {
    ($call:ident,$f:ident, $n:ident, $depth:ident) => {
        $call!{$f, $n, $depth, Neg Not}
    }
}
macro_rules! impl_unop {
    ($f:ident, $n:ident, $depth:ident, $($name:ident)*) => {
        let target = parse_expr($f, &$n.expr, $depth, 0);
        let func = $f;
        match &$n.op {
            $(
                syn::UnOp::$name(_) => {
                    quote!{
                        #func.$name(&#target)
                    }
                },
            )*
            _ => {
                panic!("cannnot handle UnOp {:?}", $n.op)
            },

        }
    }
}
macro_rules! for_all_binops {
    ($call:ident,$f:ident, $n:ident, $depth:ident) => {
        $call!{$f, $n, $depth, Eq Ne Lt Gt Le Ge Add Sub Mul Div Rem And Or}
    }
}
macro_rules! impl_binop {
    ($f:ident, $n:ident, $depth:ident, $($name:ident)*) => {
        let left = parse_expr($f, &$n.left, $depth, 0);
        let right = parse_expr($f, &$n.right, $depth, 1);
        let func = $f;
        match &$n.op {
            $(
                syn::BinOp::$name(_) => {
                    quote!{
                        #func.$name(&#left, &#right)
                    }
                },
            )*
            _ => {
                panic!("cannnot handle BinOp {:?}", $n.op)
            },
        }
    }
}

fn parse_expr(f: &Ident, e: &Expr, depth: u32, _index: u32) -> proc_macro2::TokenStream {
//    println!("[{}:{}] expr()='{:#?}'", depth, _index, e);
    let depth = depth + 1;
    match e {
        Expr::Unary(n) => {
            for_all_unops!{impl_unop, f, n, depth}
        },
        Expr::Binary(n) => {
            for_all_binops!{impl_binop, f, n, depth}
        },
        Expr::Paren(n) => {
            let target = parse_expr(f, &n.expr, depth, 0);
            quote!{
                #target
            }
        },
        Expr::Lit(lit) => {
            match &lit.lit {
                Lit::Int(litint) => {
                    // println!("litint = {:?} ", litint);
                    quote! {
                        #f.ct_int(#litint)
                    }
                },
                _ => {
                    panic!("cannnot handle Lit {:?}", lit.lit)
                },
            }
        },
        _ => {
            panic!("cannnot handle Expr {:?}", e)
        }
    }
}

struct Recipe {
    name: Ident,
    _delim: Token![,],
    expr: Expr,
}
impl syn::parse::Parse for Recipe {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _delim: input.parse()?,
            expr: input.parse()?,
        })
    }
}
#[proc_macro]
pub fn build(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Recipe {
        name,
        _delim,
        expr,    
    } = syn::parse_macro_input!(input);
//    println!("name = {}", &name);
    let expanded = parse_expr(&name, &expr, 0, 0);
    println!("\t expanded = {}", &expanded);
    expanded.into()
}
