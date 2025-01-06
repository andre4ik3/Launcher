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

use std::collections::HashSet;
use std::env::consts;
use std::str::FromStr;

use platforms::{Arch, OS};

use macros::data_structure;

/// Condition for inclusion of arguments and libraries.
#[derive(Hash)]
#[data_structure(equatable)]
pub enum Condition {
    Always,
    Never,
    Feature(String),
    OS(OS),
    Arch(Arch),
    Not(Box<Condition>),
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Xor(Vec<Condition>),
}

/// A helper enum for expressing a value that may or may not have an associated condition.
#[data_structure]
pub enum MaybeConditional<T> {
    Unconditional(T),
    Conditional {
        when: Condition,
        then: T,
    },
}

// === impl ===

macro_rules! simplify_impl {
    ( $x:expr, $conditions:ident ) => {{
        let conditions: HashSet<Condition> = $conditions
            .into_iter()
            .filter(|condition| !condition.is_empty())
            .map(|cond| cond.simplify())
            .collect();

        let mut conditions: Vec<Condition> = conditions.into_iter().collect();
        let result = match conditions.len() {
            0 => Self::Always,
            1 => conditions.swap_remove(0),
            _ => $x(conditions),
        };

        match result {
            Condition::And(it) if it.contains(&Condition::Never) => Condition::Never,
            Condition::Or(it) if it.contains(&Condition::Always) => Condition::Always,
            other => other,
        }
    }};
}

impl Condition {
    /// Evaluates a condition to a boolean.
    pub fn eval(&self, features: &Vec<String>) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::Feature(feature) => features.contains(feature),
            Self::OS(os) => os == &OS::from_str(consts::OS).unwrap(),
            Self::Arch(arch) => arch == &Arch::from_str(consts::ARCH).unwrap(),
            Self::Not(val) => !val.eval(features),
            Self::And(vals) => vals.iter().all(|v| v.eval(features)),
            Self::Or(vals) => vals.iter().any(|v| v.eval(features)),
            Self::Xor(vals) => vals
                .iter()
                .map(|v| v.eval(features) as usize)
                .reduce(|a, b| a + b)
                .unwrap_or(0)
                .eq(&1),
        }
    }

    /// Checks if a condition is empty (aka a no-op). Used to simplify expressions.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::And(vals) | Self::Or(vals) => vals.is_empty(),
            Self::Not(val) => val.is_empty(),
            _ => false,
        }
    }

    /// Simplifies a condition.
    pub fn simplify(self) -> Self {
        match self {
            Self::And(vals) => simplify_impl!(Self::And, vals),
            Self::Or(vals) => simplify_impl!(Self::Or, vals),
            Self::Xor(vals) => simplify_impl!(Self::Xor, vals),
            Self::Not(val) => match *val {
                Self::Always => Self::Never,
                Self::Never => Self::Always,
                Self::Not(val) => val.simplify(),
                _ => val.simplify(),
            },
            _ => self,
        }
    }
}

impl<T> MaybeConditional<T> {
    /// Evaluates the inner condition (if one exists) and returns the inner value expressed as an
    /// [Option].
    pub fn fold(self, features: &Vec<String>) -> Option<T> {
        match self {
            Self::Unconditional(val) => Some(val),
            Self::Conditional { when, then } => match when.eval(features) {
                true => Some(then),
                false => None
            },
        }
    }
}

// === conversion ===

#[cfg(feature = "silo")]
impl From<crate::silo::game::ApiOs> for OS {
    fn from(value: crate::silo::game::ApiOs) -> Self {
        match value {
            crate::silo::game::ApiOs::Linux => Self::Linux,
            crate::silo::game::ApiOs::MacOS => Self::MacOS,
            crate::silo::game::ApiOs::Windows => Self::Windows,
        }
    }
}

#[cfg(feature = "silo")]
impl From<crate::silo::game::ApiArch> for Arch {
    fn from(value: crate::silo::game::ApiArch) -> Self {
        match value {
            crate::silo::game::ApiArch::X86_64 => Self::X86_64,
        }
    }
}

