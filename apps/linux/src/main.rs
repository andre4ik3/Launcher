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

use gtk::prelude::*;
use gtk::{gio, glib, Box, ListBox, Orientation, SelectionMode};
use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use adw::prelude::ActionRowExt;

const APP_ID: &str = "dev.andre4ik3.Launcher";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("launcher.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let row = ActionRow::builder()
        .activatable(true)
        .title("Click me")
        .build();

    row.connect_activated(|_| {
        eprintln!("Clicked!");
    });

    let list = ListBox::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        .selection_mode(SelectionMode::None)
        // makes the list look nicer
        .css_classes(vec![String::from("boxed-list")])
        .build();

    list.append(&row);

    // Combine the content in a box
    let content = Box::new(Orientation::Vertical, 0);

    // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(&HeaderBar::new());
    content.append(&list);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Launcher")
        .default_width(350)
        // add content to window
        .content(&content)
        .build();

    window.present();
}
