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
import Combine

struct ContentView: View {
    @EnvironmentObject var rustApp: RustAppWrapper

    @State private var name = "world"
    @State private var output = ""
    @State private var counter = "Haven't got the counter yet."

    let columns = [GridItem(.flexible()), GridItem(.flexible())]

    var body: some View {
        ScrollView(.vertical) {
//            LazyVGrid(columns: [GridItem(.flexible())], alignment: .leading) {
//                ForEach(0..<100) {
//                    Text("Group \($0)")
//                        .font(.title)
//                        .accessibilityAddTraits(.isHeader)
//                    ScrollView(.horizontal) {
//                        LazyHGrid(rows: [GridItem(.flexible())], alignment: .top) {
//                            ForEach(0..<100) {
//                                Text("Instance \($0)")
//                            }
//                        }
//                    }
//                }
//            }.padding()
            VStack {
                Button("Do a java download") {
                    print("This is swift")
                    Task {
                        await rustApp.rust.do_something()
                    }
                    print("We back in swift")
                }
            }.padding()
        }
        .toolbarRole(.editor)
        .toolbar(id: "instances") {
            ToolbarItem(id: "delete", placement: .primaryAction) {
                Button { print("Goodbye, world!") }
                    label: { Label("Delete", systemImage: "trash") }
            }
            ToolbarItem(id: "edit", placement: .primaryAction) {
                Button { print("Editing, world!") }
                    label: { Label("Edit", systemImage: "pencil") }
            }
            ToolbarItem(id: "new", placement: .primaryAction) {
                Button { print("Hello, world!") }
                    label: { Label("New", systemImage: "plus") }
            }
        }
        .navigationTitle("Instances")
    }

    private func emoji(_ value: Int) -> String {
            guard let scalar = UnicodeScalar(value) else { return "?" }
            return String(Character(scalar))
        }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
            .environmentObject(RustAppWrapper(rust: RustApp()))
    }
}
