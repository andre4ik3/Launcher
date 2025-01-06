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

use url::Url;

use macros::data_structure;

use crate::core::conditional::Condition;

/// The current version that this module supports.
pub const VERSION: u64 = 0;

/// Severity level of the announcement. Depending on the severity level set, the announcement will
/// progressively get more intrusive and interfere with regular usage. Only use the higher levels in
/// extreme scenarios that affect all users -- e.g. stopping support of an OS version for users that
/// are still on that version.
#[data_structure]
pub enum MetaIndexAnnouncementSeverity {
    /// Critical announcements are non-dismissible and cause a modal popup on every app launch. They
    /// require confirmation to proceed every time when using the CLI.
    Critical,
    /// Severe announcements are dismissible after a warning and presented as red banners at the top
    /// of the app. They are shown every time when using the CLI.
    Severe,
    /// Warning announcements are dismissible and presented as yellow banners at the top of the app.
    /// They are shown once per day when running via the CLI.
    Warning,
    /// Informational announcements are dismissible and presented as blue banners at the bottom of
    /// the app. They are shown once when running via the CLI.
    Informational,
}

/// An announcement is a piece of information that can be shown to all or some users (depending on
/// a [Condition]) and can be used to supply info that reaches _all_ users. Useful to warn about
/// deprecations, vulnerabilities, widespread issues, and other things that users should know about.
#[data_structure]
pub struct MetaIndexAnnouncement {
    /// Severity of the announcement that determines how intrusive this announcement is.
    pub severity: MetaIndexAnnouncementSeverity,
    /// Condition that determines when this announcement will be shown to users.
    pub condition: Condition,
    /// Title of the announcement as shown in modal windows.
    pub title: String,
    /// Content of the announcement as shown in modal windows.
    pub content: String,
    /// If supplied, creates a "learn more" button shown in banners and modal windows.
    pub details: Option<Url>,
    /// If supplied, causes banners to display a scrolling marquee instead of the title.
    pub marquee: Option<String>,
}

/// The index of a metadata server. Used to determine if a given metadata server is online, and also
/// contains special announcements that are displayed as banners in the app.
#[data_structure]
pub struct MetaIndex {
    /// List of API versions that this meta server supports. Each version corresponds to a specific
    /// path prefix -- e.g. `1` would mean `/v1` exists, `2` for `/v2`, etc.
    pub api_versions: Vec<u64>,
    /// List of announcements. See [MetaIndexAnnouncement] for more info.
    pub announcements: Vec<MetaIndexAnnouncement>,
}
