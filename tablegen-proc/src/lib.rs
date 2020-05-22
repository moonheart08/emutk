extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, Spacing, Punct};
use proc_macro2;
use syn::{braced, parse_macro_input, token, LitInt, Result, Token, Expr, Type};
use syn::parse::{Parse, ParseStream};
use quote::*;

// oh god this code is horrible. Note to people looking: Please do not use this code
// anywhere else. It is likely full of holes.

#[derive(Clone)]
struct TableEntry {
    index: LitInt,
    idx_i: usize,
    ty: Ident,
    value: Expr,
}

struct Table {
    table_name: Ident, 
    table_len: LitInt,
    table_type: Type,
    fill: Expr,
    len_i: usize,
    brace: token::Brace,
    entries: Vec<TableEntry>,
}

impl Parse for Table {
    fn parse(input: ParseStream) -> Result<Self> {
        let table_name: Ident = input.parse()?;
        let _: Token![;] = input.parse()?;
        let table_len: LitInt = input.parse()?;
        let len_i = table_len.base10_parse::<usize>()?;
        let _: Token![;] = input.parse()?;
        let table_type: Type = input.parse()?;
        let _: Token![;] = input.parse()?;
        let fill: Expr = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let brace_content;
        let brace = braced!(brace_content in input);

        let mut entries: Vec<TableEntry> = vec![];

        loop {
            if brace_content.peek(LitInt) {
                // parse out another entry
                let index: LitInt = brace_content.parse()?;
                let idx_i = index.base10_parse::<usize>()?;
                let _: Token![=>] = brace_content.parse()?;
                let ty: Ident = brace_content.parse()?;
                let _: Token![,] = brace_content.parse()?;
                let value: Expr = brace_content.parse()?;
                let _: Token![;] = brace_content.parse()?;
                entries.push(TableEntry {
                    index,
                    idx_i,
                    ty,
                    value,
                });
            } else {
                break;
            }
        }

        Ok(Table {
            table_name,
            table_len,
            table_type,
            fill,
            len_i,
            brace,
            entries,
        })
    }
}

#[proc_macro]
pub fn gen_instr_table(items: TokenStream) -> TokenStream {
    let input = parse_macro_input!(items as Table);
    
    // could overflow memory. But I could care less.
    let int_table_len = input.len_i;
    let mut final_table: Vec<Option<TableEntry>> = vec![None; int_table_len];
    for i in input.entries {
        if i.idx_i >= final_table.len() {

        }
        final_table[i.idx_i] = Some(i.clone());
    }

    let mut out_table_interior = proc_macro2::TokenStream::new();
    for i in final_table {
        if let Some(v) = i {
            v.value.to_tokens(&mut out_table_interior);
        } else {
            input.fill.to_tokens(&mut out_table_interior);
        }
        out_table_interior.append(Punct::new(',', Spacing::Alone));
    }

    let tablename = input.table_name;
    let len = input.table_len;
    let ty = input.table_type;
    let result = quote!{
        [#out_table_interior]
    };
    
    result.into()
}