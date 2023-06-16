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

use serde::{Deserialize, Serialize};
use url::Url;

/// The types of severities for announcements.
#[derive(Debug, Deserialize, Serialize)]
pub enum AnnouncementSeverity {
    /// The announcement is shown within an announcements page.
    None,
    /// The announcement is shown within a blue bar at the top of the launcher.
    Informational,
    /// The announcement is shown within a yellow bar at the top of the launcher.
    Warning,
    /// The announcement is shown within a red bar at the top of the launcher.
    Severe,
    /// The announcement is shown within a pulsating red bar at the top of the launcher.
    /// Additionally, on startup, a popup is presented with the announcement content.
    Critical,
}

/// An announcement of varying severity to be shown to the user.
#[derive(Debug, Deserialize, Serialize)]
pub struct Announcement {
    /// A medium length string shown in a banner at the top of the launcher (when applicable).
    /// It is not shown with the title or description - it is completely standalone.
    pub marquee: String,
    /// A short title for the announcement. Shown on the announcement page above the content.
    pub title: String,
    /// A long string that details what the announcement is about. Shown on the announcement page
    /// when the web embed is not set or available.
    pub content: String,
    /// A URL to a page that will be embedded on the announcement page. Overrides the content.
    /// Note that the content will still be shown if the loading fails.
    pub embed: Option<Url>,
    /// A link that is opened when clicking the marquee or a button in the announcement page.
    pub link: Url,
    /// The severity of the announcement.
    pub severity: AnnouncementSeverity,
}

/// Information and important announcements for users.
#[derive(Debug, Deserialize, Serialize)]
pub struct MetadataIndex {
    /// Currently active announcements for users.
    pub announcements: Box<[Announcement]>,
}
