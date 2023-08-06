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
use persistence::FileRegistry;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug, Deserialize, Serialize)]
struct Testing {
    variable: String,
}

impl Default for Testing {
    fn default() -> Self {
        Self {
            variable: "Hello, world!".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = utils::setup_logging();
    info!("Hello, world!");

    let registry: FileRegistry<Testing> = FileRegistry::new_encrypted("Testing.dat").await?;

    let lock = RwLock::new(Testing {
        variable: "nothing".to_string(),
    });

    let guard = lock.read().await;
    info!("value is {}", guard.variable);
    drop(guard);

    *lock.write().await = Testing::default();

    let guard = lock.read().await;
    info!("value is {}", guard.variable);
    drop(guard);

    registry.save().await?;

    Ok(())
}
