// Copyright © 2023-2025 andre4ik3
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

use crate::utils::parse_params;
use darling::FromMeta;
use quote::{TokenStreamExt, quote};
use syn::{DeriveInput, parse_macro_input};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    rename: Option<String>,
}

pub fn api_request(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args: MacroArgs = match parse_params(attr) {
        Err(err) => return TokenStream::from(err.write_errors()),
        Ok(args) => args,
    };

    // This is the code that we are augmenting (i.e. what is underneath our macro).
    let ast = parse_macro_input!(item as DeriveInput);

    // This is what we are going to append.
    let base = quote! { #[derive(Clone, Debug, serde::Serialize)] };
    let mut extra = quote! {};

    // Add stuff to extra based on toggles and what we are modifying (struct or enum).
    if let Some(rename) = args.rename {
        extra.append_all(vec![quote! { #[serde(rename_all = #rename)] }]);
    }

    (quote! {
        #base
        #extra
        #ast
    })
    .into()
}
