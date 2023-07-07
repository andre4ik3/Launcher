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

use tokio::fs;

#[tokio::main]
async fn main() {
    let dir1 = tempfile::tempdir().unwrap();
    let dir2 = tempfile::tempdir().unwrap();
    fs::rename("/tmp/swap1", dir1.path()).await.unwrap();
    fs::rename("/tmp/swap2", dir2.path()).await.unwrap();
    fs::rename(dir1.path(), "/tmp/swap2").await.unwrap();
    fs::rename(dir2.path(), "/tmp/swap1").await.unwrap();
}
