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

use swift_bridge_build::parse_bridges;

const OUT_DIR: &str = "Launcher/Generated";
const BRIDGE_FILE: &str = "src/lib.rs";
const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    println!("cargo:rerun-if-changed={}", BRIDGE_FILE);
    println!("cargo:rerun-if-env-changed=CONFIGURATION");

    let generated = parse_bridges([BRIDGE_FILE]);
    generated.write_all_concatenated(OUT_DIR, CRATE_NAME);
}
