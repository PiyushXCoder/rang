/*
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use gio::prelude::*;
use gtk::{Application, prelude::*};
use glib::clone;
use std::env::args;

fn main() {
    let app = gtk::Application::new(Some("sng.tarangm"), Default::default())
    .expect("Failed to initiate gtk");

    app.connect_activate(|app| {
        display(app);
    });

    app.run(&args().collect::<Vec<_>>());
}

fn display(app: &Application) {
    let builder = gtk::Builder::from_file("ui.glade");

    let win = builder.get_object::<gtk::Window>("win").expect("Resource file missing!");
    win.set_application(Some(app));
    win.show_all();

    let abt = builder.get_object::<gtk::AboutDialog>("about_win").expect("Resource file missing!");
    abt.set_version(Some(env!("CARGO_PKG_VERSION")));

    builder.connect_signals(|_, handler_name| {
        match handler_name {
            "show_about" => Box::new(clone!(@weak abt => @default-return None, move |_| {
                abt.show_all();
                None
            })),
            "about_close2" | "about_close1" => Box::new(clone!(@weak abt => @default-return None, move |_| {
                abt.hide();
                Some(true.to_value())
            })),
            "gtk_main_quit" => Box::new(clone!(@weak win => @default-return None, move |_| {
                unsafe { win.destroy() }
                None
            })),
            _ => Box::new(|_| {None})
        }
    });
}
