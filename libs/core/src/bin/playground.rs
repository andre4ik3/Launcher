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

use launcher::net::auth::{
    get_game_token, get_ms_token, get_profile, get_xbl_token, get_xsts_token,
};
use reqwest::redirect::Policy;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()
        .unwrap();

    // let code = "morbius";
    // println!("Code is {}", code);
    //
    // let (token, refresh) = get_ms_token(&client, code).await.unwrap();
    // println!("Token is {}", token);
    // println!("Refresh is {}", refresh);
    //
    // let xbl = get_xbl_token(&client, &token).await.unwrap();
    // println!("XBL is {}", xbl);
    //
    // let (xsts, uhs) = get_xsts_token(&client, &xbl).await.unwrap();
    // println!("XSTS is {}", xsts);
    // println!("UHS is {}", uhs);
    //
    // let game = get_game_token(&client, &uhs, &xsts).await.unwrap();
    // println!("Game token is {}", game);

    let game = "sus";
    get_profile(&client, game).await.unwrap();
}
