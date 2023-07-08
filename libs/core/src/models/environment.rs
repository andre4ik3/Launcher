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

use std::env::consts;
use std::str::FromStr;

use platforms::{Arch, OS};
use serde::{Deserialize, Serialize};

/// A struct that describes the environment the binary is running in.
/// (Somewhat similar to a target triple)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Environment {
    pub os: OS,
    pub arch: Arch,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            os: OS::from_str(consts::OS).unwrap(),
            arch: Arch::from_str(consts::ARCH).unwrap(),
        }
    }
}
