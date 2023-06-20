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

struct Person: Identifiable {
    let givenName: String
    let familyName: String
    let emailAddress: String
    let id = UUID()

    var fullName: String { givenName + " " + familyName }
}

struct SettingsView: View {
    @State private var selection = "General"
    @State private var people = [
        Person(givenName: "Juan", familyName: "Chavez", emailAddress: "juanchavez@icloud.com"),
        Person(givenName: "Mei", familyName: "Chen", emailAddress: "meichen@icloud.com"),
        Person(givenName: "Tom", familyName: "Clark", emailAddress: "tomclark@icloud.com"),
        Person(givenName: "Gita", familyName: "Kumar", emailAddress: "gitakumar@icloud.com"),
    ]

    var body: some View {
        TabView(selection: $selection) {
            SettingsContainer(name: "General", icon: "gear") {
                Text("General")
            }.tag("General")

            SettingsContainer(
                name: "Accounts", icon: selection == "Accounts" ? "person.2.fill" : "person.2"
            ) {
                Table(people) {
                    TableColumn("Given Name", value: \.givenName)
                    TableColumn("Family Name", value: \.familyName)
                    TableColumn("E-Mail Address", value: \.emailAddress)
                }
            }.tag("Accounts")

            SettingsContainer(name: "Advanced", icon: "star") {
                Text("Advanced")
            }.tag("Advanced")
        }
    }
}

struct SettingsView_Previews: PreviewProvider {
    static var previews: some View {
        SettingsView()
    }
}
