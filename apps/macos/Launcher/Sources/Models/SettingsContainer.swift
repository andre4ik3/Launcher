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

import SwiftUI

struct SettingsContainer<Content>: View where Content: View {
    let name: any StringProtocol
    let icon: String
    let content: () -> Content

    var body: some View {
        content()
            .padding()
            .tabItem {
                Label(name, systemImage: icon)
            }
            .frame(width: 500.0, height: 300.0)
    }
}

struct SettingsContainer_Previews: PreviewProvider {
    static var previews: some View {
        SettingsContainer(name: "General", icon: "gear") {
            Text("Test")
        }
    }
}
