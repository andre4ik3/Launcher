// Copyright © 2023 andre4ik3
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

extern crate proc_macro;

use proc_macro::TokenStream;

use darling::{Error, FromMeta};
use darling::ast::NestedMeta;
use quote::{quote, TokenStreamExt};
use syn::{Data, DeriveInput, parse_macro_input};
use syn::parse::Parse;

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    strict: Option<bool>,
    #[darling(default)]
    untagged: Option<bool>,
}

/// A shortcut for `#[derive(Clone, Debug, serde::Deserialize)]`.
#[proc_macro_attribute]
pub fn api_response(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(err) => {
            return TokenStream::from(Error::from(err).write_errors());
        }
    };

    // This is the code that we are augmenting (i.e. what is underneath our macro).
    let ast = parse_macro_input!(item as DeriveInput);

    // This is what we are going to append.
    let base = quote! { #[derive(Clone, Debug, serde::Deserialize)] };
    let mut extra = quote! {};

    // These are the arguments that are passed between the parentheses of the macro invocation.
    let args = match MacroArgs::from_list(&attr_args) {
        Ok(args) => args,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    // Add stuff to extra based on toggles and what we are modifying (struct or enum).
    if let Data::Enum(_) = &ast.data {
        if args.untagged.unwrap_or(true) {
            extra.append_all(vec![quote! { #[serde(untagged)] }]);
        }
    };

    if args.strict.unwrap_or(true) {
        extra.append_all(vec![quote! { #[serde(deny_unknown_fields)] }]);
    }

    (quote! {
        #base
        #extra
        #ast
    })
        .into()
}
