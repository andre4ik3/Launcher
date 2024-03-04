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

import SwiftUI

struct Instance: View {
    let label: any StringProtocol
    let selected: Bool

    var body: some View {
        VStack {
            Image(systemName: "plus").frame(width: 64.0, height: 64.0)
            Text(label)
        }
        .contextMenu {
            Button {
            } label: {
                Label("Delete", systemImage: "trash")
            }
        }
    }
}

struct Instance_Previews: PreviewProvider {
    static var previews: some View {
        HStack {
            Instance(label: "Instance Name", selected: true)
            Instance(label: "Instance Name", selected: false)
            Instance(label: "Instance Name", selected: false)
        }
        .padding()
    }
}
