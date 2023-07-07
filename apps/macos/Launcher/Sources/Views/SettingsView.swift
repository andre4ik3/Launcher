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

import SwiftUI

func icon(_ selection: String, _ name: String, _ icon: String) -> String {
    return selection == name ? icon + ".fill" : icon
}

struct SettingsView: View {
    @EnvironmentObject var bridge: Bridge
    @State private var selection = "General"

    var body: some View {
        TabView(selection: $selection) {
            SettingsContainer(name: "General", icon: "gear") {
                Text("General").foregroundStyle(Color.accentColor)
            }
            .tag("General")

            SettingsContainer(name: "Accounts", icon: icon(selection, "Accounts", "person.2")) {
                Text("Accounts")
            }
            .tag("Accounts")

            SettingsContainer(name: "Java", icon: icon(selection, "Java", "cup.and.saucer")) {
                Text("Java")
            }
            .tag("Java")
        }
    }
}

struct SettingsView_Previews: PreviewProvider {
    static var previews: some View {
        SettingsView()
    }
}
