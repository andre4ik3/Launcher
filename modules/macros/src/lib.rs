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

//! Launcher Macros
//! ===============
//!
//! This module contains procedural macros that are used in various different places, most notably
//! in the `data` module. They reduce code repetition and centralize the various traits that our
//! data structures and API responses implement.

extern crate proc_macro;

use proc_macro::TokenStream;

mod api_request;
mod api_response;
mod data_structure;
mod utils;

/// A more concise way to write `#[derive()]` macros for data structures (two-way serialization).
#[proc_macro_attribute]
pub fn data_structure(attr: TokenStream, item: TokenStream) -> TokenStream {
    data_structure::data_structure(attr, item)
}

/// A more concise way to write `#[derive()]` macros for API requests (only serialization).
#[proc_macro_attribute]
pub fn api_request(attr: TokenStream, item: TokenStream) -> TokenStream {
    api_request::api_request(attr, item)
}

/// A more concise way to write `#[derive()]` macros for API responses (only deserialization).
#[proc_macro_attribute]
pub fn api_response(attr: TokenStream, item: TokenStream) -> TokenStream {
    api_response::api_response(attr, item)
}
