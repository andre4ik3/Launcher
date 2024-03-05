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

use std::path::Path;

use async_trait::async_trait;
use tokio::fs;
use url::Url;
use data::core::conditional::Condition;
use data::web::meta::{MetaIndex, MetaIndexAnnouncement, MetaIndexAnnouncementSeverity, VERSION};
use crate::macros::write_to_ron_file;

use crate::task::Task;

pub struct IndexTask;

#[async_trait]
impl Task for IndexTask {
    type Input = ();
    type Output = ();

    async fn run(root: impl AsRef<Path> + Send + Sync, input: Self::Input) -> anyhow::Result<Self::Output> {
        let path = root.as_ref().join("index.ron");

        let data = MetaIndex {
            api_versions: vec![VERSION],
            announcements: vec![
                MetaIndexAnnouncement {
                    severity: MetaIndexAnnouncementSeverity::Informational,
                    condition: Condition::Always,
                    title: "Hello, world!".to_string(),
                    content: "Welcome to B(a)G(e)L!".to_string(),
                    details: Some(Url::parse("https://example.com").unwrap()),
                    marquee: Some("very long text that makes the screen scroll sideways slowly hopefully, a bit more rambling so that the scroll is nice and long, oh yeah it's gonna look so good, like those things with stocks at the new york stock exchange or something".to_string()),
                }
            ],
        };

        write_to_ron_file(&path, &data).await?;
        Ok(())
    }
}
