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

use anyhow::Result;
use serde::{Deserialize, Serialize};

use persistence::DirectoryRegistry;

#[derive(Debug, Deserialize, Serialize)]
struct Testing {
    variable: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    util::setup_logging();
    tracing::info!("Hello, world!");

    let mut registry: DirectoryRegistry<Testing> =
        persistence::DirectoryRegistry::create("Testing", "Test.toml").await?;

    registry
        .insert(
            "test-id",
            Testing {
                variable: "This is a test!".to_string(),
            },
        )
        .await?;

    Ok(())
}
