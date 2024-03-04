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

use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use once_cell::sync::Lazy;

/// Shared instance of [ProjectDirs] to avoid initializing it multiple times.
static DIRS: Lazy<ProjectDirs> = Lazy::new(|| {
    ProjectDirs::from("dev", "andre4ik3", "Launcher")
        .expect("user's home directory does not exist or is inaccessible")
});

/// Takes a function from [ProjectDirs] that returns a path (e.g. [ProjectDirs::config_dir]). Runs
/// the function, ensures the path exists, resolves the path, and returns it.
fn dir(func: impl FnOnce(&ProjectDirs) -> &Path) -> PathBuf {
    let path = func(&DIRS).to_path_buf();
    fs::create_dir_all(&path).expect("failed to create directory");
    fs::canonicalize(path).expect("failed to resolve directory")
}

/// Cache directory. Things like in-progress downloads and staged extractions should be stored here.
pub static CACHE: Lazy<PathBuf> = Lazy::new(|| dir(ProjectDirs::cache_dir));

/// Config directory. Things like the app configuration and credentials should be stored here.
pub static CONFIG: Lazy<PathBuf> = Lazy::new(|| dir(ProjectDirs::config_dir));

/// Data directory. Things like Java installations and downloaded assets should be stored here.
pub static DATA: Lazy<PathBuf> = Lazy::new(|| dir(ProjectDirs::data_local_dir));
