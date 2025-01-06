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

//! Launcher Utilities Module
//! =========================
//!
//! This module is full of random code that doesn't distinctly belong anywhere else or warrant its
//! own separate module, yet is still used in various places throughout the code. It includes:
//!
//! - [directories] - Abstractions for common directories across platforms.
//! - [log] - Functions for setting up logging and panic hooks.
//! - [platforms] - Constants for the current platform.

pub use macros::*;

pub mod archive;
pub mod directories;
pub mod log;
pub mod platforms;
