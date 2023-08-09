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

use platforms::{Arch, OS};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::env::consts;
use std::str::FromStr;

/// Condition for inclusion of arguments and libraries.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Condition {
    Feature(String),
    OS((OS, VersionReq)),
    Arch(Arch),

    /// Inverts the condition.
    Not(Box<Condition>),
    /// Returns true if all conditions are true, else false.
    And(Box<[Condition]>),
    /// Returns true if at least one condition is true, else false.
    Or(Box<[Condition]>),
    /// Returns true if only one condition is true, else false.
    Xor(Box<[Condition]>),
}

#[cfg(feature = "eval")]
impl Condition {
    pub fn eval(&self, features: &[String]) -> bool {
        match self {
            Condition::Feature(feature) => features.contains(feature),
            Condition::OS((os, req)) => {
                let os_matches = os == &OS::from_str(consts::OS).unwrap();
                let ver_matches = match os_info::get().version() {
                    os_info::Version::Semantic(major, minor, patch) => {
                        req.matches(&Version::new(*major, *minor, *patch))
                    }
                    _ => true, // Unimplemented
                };
                os_matches && ver_matches
            }
            Condition::Arch(arch) => arch == &Arch::from_str(consts::ARCH).unwrap(),
            Condition::And(conditions) => conditions.iter().map(|v| v.eval(features)).all(|v| v),
            Condition::Or(conditions) => conditions.iter().map(|v| v.eval(features)).any(|v| v),
            Condition::Xor(conditions) => conditions
                .iter()
                .map(|v| v.eval(features) as usize)
                .reduce(|a, b| a + b)
                .unwrap_or(0)
                .eq(&1),
            Condition::Not(condition) => !condition.eval(features),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Conditional<T> {
    when: Condition,
    then: T,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MaybeConditional<T> {
    Unconditional(T),
    Conditional(Conditional<T>),
}
