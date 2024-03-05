// Copyright © 2023-2024 andre4ik3
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

//! Launcher Persistence Module
//! ===========================
//!
//! This module is responsible for persisting data between app restarts. It provides two ways of
//! doing so - the [FileRegistry] and the [DirectoryRegistry].
//!
//! The [FileRegistry] is the easiest to explain, understand, and use. The main thing it does is
//! read and write *something* to/from a file. The *something* can be any type, as long as it
//! implements [Default], [serde::Deserialize], and [serde::Serialize]. The data can also optionally
//! be encrypted - the registry will fully manage the encryption. Other than that, it also manages
//! access to the data, meaning it is safe to access concurrently (there can be any number of
//! readers, but only 1 writer concurrently). This allows it to be used as global state. In the
//! Launcher it is used in 2 places - the app config and the account credentials.
//!
//! The [DirectoryRegistry] is a bit trickier. It is similar to the [FileRegistry], however, instead
//! of holding just one of T, it holds multiple instances (like a [Vec]). Each element is saved to a
//! file, with the file name being tied to the element as its' ID (so now our [Vec] is a
//! [std::collections::HashMap]). The final piece of the puzzle is that each file is stored in its
//! own directory (hence the name), essentially giving each element its' own permanent directory.
//! This is perfect for things like Java installations, downloaded asset catalogs, and instances,
//! as each individual element is indexed.

pub use registry::{directory::DirectoryRegistry, Result, Error, file::FileRegistry};

pub(crate) mod crypto;
pub(crate) mod registry;
