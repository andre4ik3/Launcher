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

//! Launcher - Secure Store
//! This module handles storage of secure information for the launcher - primary credentials.
//! Rundown of how it works:
//! 1. Generate an encryption key once on startup.
//! 2. Store it in the system's keychain (e.g. macOS Keychain). Read it from there in the future.
//! 3. Use said key to encrypt and decrypt another config file with all the credentials.
//! 4. If the credential is ever lost for some reason, do everything again.