#[cfg(feature = "silo")]
impl From<crate::silo::game::ApiLibraryRule> for Condition {
    fn from(value: crate::silo::game::ApiLibraryRule) -> Self {
        let mut conditions = vec![];

        if let Some(os) = value.os {
            if let Some(name) = os.name {
                let name = OS::from(name);

                // Our app only runs on macOS >13 and Windows >11. So version requirements like
                // macOS 10.5 and Windows 10 are irrelevant.
                if let Some(version) = os.version {
                    match (version.as_str(), &name) {
                        ("^10\\.5\\.\\d$", OS::MacOS) => conditions.push(Condition::Never),
                        ("^10\\.", OS::Windows) => conditions.push(Condition::Never),

                        // Assuming Microsoft will at some point change Win11 to actually be Win11.
                        ("^11\\.", OS::Windows) => conditions.push(Condition::Never),

                        // We don't have an implementation of OS version requirement checking (yet).
                        _ => todo!()
                    }
                }

                conditions.push(Condition::OS(name));
            }

            if let Some(arch) = os.arch {
                conditions.push(Condition::Arch(Arch::from(arch)));
            }
        }

        if let Some(features) = value.features {
            for (feature, _) in features.into_iter().filter(|(_, v)| *v) {
                conditions.push(Condition::Feature(feature));
            }
        }

        match value.action {
            crate::silo::game::ApiLibraryRuleAction::Allow => Condition::And(conditions),
            crate::silo::game::ApiLibraryRuleAction::Disallow => {
                Condition::Not(Box::new(Condition::And(conditions)))
            }
        }
    }
}

#[cfg(feature = "silo")]
impl From<crate::silo::game::ApiModernGameArgument> for Vec<MaybeConditional<String>> {
    fn from(value: crate::silo::game::ApiModernGameArgument) -> Self {
        match value {
            crate::silo::game::ApiModernGameArgument::Plain(val) => vec![MaybeConditional::Unconditional(val)],
            crate::silo::game::ApiModernGameArgument::Conditional { rules, value } => {
                let condition = Condition::Or(rules.into_iter().map(Condition::from).collect()).simplify();
                match value {
                    crate::silo::game::ApiModernGameRuleValue::String(val) => vec![MaybeConditional::Conditional {
                        when: condition,
                        then: val,
                    }],
                    crate::silo::game::ApiModernGameRuleValue::Array(vals) => vals.into_iter().map(|val| MaybeConditional::Conditional {
                        when: condition.clone(),
                        then: val,
                    }).collect()
                }
            }
        }
    }
}

// === test ===

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval() {
        let features = vec!["feature-1".to_string(), "feature-2".to_string()];

        // These should always be true and false respectively
        assert!(Condition::Always.eval(&features));
        assert!(!Condition::Never.eval(&features));

        // Should be true for current OS
        let current_os = OS::from_str(consts::OS).unwrap();
        assert!(Condition::OS(current_os).eval(&features));
        assert!(!Condition::OS(OS::Unknown).eval(&features));

        // Should be true for current arch
        let current_arch = Arch::from_str(consts::ARCH).unwrap();
        assert!(Condition::Arch(current_arch).eval(&features));
        assert!(!Condition::Arch(Arch::PowerPc).eval(&features));

        // Should be true for current feature set
        assert!(Condition::Feature(features[0].clone()).eval(&features));
        assert!(!Condition::Feature("some-random-feature".to_string()).eval(&features));
    }

    #[test]
    fn simplify() {
        // Empty arrays should simplify to be always true
        assert_eq!(Condition::And(vec![]).simplify(), Condition::Always);
        assert_eq!(Condition::Or(vec![]).simplify(), Condition::Always);
        assert_eq!(Condition::Xor(vec![]).simplify(), Condition::Always);
        assert_eq!(
            Condition::Not(Box::new(Condition::Not(Box::new(Condition::Always)))).simplify(),
            Condition::Always
        );

        // Single arrays should be unwrapped to the inner condition
        let feature = Condition::Feature("testing-feature-123".to_string());
        assert_eq!(Condition::And(vec![feature.clone()]).simplify(), feature);
        assert_eq!(Condition::Or(vec![feature.clone()]).simplify(), feature);
        assert_eq!(Condition::Xor(vec![feature.clone()]).simplify(), feature);
        assert_eq!(
            Condition::Not(Box::new(Condition::Not(Box::new(feature.clone())))).simplify(),
            feature
        );

        // Simplification of And(..., Never, ...) and Or(..., Always, ...) should always be false and true
        assert_eq!(Condition::And(vec![feature.clone(), Condition::Never, feature.clone()]).simplify(), Condition::Never);
        assert_eq!(Condition::Or(vec![feature.clone(), Condition::Always, feature.clone()]).simplify(), Condition::Always);

        // Simplifying already simplified condition should be a no-op
        assert_eq!(feature.clone().simplify(), feature);
    }
}
