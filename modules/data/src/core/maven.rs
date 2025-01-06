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

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MavenIdentifier {
    pub group: String,
    pub artifact: String,
    pub version: String,
    pub classifier: Option<String>,
}

impl MavenIdentifier {
    fn new(group: impl AsRef<str>, artifact: impl AsRef<str>, version: impl AsRef<str>, classifier: Option<impl AsRef<str>>) -> Self {
        Self {
            group: group.as_ref().to_string(),
            artifact: artifact.as_ref().to_string(),
            version: version.as_ref().to_string(),
            classifier: classifier.map(|it| it.as_ref().to_string()),
        }
    }
}

impl Display for MavenIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.group, self.artifact, self.version)?;
        if let Some(classifier) = &self.classifier {
            write!(f, ":{classifier}")?;
        }

        Ok(())
    }
}

impl FromStr for MavenIdentifier {
    type Err = <[String; 3] as TryFrom<Vec<String>>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(':').map(str::to_string).collect();

        let gav: Vec<_> = parts.clone().into_iter().take(3).collect();
        let [group, artifact, version]: [String; 3] = gav.try_into()?;

        let classifier = parts.into_iter().nth(3);

        Ok(Self { group, artifact, version, classifier })
    }
}

impl Serialize for MavenIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

struct MavenVisitor;

impl<'de> Visitor<'de> for MavenVisitor {
    type Value = MavenIdentifier;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a maven artifact identifier")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        match MavenIdentifier::from_str(v) {
            Ok(maven) => Ok(maven),
            Err(_) => Err(E::custom("failed to deserialize maven artifact identifier"))
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: Error {
        Self::visit_str(self, v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        Self::visit_str(self, &v)
    }
}

impl<'de> Deserialize<'de> for MavenIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(MavenVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::MavenIdentifier;

    #[test]
    fn serialization() {
        let repr = MavenIdentifier::new("dev.andre4ik3", "cool-artifact", "1.2.3", Option::<String>::None);
        let ser = "dev.andre4ik3:cool-artifact:1.2.3".to_string();

        assert_eq!(repr.to_string(), ser);
        assert_eq!(to_string(&repr).expect("serialization failed"), format!("\"{ser}\""));

        let repr = MavenIdentifier::new("dev.andre4ik3", "cool-artifact", "1.2.3", Some("classifier"));
        let ser = "dev.andre4ik3:cool-artifact:1.2.3:classifier".to_string();

        assert_eq!(repr.to_string(), ser);
        assert_eq!(to_string(&repr).expect("serialization failed"), format!("\"{ser}\""));
    }

    #[test]
    fn deserialization() {
        let repr = MavenIdentifier::new("dev.andre4ik3", "cool-artifact", "1.2.3", Option::<String>::None);
        let ser = "dev.andre4ik3:cool-artifact:1.2.3".to_string();

        assert_eq!(from_str::<MavenIdentifier>(&format!("\"{ser}\"")).expect("deserialization failed"), repr);

        let repr = MavenIdentifier::new("dev.andre4ik3", "cool-artifact", "1.2.3", Some("classifier"));
        let ser = "dev.andre4ik3:cool-artifact:1.2.3:classifier".to_string();

        assert_eq!(from_str::<MavenIdentifier>(&format!("\"{ser}\"")).expect("deserialization failed"), repr);
    }
}
