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

use data::web::meta::{MetaIndex, MetaIndexAnnouncement, VERSION};

use crate::macros::write_to_ron_file;
use crate::path;

pub async fn run(announcements: Vec<MetaIndexAnnouncement>) -> anyhow::Result<MetaIndex> {
    let data = MetaIndex {
        api_versions: vec![VERSION],
        announcements,
    };

    write_to_ron_file(path!("index.ron"), &data).await?;
    Ok(data)
}
