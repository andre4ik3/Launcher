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

struct InstancesView: View {
    @State private var code = ""
    @EnvironmentObject var bridge: Bridge

    var body: some View {
        VStack {
            TextField("Auth code", text: $code)
            Button("Do login") {
                Task {
                    await bridge.rust.do_login(code)
                }
            }
        }.padding()
    }
}

struct InstancesView_Previews: PreviewProvider {
    static var previews: some View {
        InstancesView().environmentObject(Bridge(LauncherBridge()))
    }
}
