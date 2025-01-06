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

use proc_macro::TokenStream;
use darling::{FromMeta, Error};

/// Helper function to parse incoming attribute macro parameters.
pub fn parse_params<T: FromMeta>(attr: TokenStream) -> Result<T, Error> {
    let args = match darling::ast::NestedMeta::parse_meta_list(attr.into()) {
        Ok(args) => args,
        Err(err) => return Err(Error::from(err)),
    };

    T::from_list(&args)
}
