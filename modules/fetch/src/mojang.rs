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

use data::web::mojang::{PROFILE_URL, UserProfile};
use net::{Client, Error, Method, Request};
use net::header::HeaderValue;

use super::Result;

/// Gets a user's own profile from a game token.
pub async fn get_profile(client: &Client, token: &str) -> Result<UserProfile> {
    let mut request = Request::new(Method::GET, PROFILE_URL.try_into().unwrap());

    let value = HeaderValue::from_str(format!("Bearer {token}").as_str()).unwrap();
    request.headers_mut().insert("Authorization", value);

    let data: UserProfile = client.execute(request).await?.json().await.map_err(Error::from)?;
    Ok(data)
}
