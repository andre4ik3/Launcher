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

use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use indicatif::ProgressBar;
use tokio::fs::create_dir_all;
use tokio::time::sleep;
use xz2::write::XzEncoder;

use crate::models::game::{AssetIndex, AssetIndexFile};
use crate::utils::prog_style;
use crate::CLIENT;

const RESOURCES_BASE: &str = "https://resources.download.minecraft.net";

pub async fn run(assets: HashSet<AssetIndex>) -> Result<()> {
    println!("Generating compressed assets...");

    for index in assets {
        let id = index.id;

        let path: PathBuf = format!("_site/v1/assets/{id}.tar.xz").into();
        if path.exists() {
            continue;
        }

        // ensure parent directories exist
        create_dir_all(path.parent().unwrap()).await?;
        let file = File::create(path)?;

        // create .tar.xz encoder
        let encoder = XzEncoder::new(file, 9);
        let mut tar = tar::Builder::new(encoder);

        // download json index file
        let index = CLIENT.get(index.url).send().await?;
        let index: AssetIndexFile = index.error_for_status()?.json().await?;

        let pb = ProgressBar::new(index.objects.len() as u64);
        pb.set_style(prog_style());
        pb.set_message(format!("Assets {id}"));

        for (_name, file) in index.objects {
            pb.inc(1);

            let name = format!("{}/{}", &file.hash[0..2], file.hash);
            let url = format!("{RESOURCES_BASE}/{name}");

            // get the file
            let file = CLIENT.get(url).send().await?;
            let file = file.error_for_status()?.bytes().await?;

            // build the tar header
            let mut header = tar::Header::new_gnu();
            header.set_path(name)?;
            header.set_mode(0o644);
            header.set_size(file.len() as u64);
            header.set_cksum();

            // append to archive
            tar.append(&header, file.as_ref())?;
            sleep(Duration::from_millis(10)).await;
        }

        tar.finish()?;
        sleep(Duration::from_secs(10)).await;
    }

    Ok(())
}
