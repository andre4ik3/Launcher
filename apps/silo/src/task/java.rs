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

use platforms::{Arch, OS};
use semver::Version;
use url::Url;

use launcher::data::core::java::{Environment, JavaBuild, JavaEdition, JavaProvider};
use launcher::data::silo::java::zulu::{ZuluDetails, ZuluMetadata};

use crate::macros::write_to_ron_file;
use crate::{client, vpath};

const BASE_URL: &str = "https://api.azul.com/metadata/v1/zulu/packages/?javafx_bundled=false&crac_supported=false&latest=true&release_status=ga&availability_types=CA&certifications=tck";

const ENVIRONMENTS: &[Environment] = &[
    Environment {
        os: OS::Windows,
        arch: Arch::AArch64,
    },
    Environment {
        os: OS::Windows,
        arch: Arch::X86_64,
    },
    Environment {
        os: OS::MacOS,
        arch: Arch::AArch64,
    },
    Environment {
        os: OS::MacOS,
        arch: Arch::X86_64,
    },
    Environment {
        os: OS::Linux,
        arch: Arch::AArch64,
    },
    Environment {
        os: OS::Linux,
        arch: Arch::X86_64,
    },
];

fn convert_build(meta: ZuluMetadata, details: ZuluDetails, environment: &Environment) -> JavaBuild {
    JavaBuild {
        provider: JavaProvider::Zulu,
        version: Version::new(
            meta.java_version.0,
            meta.java_version.1,
            meta.java_version.2,
        ),
        edition: if details.name.contains("jre") {
            JavaEdition::JRE // this is preferred -- it's about 3x smaller than JDK
        } else {
            JavaEdition::JDK // full Java "development kit" with headers etc.
        },
        environment: environment.clone(),
        executable: match environment.os {
            OS::Linux | OS::MacOS => "bin/java",
            OS::Windows => "bin\\javaw.exe", // `javaw.exe` spawns a windowed process on Windows (normal `java.exe` runs headless!!)
            _ => unreachable!(),
        }
        .to_string(),
        download: meta.download_url,
        name: details.name,
        size: details.size,
        checksum: details.sha256_hash,
    }
}

pub async fn run(major_versions: Vec<u64>) -> anyhow::Result<Vec<JavaBuild>> {
    let client = client();
    let mut output = Vec::new();

    for version in major_versions {
        for environment in ENVIRONMENTS {
            // First, prepare the parameters just in the way the Azul API expects them to be.
            let os = match environment.os {
                OS::MacOS => "macos",
                OS::Windows => "windows",
                OS::Linux => "linux-glibc", // TODO: musl support in the future?
                _ => unreachable!(),
            };

            let arch = match environment.arch {
                Arch::AArch64 => "aarch64",
                Arch::X86_64 => "x64",
                _ => unreachable!(),
            };

            let archive_type = match environment.os {
                OS::MacOS | OS::Linux => "tar.gz",
                OS::Windows => "zip",
                _ => unreachable!(),
            };

            // Next, build them all into the URL that we will query.
            let url = Url::parse_with_params(BASE_URL, &[
                ("java_version", version.to_string().as_str()),
                ("os", os),
                ("arch", arch),
                ("archive_type", archive_type),
            ])?;

            // Search the Azul API for our build.
            let mut builds: Vec<ZuluMetadata> = client.get(url).await?.json().await?;

            // In all cases I've observed, the first build is the one we want.
            // If we have a build, fetch more info about it using the package details endpoint.
            if builds.is_empty() {
                tracing::warn!(
                    "Could not find build for Java {version} for {} {}!",
                    environment.os,
                    environment.arch
                );
                continue;
            }

            let meta = builds.swap_remove(0);
            let url = format!(
                "https://api.azul.com/metadata/v1/zulu/packages/{}",
                meta.package_uuid
            );
            let details: ZuluDetails = client.get(url).await?.json().await?;

            // Finally, assemble the fully parsed Java build for consumption in clients...
            let build = convert_build(meta, details, environment);

            // ...and write it to disk.
            tracing::info!(
                "Fetched Java {version} for {} {}.",
                environment.os,
                environment.arch
            );
            write_to_ron_file(
                vpath!("java/{version}/{}-{}.ron", environment.os, environment.arch),
                &build,
            )
            .await?;
            output.push(build);
        }
    }

    Ok(output)
}
