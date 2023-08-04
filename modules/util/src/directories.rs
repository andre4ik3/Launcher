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

use directories::ProjectDirs;
use once_cell::sync::Lazy;
use std::path::PathBuf;

fn dirs() -> ProjectDirs {
    // There isn't much we can do if the user's home directory doesn't exist. Panicking is fine for
    // all intents and purposes.
    ProjectDirs::from("dev", "andre4ik3", "Launcher")
        .expect("user's home directory does not exist or is inaccessible")
}

/// Cache directory. Things like in-progress downloads and staged extractions should be stored here.
pub static CACHE: Lazy<PathBuf> = Lazy::new(|| dirs().cache_dir().to_path_buf());

/// Config directory. Things like the app configuration and credentials should be stored here.
pub static CONFIG: Lazy<PathBuf> = Lazy::new(|| dirs().config_dir().to_path_buf());

/// Data directory. Things like Java installations and downloaded assets should be stored here.
pub static DATA: Lazy<PathBuf> = Lazy::new(|| dirs().data_local_dir().to_path_buf());
