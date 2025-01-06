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

import Sparkle
import SwiftUI

final class CheckForUpdatesViewModel: ObservableObject {
    @Published var canCheckForUpdates = false

    init(updater: SPUUpdater) {
        updater.publisher(for: \.canCheckForUpdates)
            .assign(to: &$canCheckForUpdates)
    }
}

@main
struct LauncherApp: App {
    @ObservedObject private var checkForUpdatesViewModel: CheckForUpdatesViewModel
    @State private var url = URL(string: "https://echo.paw.cloud")!
    private let updaterController: SPUStandardUpdaterController

    init() {
        updaterController = SPUStandardUpdaterController(
            startingUpdater: true, updaterDelegate: nil, userDriverDelegate: nil)
        self.checkForUpdatesViewModel = CheckForUpdatesViewModel(updater: updaterController.updater)
    }

    var body: some Scene {
        Window("Launcher", id: "main") {
            ContentView().environmentObject(Bridge(LauncherBridge()))
        }
        .windowToolbarStyle(.unifiedCompact)
        .commands {
            CommandGroup(after: .appInfo) {
                Button("Check for Updates…", action: updaterController.updater.checkForUpdates)
                    .disabled(!checkForUpdatesViewModel.canCheckForUpdates)
            }
        }

        Settings {
            SettingsView().environmentObject(Bridge(LauncherBridge()))
        }
    }
}

class Bridge: ObservableObject {
    var rust: LauncherBridge

    init(_ rust: LauncherBridge) {
        self.rust = rust
    }
}
