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

struct Java: Identifiable, Hashable {
    let id: String
    let provider: String
    let version: String
    let major: UInt64
}

struct JavaInstallationsView: View {
    @EnvironmentObject var bridge: Bridge
    @State private var isTaskRunning = false

    @State private var installations: [Java] = []
    @State private var selection = Set<Java.ID>()

    func refresh() async {
        let len = await bridge.rust.java_len()
        var results: [Java] = []

        if len > 0 {
            for index in 0...(len - 1) {
                let build = await bridge.rust.java_at(index)
                results.append(toJava(build))
            }
        }

        installations = results
    }

    var body: some View {
        VStack {
            Table(installations, selection: $selection) {
                TableColumn("Provider", value: \.provider).width(75.0)
                TableColumn("Version", value: \.version).width(60.0)
                TableColumn("Needed For") { build in
                    Text(description(build.major))
                }.width(min: 150.0)
            }

            HStack {
                if isTaskRunning {
                    // TODO: This is way too big, whenever task runs, the whole height changes
                    ProgressView()
                } else {
                    Text("Java versions are installed automatically as needed.").font(.footnote)
                }

                Spacer()

                // TODO: remove this once there's instances and stuff
                Button("Install") {
                    isTaskRunning = true
                    Task {
                        await bridge.rust.java_install(8)
                        await bridge.rust.java_install(16)
                        await bridge.rust.java_install(17)
                        await refresh()
                        isTaskRunning = false
                    }
                }.disabled(isTaskRunning)

                Button("Update") {
                    isTaskRunning = true
                    Task {
                        for id in selection {
                            await bridge.rust.java_update(id)
                        }
                        await refresh()
                        isTaskRunning = false
                    }
                }.disabled(selection.isEmpty || isTaskRunning)

                Button("Uninstall") {
                    isTaskRunning = true
                    Task {
                        for id in selection {
                            await bridge.rust.java_uninstall(id)
                        }
                        await refresh()
                        isTaskRunning = false
                    }
                }.disabled(selection.isEmpty || isTaskRunning)
            }.fixedSize(horizontal: false, vertical: true)
        }.task {
            await refresh()
        }
    }
}

struct JavaInstallationsView_Previews: PreviewProvider {
    static var previews: some View {
        JavaInstallationsView()
    }
}

// MARK: - Helper functions

func description(_ majorVersion: UInt64) -> String {
    switch majorVersion {
    case 8:
        return "Minecraft 1.16 and below"
    case 16:
        return "Minecraft 1.17"
    case 17:
        return "Minecraft 1.18 and above"
    default:
        return "-"
    }
}

func toJava(_ ffi: FFIJava) -> Java {
    return Java(
        id: ffi.id.toString(), provider: ffi.provider.toString(), version: ffi.version.toString(),
        major: ffi.major)
}
