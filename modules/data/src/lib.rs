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

//! Launcher Data Module
//! ====================
//!
//! This module contains data models for different APIs and services. The module itself is split
//! into many submodules that categorize the data models based on where they are used:
//!
//! - [auth] - Models for storing user accounts and credentials.
//! - [core::assets]
//! - [core::conditional] - Data-driven condition API.
//! - [core::game]
//! - [core::java] - Models for downloading and using Java builds.
//! - [silo] - Models for APIs used during Silo data generation (requires `silo` feature).
//! - [web::microsoft] - Models for interacting with Microsoft's authentication API.

pub mod auth;
pub mod core;
pub mod silo;
pub mod web;
